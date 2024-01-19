use itertools::{Itertools, Position};

fn process_input(input: &str) -> Result<i64, String> {
    let result = input
        .lines()
        .map(|line| {
            let mut numbers = line
                .split_whitespace()
                .map(|num| num.parse::<i64>().unwrap())
                .collect::<Vec<i64>>();
            let mut ultimos = Vec::new();
            loop {
                if numbers.iter().all(|num| *num == 0) {
                    break;
                }
                numbers = numbers
                    .iter()
                    .tuple_windows()
                    .with_position()
                    .map(|(pos, (iz, der))| {
                        match pos {
                            Position::First | Position::Only => ultimos.push(*iz),
                            _ => {}
                        }
                        der - iz
                    })
                    .collect::<Vec<i64>>();
            }
            let result = ultimos.iter().rev().fold(0, |acc, num| num - acc);
            result
        })
        .sum();

    Ok(result)
}
fn main() {
    let input = include_str!("input.txt");
    println!("{:?}", process_input(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_input() {
        let input = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";
        let result = process_input(input).unwrap();
        assert_eq!(result, 2);
    }
}
