use std::iter::zip;

#[derive(Debug)]
struct Mirrors {
    rows: Vec<Vec<char>>,
    columns: Vec<Vec<char>>,
}

impl Mirrors {
    fn new(input: &str) -> Self {
        let rows: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
        let columns = (0..rows[0].len())
            .map(|i| rows.iter().map(|r| r[i]).collect())
            .collect();
        Mirrors { rows, columns }
    }

    fn equal_rows(&self, primera: usize, segunda: usize) -> bool {
        self.rows[primera] == self.rows[segunda]
    }

    fn equal_columns(&self, primera: usize, segunda: usize) -> bool {
        self.columns[primera] == self.columns[segunda]
    }

    fn diferences_in_rows(&self, primera: usize, segunda: usize) -> usize {
        zip(&self.rows[primera], &self.rows[segunda])
            .filter(|(a, b)| a != b)
            .count()
    }

    fn diferences_in_columns(&self, primera: usize, segunda: usize) -> usize {
        zip(&self.columns[primera], &self.columns[segunda])
            .filter(|(a, b)| a != b)
            .count()
    }

    fn solve_perfect_mirror(&self) -> usize {
        for i in 0..self.rows.len() - 1 {
            if self.equal_rows(i, i + 1) {
                let min_dist = i.min(self.rows.len() - i - 2);
                let mut is_mirror = true;
                for j in 1..=min_dist {
                    is_mirror = is_mirror && self.equal_rows(i - j, i + j + 1);
                }
                if is_mirror {
                    return (i + 1) * 100;
                }
            }
        }
        for i in 0..self.columns.len() - 1 {
            if self.equal_columns(i, i + 1) {
                let min_dist = i.min(self.columns.len() - i - 2);
                let mut is_mirror = true;
                for j in 1..=min_dist {
                    is_mirror = is_mirror && self.equal_columns(i - j, i + j + 1);
                }
                if is_mirror {
                    return i + 1;
                }
            }
        }
        0
    }

    fn solve_one_smudge(&self) -> usize {
        for i in 0..self.rows.len() - 1 {
            let mut dif = self.diferences_in_rows(i, i + 1);
            if dif <= 1 {
                let min_dist = i.min(self.rows.len() - i - 2);
                let mut is_mirror = true;
                for j in 1..=min_dist {
                    dif += self.diferences_in_rows(i - j, i + j + 1);
                    is_mirror = is_mirror && dif <= 1;
                }
                if is_mirror && dif == 1 {
                    return (i + 1) * 100;
                }
            }
        }

        for i in 0..self.columns.len() - 1 {
            let mut dif = self.diferences_in_columns(i, i + 1);
            if dif <= 1 {
                let min_dist = i.min(self.columns.len() - i - 2);
                let mut is_mirror = true;
                for j in 1..=min_dist {
                    dif += self.diferences_in_columns(i - j, i + j + 1);
                    is_mirror = is_mirror && dif <= 1;
                }
                if is_mirror && dif == 1 {
                    return i + 1;
                }
            }
        }
        0
    }
}

fn parse_input(input: &str) -> Vec<Mirrors> {
    input.split("\n\n").map(Mirrors::new).collect()
}

fn solve_part1(input: &str) -> usize {
    let mirrors = parse_input(input);
    mirrors
        .iter()
        .map(|m| m.solve_perfect_mirror())
        .sum::<usize>()
}

fn solve_part2(input: &str) -> usize {
    let mirrors = parse_input(input);
    mirrors.iter().map(|m| m.solve_one_smudge()).sum::<usize>()
}
fn main() {
    let input = include_str!("input.txt");
    println!("Part 1: {}", solve_part1(input));
    println!("Part 2: {}", solve_part2(input));
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_parte1() {
        let input = include_str!("input_test.txt");
        let mirrors = parse_input(input);
        let result = mirrors
            .iter()
            .map(|m| m.solve_perfect_mirror())
            .sum::<usize>();
        assert_eq!(result, 405);
    }
    #[test]
    fn test_diferences_in_rows() {
        let input = include_str!("input_test.txt");
        let mirrors = parse_input(input);
        let result = mirrors[0].diferences_in_rows(0, 1);
        assert_eq!(result, 5);
        let result = mirrors[0].diferences_in_rows(2, 3);
        assert_eq!(result, 0)
    }
    #[test]
    fn test_diferences_in_columns() {
        let input = include_str!("input_test.txt");
        let mirrors = parse_input(input);
        let result = mirrors[0].diferences_in_columns(0, 1);
        assert_eq!(result, 2);
        let result = mirrors[0].diferences_in_columns(4, 5);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_parte2() {
        let input = include_str!("input_test.txt");
        let result = solve_part2(input);
        assert_eq!(result, 400);
    }
}
