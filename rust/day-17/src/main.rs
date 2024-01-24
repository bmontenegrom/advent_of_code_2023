use std::{
    collections::{BinaryHeap, HashMap},
    vec,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn opposite(&self) -> Direction {
        match self {
            Direction::North => Direction::South,
            Direction::South => Direction::North,
            Direction::East => Direction::West,
            Direction::West => Direction::East,
        }
    }
}

#[derive(Debug)]
struct Grid {
    grid: Vec<Vec<usize>>,
}
impl Grid {
    fn new(input: &str) -> Self {
        let grid = input
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| c.to_digit(10).unwrap() as usize)
                    .collect()
            })
            .collect();
        Self { grid }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
    fn valid_nexts(&self, grid: &Grid) -> Vec<(Direction, Point)> {
        let mut nexts = Vec::new();
        if self.x > 0 {
            nexts.push((Direction::West, Point::new(self.x - 1, self.y)));
        }
        if self.x < grid.grid[0].len() - 1 {
            nexts.push((Direction::East, Point::new(self.x + 1, self.y)));
        }
        if self.y > 0 {
            nexts.push((Direction::North, Point::new(self.x, self.y - 1)));
        }
        if self.y < grid.grid.len() - 1 {
            nexts.push((Direction::South, Point::new(self.x, self.y + 1)));
        }
        nexts
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Node {
    point: Point,
    direction: Direction,
    direction_count: usize,
}

impl Node {
    fn new(point: Point, direction: Direction, direction_count: usize) -> Self {
        Self {
            point,
            direction,
            direction_count,
        }
    }

    fn neighbors(node: &Node, grid: &Grid) -> Vec<Node> {
        let mut neighbors = Vec::new();
        for (direction, point) in node.point.valid_nexts(grid) {
            if direction == node.direction.opposite() {
                continue;
            } else if direction != node.direction {
                neighbors.push(Node::new(point, direction, 1));
            } else if node.direction_count < 3 {
                neighbors.push(Node::new(point, direction, node.direction_count + 1));
            }
        }
        neighbors
    }
    fn neighbor_part2(node: &Node, grid: &Grid) -> Vec<Node> {
        let mut neighbors = Vec::new();
        for (direction, point) in node.point.valid_nexts(grid) {
            if direction == node.direction.opposite() {
                continue;
            } else if direction != node.direction && node.direction_count >= 4 {
                neighbors.push(Node::new(point, direction, 1));
            } else if direction == node.direction && node.direction_count < 10 {
                neighbors.push(Node::new(point, direction, node.direction_count + 1));
            }
        }
        neighbors
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct State {
    node: Node,
    cost: usize,
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.node.cmp(&other.node)) // reverse order from usual for min heap
    }
}

fn dijkstra<F, G>(grid: &Grid, start: &Point, goal_fn: F, neighbor_fn: G) -> Option<usize>
where
    F: Fn(&Node) -> bool,
    G: Fn(&Node, &Grid) -> Vec<Node>,
{
    let mut distances = HashMap::new();
    //start down and start right
    distances.insert(Node::new(start.clone(), Direction::South, 0), 0);
    distances.insert(Node::new(start.clone(), Direction::East, 0), 0);
    let mut frontier = BinaryHeap::new();
    //the two posibles start states
    frontier.push(State {
        node: Node::new(start.clone(), Direction::South, 0),
        cost: 0,
    });
    frontier.push(State {
        node: Node::new(start.clone(), Direction::East, 0),
        cost: 0,
    });
    while let Some(State { node, cost }) = frontier.pop() {
        if goal_fn(&node) {
            return Some(cost);
        }
        for neighbor in neighbor_fn(&node, grid) {
            let new_cost = cost + grid.grid[neighbor.point.y][neighbor.point.x ];
            if let Some(&best) = distances.get(&neighbor) {
                if new_cost >= best {
                    continue;
                }
            }
            distances.insert(neighbor.clone(), new_cost);
            frontier.push(State {
                node: neighbor,
                cost: new_cost,
            });
        }
    }
    None
}

fn process_part1(input: &str) -> usize {
    let grid = Grid::new(input);
    let start = Point::new(0, 0);
    let goal = Point::new(grid.grid[0].len() - 1, grid.grid.len() - 1);
    let goal_fn = |node: &Node| node.point == goal;
    let neighbor_fn = Node::neighbors;
    dijkstra(&grid, &start, goal_fn, neighbor_fn).unwrap()
}

fn procces_part2(input: &str) -> usize {
    let grid = Grid::new(input);
    let start = Point::new(0, 0);
    let goal = Point::new(grid.grid[0].len() - 1, grid.grid.len() - 1);
    let goal_fn = |node: &Node| node.point == goal && node.direction_count >= 4;
    let neighbor_fn = Node::neighbor_part2;
    dijkstra(&grid, &start, goal_fn, neighbor_fn).unwrap()
}

fn using_pathfinding_part1(input: &str) -> usize {
    let grid = Grid::new(input);
    let start_south = Node::new(Point::new(0, 0), Direction::South, 0);
    let goal = Point::new(grid.grid[0].len() - 1, grid.grid.len() - 1);
    let south_path = pathfinding::directed::dijkstra::dijkstra(
        &start_south,
        |node| {
            Node::neighbors(node, &grid)
                .into_iter()
                .map(|n| (n.clone(), grid.grid[n.point.y][n.point.x]))
        },
        |node| node.point == goal,
    );
    let start_east = Node::new(Point::new(0, 0), Direction::East, 0);
    let east_path = pathfinding::directed::dijkstra::dijkstra(
        &start_east,
        |node| {
            Node::neighbors(node, &grid)
                .into_iter()
                .map(|n| (n.clone(), grid.grid[n.point.y][n.point.x]))
        },
        |node| node.point == goal,
    );
    south_path.unwrap().1.min(east_path.unwrap().1)
}

fn using_pathfinding_part2(input: &str) -> usize {
    let grid = Grid::new(input);
    let start_east = Node::new(Point::new(0, 0), Direction::East, 0);
    let goal = Point::new(grid.grid[0].len() - 1, grid.grid.len() - 1);
    let esast_path = pathfinding::directed::dijkstra::dijkstra(
        &start_east,
        |node| {
            Node::neighbor_part2(node, &grid)
                .into_iter()
                .map(|n| (n.clone(), grid.grid[n.point.y][n.point.x]))
        },
        |node| node.point == goal && node.direction_count >= 4,
    );
    let start_south = Node::new(Point::new(0, 0), Direction::South, 0);
    let south_path = pathfinding::directed::dijkstra::dijkstra(
        &start_south,
        |node| {
            Node::neighbor_part2(node, &grid)
                .into_iter()
                .map(|n| (n.clone(), grid.grid[n.point.y][n.point.x]))
        },
        |node| node.point == goal && node.direction_count >= 4,
    );
    esast_path.unwrap().1.min(south_path.unwrap().1)
}

#[allow(dead_code)]
fn print_path(input: &str) {
    let grid = Grid::new(input);
    let start_south = Node::new(Point::new(0, 0), Direction::South, 0);
    let goal = Point::new(grid.grid[0].len() - 1, grid.grid.len() - 1);
    let south_path = pathfinding::directed::dijkstra::dijkstra(
        &start_south,
        |node| {
            Node::neighbors(node, &grid)
                .into_iter()
                .map(|n| (n.clone(), grid.grid[n.point.y][n.point.x]))
        },
        |node| node.point == goal,
    );
    let start_east = Node::new(Point::new(0, 0), Direction::East, 0);
    let east_path = pathfinding::directed::dijkstra::dijkstra(
        &start_east,
        |node| {
            Node::neighbors(node, &grid)
                .into_iter()
                .map(|n| (n.clone(), grid.grid[n.point.y][n.point.x]))
        },
        |node| node.point == goal,
    );
    let mut grid_ponts_soth_path = vec![vec!['o'; grid.grid[0].len()]; grid.grid.len()];
    let mut grid_ponts_east_path = vec![vec!['o'; grid.grid[0].len()]; grid.grid.len()];
    for node in south_path.unwrap().0 {
        grid_ponts_soth_path[node.point.y][node.point.x] = match node.direction {
            Direction::North => '^',
            Direction::South => 'v',
            Direction::East => '>',
            Direction::West => '<',
        };
    }
    for node in east_path.unwrap().0 {
        grid_ponts_east_path[node.point.y][node.point.x] = match node.direction {
            Direction::North => '^',
            Direction::South => 'v',
            Direction::East => '>',
            Direction::West => '<',
        };
    }
    println!("-------------------------------South path--------------------------------");
    for line in grid_ponts_soth_path {
        let mut line_str = String::new();
        for c in line {
            line_str.push(c);
        }
        println!("{}", line_str);
    }
    println!("-------------------------------East path--------------------------------");
    for line in grid_ponts_east_path {
        let mut line_str = String::new();
        for c in line {
            line_str.push(c);
        }
        println!("{}", line_str);
    }
}

fn main() {
    let input = include_str!("input.txt");
    //print_path(input);
    let now = std::time::Instant::now();
    println!("Part 1: {} in {:#?}", process_part1(input), now.elapsed());
    let now = std::time::Instant::now();
    println!(
        "Part 1 using pathfinding: {} in {:#?}",
        using_pathfinding_part1(input),
        now.elapsed()
    );
    let now = std::time::Instant::now();
    println!("Part 2: {} in {:#?}", procces_part2(input), now.elapsed());
    let now = std::time::Instant::now();
    println!(
        "Part 2 using pathfinding: {} in {:#?}",
        using_pathfinding_part2(input),
        now.elapsed()
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = include_str!("input_test.txt");
        assert_eq!(process_part1(input), 102);
    }
    #[test]
    fn test_part2() {
        let input = include_str!("input_test.txt");
        assert_eq!(procces_part2(input), 94);
    }
}
