#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn distance(&self, other: &Point) -> usize {
        (self.x as isize - other.x as isize).unsigned_abs()
            + (self.y as isize - other.y as isize).unsigned_abs()
    }
    fn mins(&self, other: &Point) -> (usize, usize) {
        (self.x.min(other.x) , self.y.min(other.y))
    }
    fn maxs(&self, other: &Point) -> (usize, usize) {
        (self.x.max(other.x) , self.y.max(other.y))
    }
}

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn empty_rows(grid: &[Vec<char>]) -> Vec<usize> {
    grid.iter()
        .enumerate()
        .filter(|(_, row)| row.iter().all(|&c| c == '.'))
        .map(|(y, _)| y)
        .collect()
}

fn empty_cols(grid: &[Vec<char>]) -> Vec<usize> {
    (0..grid[0].len())
        .filter(|&x| grid.iter().all(|row| row[x] == '.'))
        .collect()
}

fn galaxys(grid: &[Vec<char>]) -> Vec<Point> {
    grid.iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .filter(|(_, &c)| c == '#')
                .map(move |(x, _)| Point { x, y })
        })
        .collect()
}

fn process_input(input: &str) -> Result<usize, String> {
    let grid = parse_input(input);
    let empty_rows = empty_rows(&grid);
    let empty_cols = empty_cols(&grid);
    let galaxys = galaxys(&grid);
    let mut sum = 0;
    for (i, galaxy) in galaxys.iter().enumerate() {
        for other in &galaxys[i + 1..] {
            let distance = galaxy.distance(other);
            let (min_x, min_y) = galaxy.mins(other);
            let (max_x, max_y) = galaxy.maxs(other);
            let rows_exp = empty_rows
                .iter()
                .filter(|&&y| y > min_y && y < max_y)
                .count();
            let cols_exp = empty_cols
                .iter()
                .filter(|&&x| x > min_x && x < max_x)
                .count();
            sum += distance + rows_exp + cols_exp;
        }
    }

    Ok(sum)
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
        let input = include_str!("test_input.txt");
        let result = process_input(input);
        assert_eq!(result, Ok(374));
    }
}