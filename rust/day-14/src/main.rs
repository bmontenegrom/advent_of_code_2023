use std::{collections::HashSet, hash::Hasher};

#[derive(Debug, Eq)]
struct State {
    plataform: Plataform,
    id: usize,
}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.plataform == other.plataform
    }
}

impl std::hash::Hash for State {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.plataform.hash(state);
    }
}
impl State {
    fn new(plataform: Plataform, id: usize) -> Self {
        State { plataform, id }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Plataform {
    grid: Vec<Vec<char>>,
}

impl Plataform {
    fn new(input: &str) -> Self {
        let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
        Plataform { grid }
    }

    fn calculate_score(&self) -> usize {
        self.grid
            .iter()
            .enumerate()
            .map(|(i, row)| row.iter().filter(|c| **c == 'O').count() * (self.grid.len() - i))
            .sum()
    }

    fn move_north(&mut self) {
        for x in 0..self.grid[0].len() {
            for y in 1..self.grid.len() {
                match self.grid[y][x] {
                    'O' => match self.grid[y - 1][x] {
                        '.' => {
                            let mut change = (y - 1) as isize;
                            while change >= 0 && self.grid[change as usize][x] == '.' {
                                change -= 1;
                            }
                            self.grid[(change + 1) as usize][x] = 'O';
                            self.grid[y][x] = '.';
                        }
                        _ => {}
                    },
                    _ => {}
                }
            }
        }
    }

    fn move_south(&mut self) {
        for x in 0..self.grid[0].len() {
            for y in (0..self.grid.len() - 1).rev() {
                match self.grid[y][x] {
                    'O' => match self.grid[y + 1][x] {
                        '.' => {
                            let mut change = y + 1;
                            while change < self.grid.len() && self.grid[change][x] == '.' {
                                change += 1;
                            }
                            self.grid[change - 1][x] = 'O';
                            self.grid[y][x] = '.';
                        }
                        _ => {}
                    },
                    _ => {}
                }
            }
        }
    }
    fn move_west(&mut self) {
        for y in 0..self.grid.len() {
            for x in 1..self.grid[0].len() {
                match self.grid[y][x] {
                    'O' => match self.grid[y][x - 1] {
                        '.' => {
                            let mut change = (x - 1) as isize;
                            while change >= 0 && self.grid[y][change as usize] == '.' {
                                change -= 1;
                            }
                            self.grid[y][(change + 1) as usize] = 'O';
                            self.grid[y][x] = '.';
                        }
                        _ => {}
                    },
                    _ => {}
                }
            }
        }
    }

    fn move_east(&mut self) {
        for y in 0..self.grid.len() {
            for x in (0..self.grid[0].len() - 1).rev() {
                match self.grid[y][x] {
                    'O' => match self.grid[y][x + 1] {
                        '.' => {
                            let mut change = x + 1;
                            while change < self.grid[0].len() && self.grid[y][change] == '.' {
                                change += 1;
                            }
                            self.grid[y][change - 1] = 'O';
                            self.grid[y][x] = '.';
                        }
                        _ => {}
                    },
                    _ => {}
                }
            }
        }
    }
    fn cycle(&mut self) {
        self.move_north();
        self.move_west();
        self.move_south();
        self.move_east();
    }
}

fn solve_part1(input: &str) -> usize {
    let mut plataform = Plataform::new(input);
    plataform.move_north();
    plataform.calculate_score()
}

fn solve_part2(input: &str) -> usize {
    let mut calculados = HashSet::new();
    let mut plataform = Plataform::new(input);
    for i in 0..1000000000 {
        calculados.insert(State::new(plataform.clone(), i));
        plataform.cycle();
        if let Some(state) = calculados.get(&State::new(plataform.clone(), i)) {
            let cycle_len = i + 1 - state.id;
            let remaining = 1000000000 - i - 1;
            let remaining = remaining % cycle_len;
            for _ in 0..remaining {
                //tambien se pueede obtener el ya calculado con el id i + remaining-cycle_len+1
                plataform.cycle();
            }
            return plataform.calculate_score();
        }
    }
    0
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
        assert_eq!(solve_part1(input), 136);
    }
    #[test]
    fn test_part2() {
        let input = include_str!("input_test.txt");
        assert_eq!(solve_part2(input), 64);
    }
}
