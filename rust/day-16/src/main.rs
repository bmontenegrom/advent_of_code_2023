use std::collections::{HashSet, VecDeque};

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
enum Directions {
    North,
    South,
    East,
    West,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Point {
    x: i64,
    y: i64,
}
impl Point {
    fn new(x: i64, y: i64) -> Self {
        Point { x, y }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Beam {
    position: Point,
    direction: Directions,
}

impl Beam {
    fn new(position: Point, direction: Directions) -> Self {
        Beam {
            position,
            direction,
        }
    }

    fn is_valid_position(&self, grid: &Grid) -> bool {
        self.position.x >= 0
            && self.position.x < grid.grid[0].len() as i64
            && self.position.y >= 0
            && self.position.y < grid.grid.len() as i64
    }

    fn move_beam(&mut self, grid: &Grid) -> (Option<Beam>, Option<Beam>) {
        let mut left = None;
        let mut right = None;
        let current_char = grid.grid[self.position.y as usize][self.position.x as usize];
        match self.direction {
            Directions::North => match current_char {
                '.' => {
                    let next_y = self.position.y - 1;
                    let next_beam =
                        Beam::new(Point::new(self.position.x, next_y), Directions::North);
                    if next_beam.is_valid_position(grid) {
                        left = Some(next_beam)
                    }
                }
                '/' => {
                    let next_x = self.position.x + 1;
                    let next_beam =
                        Beam::new(Point::new(next_x, self.position.y), Directions::East);
                    if next_beam.is_valid_position(grid) {
                        left = Some(next_beam)
                    }
                }
                '\\' => {
                    let next_x = self.position.x - 1;
                    let next_beam =
                        Beam::new(Point::new(next_x, self.position.y), Directions::West);
                    if next_beam.is_valid_position(grid) {
                        left = Some(next_beam)
                    }
                }
                '|' => {
                    let next_y = self.position.y - 1;
                    let next_beam =
                        Beam::new(Point::new(self.position.x, next_y), Directions::North);
                    if next_beam.is_valid_position(grid) {
                        left = Some(next_beam)
                    }
                }
                '-' => {
                    let next_x = self.position.x + 1;
                    let next_beam =
                        Beam::new(Point::new(next_x, self.position.y), Directions::East);
                    if next_beam.is_valid_position(grid) {
                        left = Some(next_beam)
                    }
                    let next_x = self.position.x - 1;
                    let next_beam =
                        Beam::new(Point::new(next_x, self.position.y), Directions::West);
                    if next_beam.is_valid_position(grid) {
                        right = Some(next_beam)
                    }
                }
                _ => panic!("no deberia pasar"),
            },
            Directions::South => match current_char {
                '.' => {
                    let next_y = self.position.y + 1;
                    let next_beam =
                        Beam::new(Point::new(self.position.x, next_y), Directions::South);
                    if next_beam.is_valid_position(grid) {
                        left = Some(next_beam)
                    }
                }
                '/' => {
                    let next_x = self.position.x - 1;
                    let next_beam =
                        Beam::new(Point::new(next_x, self.position.y), Directions::West);
                    if next_beam.is_valid_position(grid) {
                        left = Some(next_beam)
                    }
                }
                '\\' => {
                    let next_x = self.position.x + 1;
                    let next_beam =
                        Beam::new(Point::new(next_x, self.position.y), Directions::East);
                    if next_beam.is_valid_position(grid) {
                        left = Some(next_beam)
                    }
                }
                '|' => {
                    let next_y = self.position.y + 1;
                    let next_beam =
                        Beam::new(Point::new(self.position.x, next_y), Directions::South);
                    if next_beam.is_valid_position(grid) {
                        left = Some(next_beam)
                    }
                }
                '-' => {
                    let next_x = self.position.x + 1;
                    let next_beam =
                        Beam::new(Point::new(next_x, self.position.y), Directions::East);
                    if next_beam.is_valid_position(grid) {
                        left = Some(next_beam)
                    }
                    let next_x = self.position.x - 1;
                    let next_beam =
                        Beam::new(Point::new(next_x, self.position.y), Directions::West);
                    if next_beam.is_valid_position(grid) {
                        right = Some(next_beam)
                    }
                }
                _ => panic!("no deberia pasar"),
            },
            Directions::East => match current_char {
                '.' => {
                    let next_x = self.position.x + 1;
                    let next_beam =
                        Beam::new(Point::new(next_x, self.position.y), Directions::East);
                    if next_beam.is_valid_position(grid) {
                        left = Some(next_beam)
                    }
                }
                '/' => {
                    let next_y = self.position.y - 1;
                    let next_beam =
                        Beam::new(Point::new(self.position.x, next_y), Directions::North);
                    if next_beam.is_valid_position(grid) {
                        left = Some(next_beam)
                    }
                }
                '\\' => {
                    let next_y = self.position.y + 1;
                    let next_beam =
                        Beam::new(Point::new(self.position.x, next_y), Directions::South);
                    if next_beam.is_valid_position(grid) {
                        left = Some(next_beam)
                    }
                }
                '|' => {
                    let next_y = self.position.y - 1;
                    let next_beam =
                        Beam::new(Point::new(self.position.x, next_y), Directions::North);
                    if next_beam.is_valid_position(grid) {
                        left = Some(next_beam)
                    }
                    let next_y = self.position.y + 1;
                    let next_beam =
                        Beam::new(Point::new(self.position.x, next_y), Directions::South);
                    if next_beam.is_valid_position(grid) {
                        right = Some(next_beam)
                    }
                }
                '-' => {
                    let next_x = self.position.x + 1;
                    let next_beam =
                        Beam::new(Point::new(next_x, self.position.y), Directions::East);
                    if next_beam.is_valid_position(grid) {
                        left = Some(next_beam)
                    }
                }
                _ => {}
            },
            Directions::West => match current_char {
                '.' => {
                    let next_x = self.position.x - 1;
                    let next_beam =
                        Beam::new(Point::new(next_x, self.position.y), Directions::West);
                    if next_beam.is_valid_position(grid) {
                        left = Some(next_beam)
                    }
                }
                '/' => {
                    let next_y = self.position.y + 1;
                    let next_beam =
                        Beam::new(Point::new(self.position.x, next_y), Directions::South);
                    if next_beam.is_valid_position(grid) {
                        left = Some(next_beam)
                    }
                }
                '\\' => {
                    let next_y = self.position.y - 1;
                    let next_beam =
                        Beam::new(Point::new(self.position.x, next_y), Directions::North);
                    if next_beam.is_valid_position(grid) {
                        left = Some(next_beam)
                    }
                }
                '|' => {
                    let next_y = self.position.y - 1;
                    let next_beam =
                        Beam::new(Point::new(self.position.x, next_y), Directions::North);
                    if next_beam.is_valid_position(grid) {
                        left = Some(next_beam)
                    }
                    let next_y = self.position.y + 1;
                    let next_beam =
                        Beam::new(Point::new(self.position.x, next_y), Directions::South);
                    if next_beam.is_valid_position(grid) {
                        right = Some(next_beam)
                    }
                }
                '-' => {
                    let next_x = self.position.x - 1;
                    let next_beam =
                        Beam::new(Point::new(next_x, self.position.y), Directions::West);
                    if next_beam.is_valid_position(grid) {
                        left = Some(next_beam)
                    }
                }
                _ => panic!("no deberia pasar"),
            },
        }
        (left, right)
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Grid {
    grid: Vec<Vec<char>>,
}

impl Grid {
    fn new(input: &str) -> Self {
        let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
        Grid { grid }
    }

    fn energized_tiles(&self, start_bean: Beam) -> usize {
        let mut visited_tiles = HashSet::new();
        let mut calculated_beams: HashSet<Beam> = HashSet::new();
        let mut cola = VecDeque::new();
        cola.push_back(start_bean);
        while !cola.is_empty() {
            let mut current_beam = cola.pop_front().unwrap();
            visited_tiles.insert(current_beam.position.clone());
            if !calculated_beams.contains(&current_beam) {
                calculated_beams.insert(current_beam.clone());
                let (left, right) = current_beam.move_beam(self);
                if let Some(left) = left {
                    cola.push_back(left);
                }
                if let Some(right) = right {
                    cola.push_back(right);
                }
            }
        }

        visited_tiles.len()
    }
}

fn process_part1(input: &str) -> usize {
    let grid = Grid::new(input);
    let start_beam = Beam::new(Point::new(0, 0), Directions::East);
    grid.energized_tiles(start_beam)
}

fn process_part2(input: &str) -> usize {
    let grid = Grid::new(input);
    let mut max = grid.grid.iter().enumerate().fold(0, |mut max, (y, _)| {
        let start_beam = Beam::new(Point::new(0, y as i64), Directions::East);
        max = max.max(grid.energized_tiles(start_beam));
        max
    });
    max = grid.grid[0]
        .iter()
        .enumerate()
        .fold(max, |mut max, (x, _)| {
            let start_beam = Beam::new(Point::new(x as i64, 0), Directions::South);
            max = max.max(grid.energized_tiles(start_beam));
            max
        });
    let last_row = grid.grid.len() - 1;
    max = grid.grid[last_row]
        .iter()
        .enumerate()
        .fold(max, |mut max, (x, _)| {
            let start_beam = Beam::new(Point::new(x as i64, last_row as i64), Directions::North);
            max = max.max(grid.energized_tiles(start_beam));
            max
        });
    let last_column = grid.grid[0].len() - 1;
    max = grid.grid.iter().enumerate().fold(max, |mut max, (y, _)| {
        let start_beam = Beam::new(Point::new(last_column as i64, y as i64), Directions::West);
        max = max.max(grid.energized_tiles(start_beam));
        max
    });
    max
}

fn main() {
    let input = include_str!("input.txt");
    let now = std::time::Instant::now();
    println!("Part 1: {} in : {:?}", process_part1(input), now.elapsed());
    let now = std::time::Instant::now();
    println!("Part 2: {} in : {:?}", process_part2(input), now.elapsed());
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_part1() {
        let input = include_str!("input_test.txt");
        let result = process_part1(input);
        assert_eq!(result, 46);
    }

    #[test]
    fn test_part2() {
        let input = include_str!("input_test.txt");
        let result = process_part2(input);
        assert_eq!(result, 51);
    }
}
