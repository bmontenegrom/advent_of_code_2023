use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, alphanumeric1, line_ending, multispace1},
    combinator::eof,
    multi::{fold_many1, many1},
    sequence::{delimited, separated_pair, terminated},
    IResult, Parser,
};
use num_integer::Integer;
use std::{collections::HashMap, str::FromStr};

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Direction {
    Left,
    Right,
}

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
struct Node {
    first: char,
    second: char,
    third: char,
}

impl Node {
    fn is_end_node(&self) -> bool {
        self.third == 'Z'
    }
    fn is_start_node(&self) -> bool {
        self.third == 'A'
    }
}

impl FromStr for Node {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars();
        let first = chars.next().ok_or("no existe primer caracter")?;
        let second = chars.next().ok_or("no existe segundo caracter")?;
        let third = chars.next().ok_or("no existe tercer caracter")?;
        Ok(Node {
            first,
            second,
            third,
        })
    }
}

#[derive(Debug)]
struct Instructions {
    instructions: Vec<Direction>,
}

impl Instructions {
    fn cycle(&self) -> impl Iterator<Item = Direction> + '_ {
        self.instructions.iter().copied().cycle()
    }
}
#[derive(Debug)]
struct PuzzleMap {
    instructions: Instructions,
    nodes: HashMap<Node, (Node, Node)>,
}

impl PuzzleMap {
    fn navigate(&self, inicio: &Node) -> Result<u64, String> {
        let mut current_node = *inicio;
        let result = self
            .instructions
            .cycle()
            .take_while(|direction| {
                let (left, right) = self.nodes.get(&current_node).unwrap();
                match direction {
                    Direction::Left => {
                        current_node = *left;
                    }
                    Direction::Right => {
                        current_node = *right;
                    }
                }
                !current_node.is_end_node()
            })
            .count();
        Ok(result as u64 + 1)
    }

    fn ghost_navigate(&self) -> Result<u64, String> {
        self.nodes
            .keys()
            .filter(|node| node.is_start_node())
            .map(|node| self.navigate(node))
            .try_fold(1, |acc, cant| Ok(acc.lcm(&cant?)))
    }
}

fn node_parser(input: &str) -> IResult<&str, Node> {
    let (input, str) = alphanumeric1(input)?;
    Ok((input, str.parse().unwrap()))
}

fn instructions_parser(input: &str) -> IResult<&str, Instructions> {
    let (input, instructions) = many1(alt((
        complete::char('R').map(|_| Direction::Right),
        complete::char('L').map(|_| Direction::Left),
    )))(input)?;
    Ok((input, Instructions { instructions }))
}

fn parse_input(input: &str) -> IResult<&str, PuzzleMap> {
    let (input, instructions) = instructions_parser(input)?;
    let (input, _) = multispace1(input)?;
    let (input, nodes) = fold_many1(
        terminated(
            separated_pair(
                node_parser,
                tag(" = "),
                delimited(
                    complete::char('('),
                    separated_pair(node_parser, tag(", "), node_parser),
                    complete::char(')'),
                ),
            ),
            alt((line_ending, eof)),
        ),
        HashMap::new,
        |mut acc: HashMap<Node, (Node, Node)>, (key, value)| {
            acc.insert(key, value);
            acc
        },
    )(input)?;
    Ok((
        input,
        PuzzleMap {
            instructions,
            nodes,
        },
    ))
}

fn main() {
    let input = include_str!("input.txt");
    let (_, map) = parse_input(input).expect("no se pudo parsear el input");
    println!("{:?}", map.ghost_navigate());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_camino1() {
        let input = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";
        let (_, map) = parse_input(input).expect("no se pudo parsear el input");
        assert_eq!(map.ghost_navigate().unwrap(), 6);
    }
}
