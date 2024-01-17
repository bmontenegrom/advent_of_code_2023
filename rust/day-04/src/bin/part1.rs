use std::{collections::HashSet, fs::File, io::Read, path::Path};

use nom::{
    bytes::complete::tag,
    character::complete::{self, digit1, line_ending, space0, space1},
    multi::{fold_many1, separated_list1},
    sequence::{delimited, separated_pair, terminated, tuple},
    IResult, Parser,
};

#[derive(Debug)]
struct Card {
    numeros_ganadores: HashSet<u32>,
    numeros: HashSet<u32>,
}

impl Card {
    fn puntaje(&self) -> u32 {
        let cantidad = self.numeros_ganadores.intersection(&self.numeros).count() as u32;
        if cantidad == 0 {
            0
        } else {
            2u32.pow(cantidad - 1)
        }
    }
}

fn set(input: &str) -> IResult<&str, HashSet<u32>> {
    fold_many1(
        terminated(complete::u32, space0),
        HashSet::new,
        |mut acc: HashSet<_>, item| {
            acc.insert(item);
            acc
        },
    )(input)
}

//Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
fn card(input: &str) -> IResult<&str, Card> {
    let (input, _) = delimited(
        tuple((tag("Card"), space1)),
        digit1,
        tuple((tag(":"), space1)),
    )(input)?;
    separated_pair(set, tuple((tag("|"), space1)), set)
        .map(|(numeros_ganadores, numeros)| Card {
            numeros_ganadores,
            numeros,
        })
        .parse(input)
}

fn cards(input: &str) -> IResult<&str, Vec<Card>> {
    separated_list1(line_ending, card)(input)
}

fn process_input(input: &str) -> Result<u32, String> {
    let (_, cards) = cards(input).expect("no se pudo parsear el input");
    let resultado = cards.iter().map(|card| card.puntaje()).sum();
    Ok(resultado)
}

fn main() {
    let path = Path::new("src/bin/input1.txt");
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
mod test {
    use super::*;

    #[test]
    fn test_proc() -> Result<(), String> {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        assert_eq!(process_input(input)?, 13);
        Ok(())
    }
}
