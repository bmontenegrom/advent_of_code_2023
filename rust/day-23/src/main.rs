use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
    ops::Add,
};

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
    fn neighbors(&self) -> Vec<Self> {
        let left = Self::new(-1, 0);
        let right = Self::new(1, 0);
        let up = Self::new(0, -1);
        let down = Self::new(0, 1);
        vec![*self + left, *self + right, *self + up, *self + down]
    }
}

#[derive(Debug)]
struct Map {
    map: HashMap<Point, char>,
    start: Point,
    end: Point,
}

impl Map {
    fn new(input: &str) -> Self {
        let map = input
            .lines()
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .map(move |(x, c)| (Point::new(x as i32, y as i32), c))
            })
            .collect::<HashMap<Point, char>>();

        let max_x = map.keys().map(|p| p.x).max().unwrap();
        let max_y = map.keys().map(|p| p.y).max().unwrap();
        let start = Point::new(1, 0);
        let end = Point::new(max_x - 1, max_y);
        Self { map, start, end }
    }

    fn neighbors(&self, point: &Point) -> Vec<Point> {
        let left = Point::new(-1, 0);
        let right = Point::new(1, 0);
        let up = Point::new(0, -1);
        let down = Point::new(0, 1);
        let directions = vec![left, right, up, down];
        match self.map.get(point).unwrap() {
            '>' => return vec![*point + right],
            '<' => return vec![*point + left],
            '^' => return vec![*point + up],
            'v' => return vec![*point + down],
            _ => {}
        }
        let mut neighbors = Vec::new();
        for direction in directions {
            let neighbor = *point + direction;
            //if neighbot is not valid, skip
            match self.map.get(&neighbor) {
                None => continue,
                Some(c) => match (c, direction.x, direction.y) {
                    ('#', _, _) => continue,
                    ('>', -1, 0) => continue,
                    ('<', 1, 0) => continue,
                    ('^', 0, 1) => continue,
                    ('v', 0, -1) => continue,
                    _ => neighbors.push(neighbor),
                },
            }
        }
        neighbors
    }

    fn longest_path(&self) -> usize {
        let mut ends = Vec::new();
        let mut visited = HashSet::new();
        self.longest_path_dfs(self.start, &mut visited, 0, &mut ends);
        *ends.iter().max().unwrap()
    }

    fn longest_path_dfs(
        &self,
        point: Point,
        visited: &mut HashSet<Point>,
        steps: usize,
        ends: &mut Vec<usize>,
    ) {
        if point == self.end {
            ends.push(steps);
            return;
        }
        if visited.contains(&point) {
            return;
        }
        visited.insert(point);
        for neighbor in self.neighbors(&point) {
            if visited.contains(&neighbor) {
                continue;
            }
            self.longest_path_dfs(neighbor, visited, steps + 1, ends);
        }
        visited.remove(&point);
    }

    fn neighbors_part2(&self, point: &Point) -> Vec<Point> {
        let mut neighbors = Vec::new();
        for neighbor in point.neighbors() {
            match self.map.get(&neighbor) {
                None => continue,
                Some(c) => match c {
                    '#' => continue,
                    _ => neighbors.push(neighbor),
                },
            }
        }
        neighbors
    }

    //find distances from branching points to other branching points
    fn find_branching_distances(&self) -> HashMap<Point, Vec<(Point, usize)>> {
        //rename points acording to branching
        let map = self
            .map
            .iter()
            .filter(|(_, c)| **c != '#')
            .map(|(p, _)| {
                let n = self.neighbors_part2(p).len();
                (*p, n)
            })
            .collect::<HashMap<_, _>>();
        //find branching points, start and end
        let nodes = map
            .iter()
            .filter(|(_, n)| **n != 2)
            .map(|(p, _)| *p)
            .collect::<HashSet<_>>();
        let mut distances: HashMap<Point, Vec<(Point, usize)>> = HashMap::new();
        for node in nodes.iter(){
            for mut neighor in self.neighbors_part2(node){
                let mut prev = *node;
                let mut dist = 0;
                loop {
                    dist += 1;
                    let neighbors = self.neighbors_part2(&neighor);
                    let neighbors = neighbors.iter().filter(|n|**n!=prev).collect::<Vec<_>>();
                    if neighbors.len() != 1{
                        distances.entry(*node).or_default().push((neighor, dist));
                        break;
                    }
                    prev = neighor;
                    neighor = *neighbors[0];
                    
                }
            }
        }
        distances
    }

    fn longest_path_part2(&self) -> usize {
        let distances = self.find_branching_distances();
        let mut visited = HashSet::new();
        let mut ends = Vec::new();
        self.longest_path_part2_dfs(self.start, &mut visited, 0, &mut ends, &distances);
        *ends.iter().max().unwrap()
    }

    fn longest_path_part2_dfs(
        &self,
        point: Point,
        visited: &mut HashSet<Point>,
        steps: usize,
        ends: &mut Vec<usize>,
        distances: &HashMap<Point, Vec<(Point, usize)>>,
    ) {
        if point == self.end{
            ends.push(steps);
            return;
        }
        if visited.contains(&point) {
            return;
        }
        visited.insert(point);
        let neighbors = distances.get(&point).unwrap();
        for (neighbor, dist) in neighbors {
            if visited.contains(neighbor) {
                continue;
            }
            self.longest_path_part2_dfs(*neighbor, visited, steps + dist, ends, distances);
        }
        visited.remove(&point);
    }
}

fn main() {
    let input = include_str!("input.txt");
    let map = Map::new(input);
    let now = std::time::Instant::now();
    println!("Part 1: {} in: {:?}", map.longest_path(), now.elapsed());
    println!("Part 2: {} in: {:?}", map.longest_path_part2(), now.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = include_str!("input_test.txt");
        let map = Map::new(input);
        assert_eq!(map.longest_path(), 94);
    }

    #[test]
    fn test_part2() {
        let input = include_str!("input_test.txt");
        let map = Map::new(input);
        assert_eq!(map.longest_path_part2(), 154);
    }
}
