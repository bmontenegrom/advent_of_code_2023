use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, digit1, hex_digit1, line_ending, space1},
    multi::separated_list1,
    sequence::delimited,
    IResult, Parser,
};

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}
#[derive(Debug)]
struct Instruction<'a> {
    direction: Direction,
    distance: isize,
    color: &'a str,
}

#[derive(Debug, Clone, Copy)]
struct Point {
    x: isize,
    y: isize,
}

impl Point {
    fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }
    fn manhattan_distance(&self, other: &Point) -> isize {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

fn parse_instruction(input: &str) -> IResult<&str, Instruction> {
    let (input, direction) = alt((
        complete::char('U').map(|_| Direction::Up),
        complete::char('D').map(|_| Direction::Down),
        complete::char('L').map(|_| Direction::Left),
        complete::char('R').map(|_| Direction::Right),
    ))(input)?;
    let (input, distance) = delimited(space1, digit1, space1)(input)?;
    let (input, color) = delimited(tag("(#"), hex_digit1, complete::char(')'))(input)?;
    Ok((
        input,
        Instruction {
            direction,
            distance: distance.parse().unwrap(),
            color,
        },
    ))
}

fn parse_input(input: &str) -> IResult<&str, Vec<Instruction>> {
    separated_list1(line_ending, parse_instruction)(input)
}

//internal area
fn sholace_formula(points: &[Point]) -> isize {
    points
        .iter()
        .zip(points.iter().cycle().skip(1))
        .map(|(p1, p2)| p1.x * p2.y - p2.x * p1.y)
        .sum::<isize>()
        .abs()
        / 2
}

//A = i +b/2 -1 ==> i = A - b/2 +1 ,
//this gives te internal points, we need to add the perimeter points
//then wich is the sum of the manhatan distance of the points, the perimeter (b)
// total_points = A -b/2 +1 + b = A + b/2 +1
fn area(points: &[Point]) -> isize {
    let internal_area = sholace_formula(points);
    let perimeter = points
        .iter()
        .zip(points.iter().cycle().skip(1))
        .map(|(p1, p2)| p1.manhattan_distance(p2))
        .sum::<isize>();
    internal_area + perimeter / 2 + 1
}

fn build_points_part1(instructions: &[Instruction]) -> Vec<Point> {
    let mut points = Vec::new();
    let mut current_point = Point::new(0, 0);
    points.push(current_point);
    instructions.iter().for_each(|instruction| {
        let mut new_point = current_point;
        match instruction.direction {
            Direction::Up => new_point.y += instruction.distance,
            Direction::Down => new_point.y -= instruction.distance,
            Direction::Left => new_point.x -= instruction.distance,
            Direction::Right => new_point.x += instruction.distance,
        }
        points.push(new_point);
        current_point = new_point;
    });
    points
}

fn solve_part1(input: &str) -> isize {
    let (_, instructions) = parse_input(input).unwrap();
    let points = build_points_part1(&instructions);
    area(&points)
}

fn color_to_instruction(color: &str) -> Instruction {
    let (color, direction) = color.split_at(color.len() - 1);
    let direction = match direction {
        "3" => Direction::Up,
        "1" => Direction::Down,
        "2" => Direction::Left,
        "0" => Direction::Right,
        _ => panic!("Invalid direction"),
    };
    let distance = isize::from_str_radix(color, 16).unwrap();
    Instruction {
        direction,
        distance,
        color,
    }
}

fn solve_part2(input: &str) -> isize {
    let (_, instructions) = parse_input(input).unwrap();
    let instructions = instructions
        .iter()
        .map(|instruction| color_to_instruction(instruction.color))
        .collect::<Vec<Instruction>>();
    let points = build_points_part1(&instructions);
    area(&points)
}

fn main() {
    let input = include_str!("input.txt");
    let now = std::time::Instant::now();
    println!("Part 1: {} in {:?}", solve_part1(input), now.elapsed());
    let now = std::time::Instant::now();
    println!("Part 2: {} in {:?}", solve_part2(input), now.elapsed());
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_part1() {
        let input = include_str!("input_test.txt");
        assert_eq!(solve_part1(input), 62);
    }

    #[test]
    fn test_part2() {
        let input = include_str!("input_test.txt");
        assert_eq!(solve_part2(input), 952408144115);
    }
}
