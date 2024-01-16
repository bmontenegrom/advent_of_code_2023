use regex::Regex;
use std::{collections::HashSet, fs::File, io::Read, path::Path};

#[derive(Debug, PartialEq, Eq, Hash)]
struct Position {
    x: u32,
    y: u32,
}

#[derive(Debug)]
struct Numero {
    valor: u32,
    largo: u32,
    posicion: Position,
}

#[derive(Debug)]
struct Simbolo {
    _valor: char,
    posicion: Position,
}

impl Simbolo {
    fn vecinos<'a>(&'a self, numeros: &'a [Numero]) -> Vec<&Numero> {
        let vecinos = numeros
            .iter()
            .filter(|numero| {
                let x = self.posicion.x;
                let y = self.posicion.y;
                let x1 = numero.posicion.x;
                let y1 = numero.posicion.y;
                let largo = numero.largo;
                let mut posiciones_numero = HashSet::new();
                for i in 0..largo {
                    posiciones_numero.insert(Position { x: x1 + i, y: y1 });
                }
                let vecinos_simbolo = HashSet::from([
                    Position { x: x - 1, y: y - 1 },
                    Position {  x, y: y - 1 },
                    Position { x: x + 1, y: y - 1 },
                    Position { x: x - 1, y },
                    Position { x: x + 1, y },
                    Position { x: x - 1, y: y + 1 },
                    Position { x, y: y + 1 },
                    Position { x: x + 1, y: y + 1 },
                ]);
                posiciones_numero.intersection(&vecinos_simbolo).count() > 0
            })
            .collect::<Vec<&Numero>>();
        vecinos
    }
}

fn numeros(input: & str) -> Vec<Numero> {
    let mut numeros = Vec::new();
    let num_regex = Regex::new(r"(\d+)").unwrap();
    for (y, line) in input.lines().enumerate() {
        for numero in num_regex.find_iter(line) {
            let posicion = Position {
                x: numero.start() as u32,
                y: y as u32,
            };
            let largo = numero.as_str().len() as u32;
            let valor = numero.as_str().parse().unwrap();
            numeros.push(Numero {
                valor,
                largo,
                posicion,
            });
        }
    }
    numeros
}

fn simbolos(input: & str) -> Vec<Simbolo> {
    let mut simbolos = Vec::new();
    for (y, line) in input.lines().enumerate() {
        for (x, _valor) in line.chars().enumerate() {
            if !_valor.is_ascii_digit() && _valor != '.' {
                let posicion = Position {
                    x: x as u32,
                    y: y as u32,
                };
                simbolos.push(Simbolo { _valor, posicion });
            }
        }
    }
    simbolos
}

fn process_input(input: &str) -> u32 {
    let numeros = numeros(input);
    let simbolos = simbolos(input);
    simbolos
        .iter()
        .filter_map(|simbolo| {
            let vecinos = simbolo.vecinos(&numeros);
            if vecinos.len() == 2 {
                Some(vecinos.iter().map(|numero| numero.valor).product::<u32>())
            } else {
                None
            }
        }).sum()
        
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
        Ok(_) =>  println!("{}", process_input(&s)),            
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_input() {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664..598.";
        assert_eq!(process_input(input), 467835);
    }
}
