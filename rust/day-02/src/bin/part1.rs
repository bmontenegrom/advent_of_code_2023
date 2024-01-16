use std::{collections::HashMap, fs::File, io::Read, ops::Not, path::Path};

use nom::{
    bytes::complete::tag,
    character::complete::{self, alpha1, digit1, line_ending},
    multi::separated_list1,
    sequence::{preceded, separated_pair},
    IResult,
};

#[derive(Debug)]
struct Tirada<'a> {
    color: &'a str,
    cantidad: u32,
}
#[derive(Debug)]
struct Set<'a> {
    tiradas: Vec<Tirada<'a>>,
}
#[derive(Debug)]
struct Game<'a> {
    id: u32,
    sets: Vec<Set<'a>>,
}

impl<'a> Game<'a> {
    fn id_game_valido(&self, map: &HashMap<&'a str, u32>) -> Option<u32> {
        self.sets
            .iter()
            .any(|set| {
                set.tiradas.iter().any(|tirada| {
                    tirada.cantidad > *map.get(tirada.color).expect("no existe el color")
                })
            })
            .not()
            .then_some(self.id)
    }
}

// 3 blue
fn tirada(input: &str) -> IResult<&str, Tirada> {
    let (input, (cantidad, color)) = separated_pair(complete::u32, tag(" "), alpha1)(input)?;
    Ok((input, Tirada { color, cantidad }))
}
// 3 blue, 4 red
fn set(input: &str) -> IResult<&str, Set> {
    let (input, tiradas) = separated_list1(tag(", "), tirada)(input)?;
    Ok((input, Set { tiradas }))
}
//Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
fn game(input: &str) -> IResult<&str, Game> {
    let (input, id) = preceded(tag("Game "), digit1)(input)?;
    let (input, sets) = preceded(tag(": "), separated_list1(tag("; "), set))(input)?;
    Ok((
        input,
        Game {
            id: id.parse().unwrap(),
            sets,
        },
    ))
}

fn parse_games(input: &str) -> IResult<&str, Vec<Game>> {
    let (input, games) = separated_list1(line_ending, game)(input)?;
    Ok((input, games))
}

fn process_input(input: &str) -> Result<u32, String> {
    let map = HashMap::from([("blue", 14), ("red", 12), ("green", 13)]);
    let (_, games) = parse_games(input).expect("no se pudo parsear");
    let res = games.iter().filter_map(|game| game.id_game_valido(&map)).sum();
    Ok(res)

}

fn main() {
    let path = Path::new("src/bin/input_1.txt");
    let mut file = match File::open(path) {
        Ok(file) => file,
        Err(_) => panic!("no se encuentra el archivo"),
    };
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(_) => panic!("couldn't read file"),
        Ok(_) => match process_input(&s) {
            Ok(output) => println!("{}", output),
            Err(error) => println!("{}", error),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_input() -> Result<(), String> {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        //println!("{:?}", parse_games(input));
        assert_eq!(process_input(input)?, 8);
        Ok(())
    }
}
