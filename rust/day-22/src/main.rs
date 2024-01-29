use std::collections::{HashMap, HashSet, VecDeque};

use itertools::Itertools;

#[derive(Debug, Clone)]
struct Point {
    x: usize,
    y: usize,
    z: usize,
}
impl Point {
    fn new(x: usize, y: usize, z: usize) -> Self {
        Self { x, y, z }
    }
    fn parse(input: &str) -> Self {
        let (x, y, z) = input
            .split(',')
            .map(|s| s.parse::<usize>().unwrap())
            .collect_tuple()
            .unwrap();
        Point::new(x, y, z)
    }
}

#[derive(Debug, Clone)]
struct Brick {
    start: Point,
    end: Point,
    id: usize,
}
impl Brick {
    fn new(start: Point, end: Point, id: usize) -> Self {
        Self { start, end, id }
    }

    fn parse(input: &str, id: usize) -> Self {
        let (start, end) = input.split('~').collect_tuple().unwrap();
        let start = Point::parse(start);
        let end = Point::parse(end);
        Brick::new(start, end, id)
    }

    fn intersecs_xy(&self, other: &Self) -> bool {
        self.start.x <= other.end.x
            && self.end.x >= other.start.x
            && self.start.y <= other.end.y
            && self.end.y >= other.start.y
    }

    fn hiegs_z_to_drop(&self, others: &[Self]) -> usize {
        others
            .iter()
            .filter(|brick| self.intersecs_xy(brick) && brick.end.z < self.start.z)
            .map(|brick| brick.end.z)
            .max()
            .unwrap_or(0)
    }
    fn ids_above(&self, others: &[Self]) -> Vec<usize> {
        others
            .iter()
            .filter(|brick| self.intersecs_xy(brick) && brick.start.z == self.end.z + 1)
            .map(|brick| brick.id)
            .collect()
    }
    fn ids_under(&self, others: &[Self]) -> Vec<usize> {
        others
            .iter()
            .filter(|brick| self.intersecs_xy(brick) && brick.end.z == self.start.z - 1)
            .map(|brick| brick.id)
            .collect()
    }
}

#[derive(Debug)]
struct Wall {
    bricks: Vec<Brick>,
}

impl Wall {
    fn new(bricks: Vec<Brick>) -> Self {
        Self { bricks }
    }
    fn parse(input: &str) -> Self {
        let mut bricks: Vec<_> = input
            .lines()
            .enumerate()
            .map(|(id, line)| Brick::parse(line, id))
            .collect();
        bricks.sort_by_key(|brick| brick.start.z);

        Wall::new(bricks)
    }

    fn droped_bricks(&self) -> Vec<Brick> {
        let mut result = Vec::new();
        self.bricks.iter().for_each(|brick| {
            let mut new_brick = brick.clone();
            let lowest = new_brick.hiegs_z_to_drop(&result);
            let lenght = new_brick.end.z - new_brick.start.z;
            new_brick.start.z = lowest + 1;
            new_brick.end.z = new_brick.start.z + lenght;
            result.push(new_brick);
        });
        result
    }
}

fn solve_part1(input: &str) -> usize {
    let wall = Wall::parse(input);
    let droped = wall.droped_bricks();
    let map_under: HashMap<usize, Vec<usize>> = droped
        .iter()
        .map(|brick| {
            let under = brick.ids_under(&droped);
            (brick.id, under)
        })
        .collect();
    let map_above: HashMap<usize, Vec<usize>> = droped
        .iter()
        .map(|brick| {
            let above = brick.ids_above(&droped);
            (brick.id, above)
        })
        .collect();
    let mut removable = map_under.iter().filter(|(_, under)| under.len() > 1).fold(
        HashSet::new(),
        |mut acc, (_, under)| {
            under.iter().for_each(|id| {
                acc.insert(*id);
            });
            acc
        },
    );
    map_under
        .iter()
        .filter(|(_, under)| under.len() == 1)
        .for_each(|(_, under)| {
            removable.remove(&under[0]);
        });
    map_above
        .iter()
        .filter(|(_, above)| above.is_empty())
        .map(|(id, _)| id)
        .for_each(|id| {
            removable.insert(*id);
        });
    removable.len()
}

fn solve_part2(input: &str) -> usize {
    let wall = Wall::parse(input);
    let droped = wall.droped_bricks();
    let map_under: HashMap<usize, Vec<usize>> = droped
        .iter()
        .map(|brick| {
            let under = brick.ids_under(&droped);
            (brick.id, under)
        })
        .collect();
    let map_above: HashMap<usize, Vec<usize>> = droped
        .iter()
        .map(|brick| {
            let above = brick.ids_above(&droped);
            (brick.id, above)
        })
        .collect();
    let mut count = 0;
    droped.iter().for_each(|brick| {
        let mut desintegrated = HashSet::new();
        desintegrated.insert(brick.id);

        let above = map_above.get(&brick.id).unwrap();
        let mut queue = VecDeque::new();
        above.iter().for_each(|id| {
            queue.push_back(*id);
        });
        while let Some(id) = queue.pop_front() {
            let under = map_under.get(&id).unwrap();
            if under.iter().all(|id| desintegrated.contains(id)) {
                desintegrated.insert(id);
                let above = map_above.get(&id).unwrap();
                above.iter().for_each(|id| {
                    queue.push_back(*id);
                });
            }
        }
        count += desintegrated.len() - 1;
    });

    count
}

fn main() {
    let input = include_str!("input.txt");
    let now = std::time::Instant::now();
    println!("Part 1: {} in {:?}", solve_part1(input), now.elapsed());
    println!("Part 2: {} in {:?}", solve_part2(input), now.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = include_str!("input_test.txt");
        assert_eq!(solve_part1(input), 5);
    }
    #[test]
    fn test_part2() {
        let input = include_str!("input_test.txt");
        assert_eq!(solve_part2(input), 7);
    }
}
