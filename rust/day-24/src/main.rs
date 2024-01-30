use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{self, line_ending, space1},
    multi::separated_list1,
    sequence::{delimited, separated_pair, terminated},
    IResult, Parser,
};

use nalgebra::{Matrix6x1, Matrix6, RowVector6};
#[derive(Debug)]
enum Intersection {
    Point(Point),
    All,
}
#[derive(Debug)]
struct Point {
    x: i64,
    y: i64,
    z: i64,
}
#[derive(Debug)]
struct Velocity {
    x: i64,
    y: i64,
    z: i64,
}

#[derive(Debug)]
struct Hail {
    position: Point,
    velocity: Velocity,
}

impl Hail {
    fn intersection_xy(&self, other: &Hail) -> Option<Intersection> {
        let slope_self = self.velocity.y as f64 / self.velocity.x as f64;
        let slope_other = other.velocity.y as f64 / other.velocity.x as f64;
        let intersept_self = self.position.y as f64 - slope_self * self.position.x as f64;
        let intersept_other = other.position.y as f64 - slope_other * other.position.x as f64;
        if slope_self == slope_other && intersept_self == intersept_other {
            return Some(Intersection::All);
        } else if slope_self == slope_other {
            return None;
        }
        let x = (intersept_other - intersept_self) / (slope_self - slope_other);
        let y = slope_self * x + intersept_self;
        Some(Intersection::Point(Point {
            x: x as i64,
            y: y as i64,
            z: 0,
        }))
    }

    fn intersects_in_the_past_xy(&self, point: &Point) -> bool {
        let x = point.x - self.position.x;
        let y = point.y - self.position.y;
        let x = x as f64 / self.velocity.x as f64;
        let y = y as f64 / self.velocity.y as f64;
        x < 0.0 && y < 0.0
    }
}

fn point(input: &str) -> IResult<&str, Point> {
    let (input, x) = complete::i64(input)?;
    let (input, _) = terminated(tag(","), space1)(input)?;
    let (input, y) = complete::i64(input)?;
    let (input, _) = terminated(tag(","), space1)(input)?;
    let (input, z) = complete::i64(input)?;
    Ok((input, Point { x, y, z }))
}

fn velocity(input: &str) -> IResult<&str, Velocity> {
    let (input, x) = complete::i64(input)?;
    let (input, _) = terminated(tag(","), space1)(input)?;
    let (input, y) = complete::i64(input)?;
    let (input, _) = terminated(tag(","), space1)(input)?;
    let (input, z) = complete::i64(input)?;
    Ok((input, Velocity { x, y, z }))
}

fn parse(input: &str) -> IResult<&str, Vec<Hail>> {
    separated_list1(
        line_ending,
        separated_pair(point, delimited(space1, tag("@"), space1), velocity)
            .map(|(position, velocity)| Hail { position, velocity }),
    )(input)
}

fn solve_part1(input: &str, min: i64, max: i64) -> usize {
    let (_, hails) = parse(input).unwrap();

    hails
        .iter()
        .tuple_combinations()
        .filter_map(|(hail1, hail2)| {
            let intersection = hail1.intersection_xy(hail2);
            match intersection {
                Some(Intersection::Point(ref point)) => {
                    //println!("{:?} {:?} {:?}", hail1, hail2, point);
                    if !hail1.intersects_in_the_past_xy(point)
                        && !hail2.intersects_in_the_past_xy(point)
                        && point.x >= min
                        && point.x <= max
                        && point.y >= min
                        && point.y <= max
                    {
                        Some(intersection)
                    } else {
                        None
                    }
                }
                Some(Intersection::All) => Some(intersection),
                None => None,
            }
        })
        .count()
}

fn solve_part2(input: &str)-> i128{
    let (_, hails) = parse(input).unwrap();
    let p0 = &hails[0].position;
    let p1 = &hails[1].position;
    let p2 = &hails[2].position;
    let v0 = &hails[0].velocity;
    let v1 = &hails[1].velocity;
    let v2 = &hails[2].velocity;

    let b = Matrix6x1::from_row_slice(&[
        ((p0.y as i128 * v0.x as i128 - p1.y as i128 * v1.x as i128)
            - (p0.x as i128 * v0.y as i128 - p1.x as i128 * v1.y as i128)) as f64,
        ((p0.y as i128 * v0.x as i128 - p2.y as i128 * v2.x as i128)
            - (p0.x as i128 * v0.y as i128 - p2.x as i128 * v2.y as i128)) as f64,
        ((p0.z as i128 * v0.x as i128 - p1.z as i128 * v1.x as i128)
            - (p0.x as i128 * v0.z as i128 - p1.x as i128 * v1.z as i128)) as f64,
        ((p0.z as i128 * v0.x as i128 - p2.z as i128 * v2.x as i128)
            - (p0.x as i128 * v0.z as i128 - p2.x as i128 * v2.z as i128)) as f64,
        ((p0.z as i128 * v0.y as i128 - p1.z as i128 * v1.y as i128)
            - (p0.y as i128 * v0.z as i128 - p1.y as i128 * v1.z as i128)) as f64,
        ((p0.z as i128 * v0.y as i128 - p2.z as i128 * v2.y as i128)
            - (p0.y as i128 * v0.z as i128 - p2.y as i128 * v2.z as i128)) as f64,
    ]);

    let a = Matrix6::from_rows(&[
        RowVector6::new(v1.y as f64 - v0.y as f64, v0.x as f64  - v1.x as f64, 0.0, p0.y as f64 - p1.y as f64, p1.x as f64 - p0.x as f64, 0.0),
        RowVector6::new(v2.y as f64 - v0.y as f64, v0.x as f64 - v2.x as f64, 0.0, p0.y as f64 - p2.y as f64, p2.x as f64 - p0.x as f64, 0.0),
        RowVector6::new(v1.z as f64 - v0.z as f64, 0.0, v0.x as f64 - v1.x as f64, p0.z as f64 - p1.z as f64, 0.0, p1.x as f64 - p0.x as f64),
        RowVector6::new(v2.z as f64 - v0.z as f64, 0.0, v0.x as f64 - v2.x as f64, p0.z as f64 - p2.z as f64, 0.0, p2.x as f64 - p0.x as f64),
        RowVector6::new(0.0, v1.z as f64 - v0.z as f64, v0.y as f64 - v1.y as f64, 0.0, p0.z as f64 - p1.z as f64, p1.y as f64 - p0.y as f64),
        RowVector6::new(0.0, v2.z as f64 - v0.z as f64, v0.y as f64 - v2.y as f64, 0.0, p0.z as f64 - p2.z as f64, p2.y as f64 - p0.y as f64),
    ]);

    let r = a.lu().solve(&b).unwrap();
    r[0] as i128 + r[1] as i128 + r[2] as i128
    

}


fn main() {
    let input = include_str!("input.txt");
    let now = std::time::Instant::now();
    println!("Part 1: {} in : {:?}", solve_part1(input, 200000000000000, 400000000000000), now.elapsed());
    println!("Part 2: {} in : {:?}", solve_part2(input), now.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let input = include_str!("input_test.txt");
        assert_eq!(solve_part1(input, 7, 27), 2);
    }
}
