use std::{collections::{HashMap, HashSet}, hash::Hash};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Point {
    x: i64,
    y: i64,
}

#[derive(Debug, PartialEq, Eq)]
enum TypeNode {
    NorthSouth,
    EastWest,
    NorthEast,
    NorthWest,
    SouthWest,
    SouthEast,
    Ground,
    Start,
}

#[derive(Debug)]
struct Node {
    point: Point,
    node_type: TypeNode,
}



fn start_equivalence(map:&HashMap<Point,Node>, visitados: &HashSet<Point>, start: Point )->TypeNode{
    let north = Point {
        x: start.x,
        y: start.y - 1,
    };
    let south = Point {
        x: start.x,
        y: start.y + 1,
    };
    let east = Point {
        x: start.x + 1,
        y: start.y,
    };
    let west = Point {
        x: start.x - 1,
        y: start.y,
    };
    let conected_north = visitados.contains(&north) && match map.get(&north) {
        Some(node) => node.node_type == TypeNode::NorthSouth
            || node.node_type == TypeNode::SouthEast
            || node.node_type == TypeNode::SouthWest,
        None => false,
    };
    let conected_south = visitados.contains(&south) && match map.get(&south) {
        Some(node) => node.node_type == TypeNode::NorthSouth
            || node.node_type == TypeNode::NorthEast
            || node.node_type == TypeNode::NorthWest,
        None => false,
    };
    let conected_east = visitados.contains(&east) && match map.get(&east) {
        Some(node) => node.node_type == TypeNode::EastWest
            || node.node_type == TypeNode::NorthWest
            || node.node_type == TypeNode::SouthWest,
        None => false,
    };
    let conected_west = visitados.contains(&west) && match map.get(&west) {
        Some(node) => node.node_type == TypeNode::EastWest
            || node.node_type == TypeNode::NorthEast
            || node.node_type == TypeNode::SouthEast,
        None => false,
    };
    let case = (conected_north, conected_south, conected_east, conected_west);
    match case {
        (true, true, false, false) => TypeNode::NorthSouth,
        (true, false, true, false) => TypeNode::NorthEast,
        (true, false, false, true) => TypeNode::NorthWest,
        (false, true, true, false) => TypeNode::SouthEast,
        (false, true, false, true) => TypeNode::SouthWest,
        (false, false, true, true) => TypeNode::EastWest,
        _ => panic!("No se puede llegar al punto de inicio"),
        
    }
}

fn parse_input(input: &str) -> Result<HashMap<Point, Node>, String> {
    let mut map = HashMap::new();
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let point = Point {
                x: x as i64,
                y: y as i64,
            };
            let node_type = match c {
                '|' => TypeNode::NorthSouth,
                '-' => TypeNode::EastWest,
                'L' => TypeNode::NorthEast,
                'J' => TypeNode::NorthWest,
                '7' => TypeNode::SouthWest,
                'F' => TypeNode::SouthEast,
                '.' => TypeNode::Ground,
                'S' => TypeNode::Start,
                _ => return Err(format!("Invalid character: {}", c)),
            };
            map.insert(point, Node { point, node_type });
        }
    }
    Ok(map)
}

fn process_input(input: &str) -> Result<u64, String> {
    let map = parse_input(input)?;
    let mut current_point = Point { x: 0, y: 0 };
    for node in map.values() {
        if node.node_type == TypeNode::Start {
            current_point = node.point;
            break;
        }
    }
    

    let mut visitados: HashSet<Point> = HashSet::new();
    while !visitados.contains(&current_point) {
        let north = Point {
            x: current_point.x,
            y: current_point.y - 1,
        };
        let south = Point {
            x: current_point.x,
            y: current_point.y + 1,
        };
        let east = Point {
            x: current_point.x + 1,
            y: current_point.y,
        };
        let west = Point {
            x: current_point.x - 1,
            y: current_point.y,
        };
        let next_point = match map.get(&current_point).unwrap().node_type {
            TypeNode::NorthSouth => {
                if !visitados.contains(&north)
                    && (map.get(&north).unwrap().node_type == TypeNode::NorthSouth
                        || map.get(&north).unwrap().node_type == TypeNode::SouthEast
                        || map.get(&north).unwrap().node_type == TypeNode::SouthWest)
                {
                    north
                } else if !visitados.contains(&south)
                    && (map.get(&south).unwrap().node_type == TypeNode::NorthSouth
                        || map.get(&south).unwrap().node_type == TypeNode::NorthEast
                        || map.get(&south).unwrap().node_type == TypeNode::NorthWest)
                {
                    south
                } else {
                    current_point
                }
            }
            TypeNode::EastWest => {
                if !visitados.contains(&east)
                    && (map.get(&east).unwrap().node_type == TypeNode::EastWest
                        || map.get(&east).unwrap().node_type == TypeNode::NorthWest
                        || map.get(&east).unwrap().node_type == TypeNode::SouthWest)
                {
                    east
                } else if !visitados.contains(&west)
                    && (map.get(&west).unwrap().node_type == TypeNode::EastWest
                        || map.get(&west).unwrap().node_type == TypeNode::NorthEast
                        || map.get(&west).unwrap().node_type == TypeNode::SouthEast)
                {
                    west
                } else {
                    current_point
                }
            }
            TypeNode::NorthEast => {
                if !visitados.contains(&north)
                    && (map.get(&north).unwrap().node_type == TypeNode::NorthSouth
                        || map.get(&north).unwrap().node_type == TypeNode::SouthEast
                        || map.get(&north).unwrap().node_type == TypeNode::SouthWest)
                {
                    north
                } else if !visitados.contains(&east)
                    && (map.get(&east).unwrap().node_type == TypeNode::EastWest
                        || map.get(&east).unwrap().node_type == TypeNode::NorthWest
                        || map.get(&east).unwrap().node_type == TypeNode::SouthWest)
                {
                    east
                } else {
                    current_point
                }
            }
            TypeNode::NorthWest => {
                if !visitados.contains(&north)
                    && (map.get(&north).unwrap().node_type == TypeNode::NorthSouth
                        || map.get(&north).unwrap().node_type == TypeNode::SouthEast
                        || map.get(&north).unwrap().node_type == TypeNode::SouthWest)
                {
                    north
                } else if !visitados.contains(&west)
                    && (map.get(&west).unwrap().node_type == TypeNode::EastWest
                        || map.get(&west).unwrap().node_type == TypeNode::NorthEast
                        || map.get(&west).unwrap().node_type == TypeNode::SouthEast)
                {
                    west
                } else {
                    current_point
                }
            }
            TypeNode::SouthWest => {
                if !visitados.contains(&south)
                    && (map.get(&south).unwrap().node_type == TypeNode::NorthSouth
                        || map.get(&south).unwrap().node_type == TypeNode::NorthEast
                        || map.get(&south).unwrap().node_type == TypeNode::NorthWest)
                {
                    south
                } else if !visitados.contains(&west)
                    && (map.get(&west).unwrap().node_type == TypeNode::EastWest
                        || map.get(&west).unwrap().node_type == TypeNode::NorthEast
                        || map.get(&west).unwrap().node_type == TypeNode::SouthEast)
                {
                    west
                } else {
                    current_point
                }
            }

            TypeNode::SouthEast => {
                if !visitados.contains(&south)
                    && (map.get(&south).unwrap().node_type == TypeNode::NorthSouth
                        || map.get(&south).unwrap().node_type == TypeNode::NorthEast
                        || map.get(&south).unwrap().node_type == TypeNode::NorthWest)
                {
                    south
                } else if !visitados.contains(&east)
                    && (map.get(&east).unwrap().node_type == TypeNode::EastWest
                        || map.get(&east).unwrap().node_type == TypeNode::NorthWest
                        || map.get(&east).unwrap().node_type == TypeNode::SouthWest)
                {
                    east
                } else {
                    current_point
                }
            }
            TypeNode::Ground => {
                return Err(format!("Ground at {:?}", current_point));
            }
            TypeNode::Start => {
                if !visitados.contains(&north)
                    && match map.get(&north) {
                        Some(node) => node.node_type == TypeNode::NorthSouth
                            || node.node_type == TypeNode::SouthEast
                            || node.node_type == TypeNode::SouthWest,
                        None => false,
                    }
                {
                    north
                } else if !visitados.contains(&south)
                    && match map.get(&south) {
                        Some(node) => node.node_type == TypeNode::NorthSouth
                            || node.node_type == TypeNode::NorthEast
                            || node.node_type == TypeNode::NorthWest,
                        None => false,
                    }
                {
                    south
                } else if !visitados.contains(&east)
                    && match map.get(&east) {
                        Some(node) => node.node_type == TypeNode::EastWest
                            || node.node_type == TypeNode::NorthWest
                            || node.node_type == TypeNode::SouthWest,
                        None => false,
                        
                    }
                {
                    east
                } else if !visitados.contains(&west)
                    && match map.get(&west) {
                        Some(node) => node.node_type == TypeNode::EastWest
                            || node.node_type == TypeNode::NorthEast
                            || node.node_type == TypeNode::SouthEast,
                        None => false,
                        
                    }
                {
                    west
                } else {
                    return Err(format!("No path at {:?}", current_point));
                }
            }
        };

        visitados.insert(current_point);
        current_point = next_point;
    }
    
    

    let mut interno = HashSet::new();
    let max_x = map.keys().max_by_key(|p| p.x).unwrap().x;
    let max_y = map.keys().max_by_key(|p| p.y).unwrap().y;
    for x in 0..=max_x {
        let mut letf = 0;
        let mut right = 0;
        for y in 0..=max_y {
            match visitados.contains(&Point { x, y }) {
                true => match map.get(&Point { x, y }).unwrap().node_type {
                    TypeNode::EastWest => {
                        letf += 1;
                        right += 1;
                    }
                    TypeNode::NorthEast => {
                        right += 1;
                    }
                    TypeNode::NorthWest => {
                        letf += 1;
                    }
                    TypeNode::SouthWest => {
                        letf += 1;
                    }
                    TypeNode::SouthEast => {
                        right += 1;
                    }

                    TypeNode::Start => {
                        
                        match start_equivalence(&map, &visitados, Point { x, y }) {
                            TypeNode::EastWest => {
                                letf += 1;
                                right += 1;
                            }
                            TypeNode::NorthEast => {
                                right += 1;
                            }
                            TypeNode::NorthWest => {
                                letf += 1;
                            }
                            TypeNode::SouthWest => {
                                letf += 1;
                            }
                            TypeNode::SouthEast => {
                                right += 1;
                            }
                            _ => {}
                        }
                    }
                    _ => {}
                },

                false => {
                                        
                    if letf.min(right) % 2 == 1 {
                        interno.insert(Point { x, y });
                    }
                }
            }
        }
    }
    Ok(interno.len() as u64) //siempre hay un ciclo
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
        let input = "...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........";
        let result = process_input(input).unwrap();
        assert_eq!(result, 4);
    }
    #[test]
    fn test_process_input2() {
        let input = ".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...";
        let result = process_input(input).unwrap();
        assert_eq!(result, 8);
    }
    #[test]
    fn test_process_input3() {
        let input = "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.";
        let result = process_input(input).unwrap();
        assert_eq!(result, 10);
    }
}
