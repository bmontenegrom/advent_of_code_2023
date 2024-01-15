use core::panic;
use std::{fs::File, io::Read, path::Path};

fn main() {
    let path = Path::new("input_1.txt");
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
    let output = intput
        .lines()
        .map(|line| {
            let mut it = line.chars().filter_map(|character| character.to_digit(10));
            let primero = it.next().expect("deveria ser un numero");

            let ultimo = it.last();

            match ultimo {
                Some(unidad) => primero * 10 + unidad,
                None => primero * 10 + primero,
            }
        })
        .sum();
    
    output
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_input() {
       let test_str = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
        assert_eq!(process_input(test_str), 142);
       
    }
}
