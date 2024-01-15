use core::panic;
use std::{fs::File, io::Read, path::Path};

fn main() {
    let path = Path::new("input_2.txt");
    let mut file = match File::open(&path) {
        Ok(file) => file,
        Err(_) => panic!("couldn't open file"),
    };
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(_) => panic!("couldn't read file"),
        Ok(_) => println!("{}", process_input(&s)),
    };
}

fn process_input(intput: &str) -> u32 {
    intput.lines().map(process_line).sum()
}

fn process_line(line: &str) -> u32 {
    let mut it = (0..line.len()).filter_map(|index| {
        let sub_string = &line[index..];
        let result = if sub_string.starts_with("one") {
            '1'
        } else if sub_string.starts_with("two") {
            '2'
        } else if sub_string.starts_with("three") {
            '3'
        } else if sub_string.starts_with("four") {
            '4'
        } else if sub_string.starts_with("five") {
            '5'
        } else if sub_string.starts_with("six") {
            '6'
        } else if sub_string.starts_with("seven") {
            '7'
        } else if sub_string.starts_with("eight") {
            '8'
        } else if sub_string.starts_with("nine") {
            '9'
        } else {
            sub_string.chars().next().unwrap()
        };
        result.to_digit(10)
    });

    let primero = it.next().expect("deveria ser un numero");

    let ultimo = it.last();

    match ultimo {
        Some(unidad) => primero * 10 + unidad,
        None => primero * 10 + primero,
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_input() {
       let test_input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
        assert_eq!(process_input(test_input), 281);
       
    }
}