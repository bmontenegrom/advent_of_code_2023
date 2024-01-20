use std::collections::{HashMap, HashSet};




#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Point {
    x: i64,
    y: i64,
}

#[derive(Debug, PartialEq, Eq)]
enum TypeNode{
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
struct Node{
    point: Point,
    node_type: TypeNode,
}



fn parse_input(input: &str)-> Result<HashMap<Point, Node>, String>{
    let mut map = HashMap::new();
    for (y, line) in input.lines().enumerate(){
        for (x, c) in line.chars().enumerate(){
            let point = Point{x: x as i64, y: y as i64};
            let node_type = match c{
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
            map.insert(point, Node{point, node_type});
        }
    }
    Ok(map)
}

fn process_input(input: &str)-> Result<u64, String>{
    let map = parse_input(input)?;
    let mut current_point = Point{x: 0, y: 0};
    for node in map.values(){
        if node.node_type == TypeNode::Start{
            current_point = node.point;
            break;
        }
    }

    let mut visitados: HashSet<Point> = HashSet::new();
    while !visitados.contains(&current_point) {
        let north = Point{x: current_point.x, y: current_point.y - 1};
        let south = Point{x: current_point.x, y: current_point.y + 1};
        let east = Point{x: current_point.x + 1, y: current_point.y};
        let west = Point{x: current_point.x - 1, y: current_point.y};
        let next_point =  match map.get(&current_point).unwrap().node_type {
            TypeNode::NorthSouth => {
                if !visitados.contains(&north) && (map.get(&north).unwrap().node_type == TypeNode::NorthSouth || map.get(&north).unwrap().node_type == TypeNode::SouthEast || map.get(&north).unwrap().node_type == TypeNode::SouthWest){ 
                    north
                } else if !visitados.contains(&south) && (map.get(&south).unwrap().node_type == TypeNode::NorthSouth || map.get(&south).unwrap().node_type == TypeNode::NorthEast || map.get(&south).unwrap().node_type == TypeNode::NorthWest){
                    south
                } else {
                    current_point
                }
            },
            TypeNode::EastWest => {
                if !visitados.contains(&east) && (map.get(&east).unwrap().node_type == TypeNode::EastWest || map.get(&east).unwrap().node_type == TypeNode::NorthWest || map.get(&east).unwrap().node_type == TypeNode::SouthWest){ 
                    east
                } else if !visitados.contains(&west) && (map.get(&west).unwrap().node_type == TypeNode::EastWest || map.get(&west).unwrap().node_type == TypeNode::NorthEast || map.get(&west).unwrap().node_type == TypeNode::SouthEast){
                    west
                } else {
                    current_point
                }
            },
            TypeNode::NorthEast => {
                if !visitados.contains(&north) && (map.get(&north).unwrap().node_type == TypeNode::NorthSouth || map.get(&north).unwrap().node_type == TypeNode::SouthEast || map.get(&north).unwrap().node_type == TypeNode::SouthWest){ 
                    north
                } else if !visitados.contains(&east) && (map.get(&east).unwrap().node_type == TypeNode::EastWest || map.get(&east).unwrap().node_type == TypeNode::NorthWest || map.get(&east).unwrap().node_type == TypeNode::SouthWest){
                    east
                } else {
                    current_point
                }
            },
            TypeNode::NorthWest => {
                if !visitados.contains(&north) && (map.get(&north).unwrap().node_type == TypeNode::NorthSouth || map.get(&north).unwrap().node_type == TypeNode::SouthEast || map.get(&north).unwrap().node_type == TypeNode::SouthWest){ 
                    north
                } else if !visitados.contains(&west) && (map.get(&west).unwrap().node_type == TypeNode::EastWest || map.get(&west).unwrap().node_type == TypeNode::NorthEast || map.get(&west).unwrap().node_type == TypeNode::SouthEast){
                    west
                } else {
                    current_point
                }
            },
            TypeNode::SouthWest => {
                if !visitados.contains(&south) && (map.get(&south).unwrap().node_type == TypeNode::NorthSouth || map.get(&south).unwrap().node_type == TypeNode::NorthEast || map.get(&south).unwrap().node_type == TypeNode::NorthWest){ 
                    south
                } else if !visitados.contains(&west) && (map.get(&west).unwrap().node_type == TypeNode::EastWest || map.get(&west).unwrap().node_type == TypeNode::NorthEast || map.get(&west).unwrap().node_type == TypeNode::SouthEast){
                    west
                } else {
                    current_point
                }
            },
            
            TypeNode::SouthEast => {
                if !visitados.contains(&south) && (map.get(&south).unwrap().node_type == TypeNode::NorthSouth || map.get(&south).unwrap().node_type == TypeNode::NorthEast || map.get(&south).unwrap().node_type == TypeNode::NorthWest){ 
                    south
                } else if !visitados.contains(&east) && (map.get(&east).unwrap().node_type == TypeNode::EastWest || map.get(&east).unwrap().node_type == TypeNode::NorthWest || map.get(&east).unwrap().node_type == TypeNode::SouthWest){
                    east
                } else {
                    current_point
                }
            },
            TypeNode::Ground => {
                return Err(format!("Ground at {:?}", current_point));
            },
            TypeNode::Start => {
                if !visitados.contains(&north)&& map.get(&north).unwrap().node_type != TypeNode::Ground{
                    north
                }
                else if !visitados.contains(&south)&& map.get(&south).unwrap().node_type != TypeNode::Ground{
                    south
                }
                else if !visitados.contains(&east)&& map.get(&east).unwrap().node_type != TypeNode::Ground{
                    east
                }
                else if !visitados.contains(&west)&& map.get(&west).unwrap().node_type != TypeNode::Ground{
                    west
                }
                else {
                    return Err(format!("No path at {:?}", current_point));
                }
            },
        };
       
        visitados.insert(current_point);
        current_point = next_point;
            
        };
        let largo_ciclo = visitados.len() as u64;
        Ok(largo_ciclo/2 + largo_ciclo%2)//siempre hay un ciclo

    }




fn main(){
    let input = include_str!("input.txt");
    println!("{:?}", process_input(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_input() {
        let input = ".....
.S-7.
.|.|.
.L-J.
.....";
        let result = process_input(input).unwrap();
        assert_eq!(result, 4);
    }
    #[test]
    fn test_process_input2() {
        let input = "..F7.
.FJ|.
SJ.L7
|F--J
LJ...";
        let result = process_input(input).unwrap();
        assert_eq!(result, 8);
    }

}