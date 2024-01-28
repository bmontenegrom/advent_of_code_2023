use std::collections::{HashMap, HashSet, VecDeque, hash_map::Entry::Vacant};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
    fn neigbours(&self) -> Vec<Point> {
        vec![
            Point::new(self.x - 1, self.y),
            Point::new(self.x + 1, self.y),
            Point::new(self.x, self.y - 1),
            Point::new(self.x, self.y + 1),
        ]
    }
}

struct Garden {
    rocks: HashSet<Point>,
    start: Point,
    max_x: i32,
    max_y: i32,
}

impl Garden {
    fn new(input: &str) -> Self {
        let max_y = input.lines().count() as i32;
        let max_x = input.lines().next().unwrap().len() as i32;
        let mut rocks = HashSet::new();
        let mut start = Point::new(0, 0);
        for (y, line) in input.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                if c == 'S' {
                    start = Point::new(x as i32, y as i32);
                } else if c == '#' {
                    rocks.insert(Point::new(x as i32, y as i32));
                }
            }
        }
        Self {
            rocks,
            start,
            max_x,
            max_y,
        }
    }

    fn neigbours(&self, point: &Point) -> Vec<Point> {
        point
            .neigbours()
            .into_iter()
            .filter(|p| {
                p.x >= 0
                    && p.y >= 0
                    && p.x < self.max_x
                    && p.y < self.max_y
                    && !self.rocks.contains(p)
            })
            .collect()
    }

    fn solve_part_1(&self, steps: usize) -> usize {
        let mut current = HashSet::new();
        current.insert(self.start);
        let mut next = HashSet::new();
        for _ in 0..steps {
            for point in current.iter() {
                for neighbour in self.neigbours(point) {
                    next.insert(neighbour);
                }
            }
            std::mem::swap(&mut current, &mut next);
            next.clear();
        }
        current.len()
    }

    fn calculate_distances(&self) -> HashMap<Point, usize> {
        let mut distances = HashMap::new();
        let mut frontier = VecDeque::new();
        frontier.push_back((self.start, 0));
        while let Some((point, distance)) = frontier.pop_front() {
            if let Vacant(e) = distances.entry(point) {
                e.insert(distance);
                for neighbour in self.neigbours(&point) {
                    frontier.push_back((neighbour, distance + 1));
                }
            }
        }
        distances
    }
    //magic numbers come from https://www.youtube.com/watch?v=KOHYAlsOwOM
    //numbers of steps is 26501365 = 202300*131 + 65
    //65 = 131/2
    //131 is the sice of the gird, 202300 is the number of grids we can reach
    //of thath 202300 some are cover completely and some are not
    //thats the corners, depeding on the parity of the ditsance we have to add or substract
    fn solve_part_2(&self) -> usize {
        let distances = self.calculate_distances();
        let (odds, evens, odd_corners, even_corners) = distances.iter().fold(
            (0_usize, 0_usize, 0_usize, 0_usize),
            |(odds, evens, odd_corners, even_corners), (_, distance)| {
                if *distance % 2 == 1 && *distance > 65 {
                    (odds + 1, evens, odd_corners + 1, even_corners)
                } else if *distance % 2 == 1 {
                    (odds + 1, evens, odd_corners, even_corners)
                } else if *distance % 2 == 0 && *distance > 65 {
                    (odds, evens + 1, odd_corners, even_corners + 1)
                } else {
                    (odds, evens + 1, odd_corners, even_corners)
                }
            },
        );
        let count = 202300;
        let total_odds = odds * (count + 1) * (count + 1); //odds*(n+1)^2
        let total_evens = evens * (count * count); //evens*n^2
        let total_odd_corners = odd_corners * (count + 1); //odd_corners*(n+1)
        let total_even_corners = even_corners * count; //even_corners*n
        total_odds + total_evens - total_odd_corners + total_even_corners
    }
}

fn main() {
    let input = include_str!("input.txt");
    let garden = Garden::new(input);
    let now = std::time::Instant::now();
    println!("Part 1: {} in {:?}", garden.solve_part_1(64), now.elapsed());
    println!("Part 2: {} in {:?}", garden.solve_part_2(), now.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = include_str!("input_test.txt");
        let garden = Garden::new(input);
        assert_eq!(garden.solve_part_1(6), 16);
    }
}
