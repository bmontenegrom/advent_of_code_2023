use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, digit1},
    combinator::map,
    multi::separated_list1,
    sequence::preceded,
    IResult,
};

#[derive(Debug)]
enum Operation {
    Remove,
    Add(usize),
}

#[derive(Debug)]
struct Step<'a> {
    label: &'a str,
    operation: Operation,
}
impl<'a> Step<'a> {
    fn new(label: &'a str, operation: Operation) -> Self {
        Step { label, operation }
    }
}

#[derive(Debug, Clone)]
struct Lens<'a> {
    label: &'a str,
    focal_length: usize,
}
impl<'a> Lens<'a> {
    fn new(label: &'a str, focal_length: usize) -> Self {
        Lens {
            label,
            focal_length,
        }
    }
    fn hash(&self) -> usize {
        self.label
            .chars()
            .fold(0, |acc, next_char| (acc + next_char as usize) * 17 % 256)
    }
}
impl<'a> PartialEq for Lens<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.label == other.label
    }
}

#[derive(Debug)]
struct Boxes<'a> {
    boxes: Vec<Vec<Lens<'a>>>,
}
impl<'a> Boxes<'a> {
    fn new() -> Self {
        Boxes {
            boxes: vec![vec![]; 256],
        }
    }

    fn add_lens(&mut self, lens_to_add: Lens<'a>) {
        for lens in self.boxes[lens_to_add.hash()].iter_mut() {
            if lens == &lens_to_add {
                lens.focal_length = lens_to_add.focal_length;
                return;
            }
        }
        self.boxes[lens_to_add.hash()].push(lens_to_add);
    }

    fn remove_lens(&mut self, lens_to_remove: Lens<'a>) {
        self.boxes[lens_to_remove.hash()].retain(|lens| lens != &lens_to_remove);
    }

    fn do_step(&mut self, step: Step<'a>) {
        match step.operation {
            Operation::Add(focal_length) => {
                self.add_lens(Lens::new(step.label, focal_length));
            }
            Operation::Remove => {
                self.remove_lens(Lens::new(step.label, 0));
            }
        }
    }
    fn do_steps(&mut self, steps: Vec<Step<'a>>) {
        for step in steps {
            self.do_step(step);
        }
    }
    fn focusion_power(&self) -> usize {
        self.boxes
            .iter()
            .enumerate()
            .map(|(box_number, lenses)| {
                lenses
                    .iter()
                    .enumerate()
                    .map(|(lens_number, lens)| {
                        (box_number + 1) * (lens_number + 1) * lens.focal_length
                    })
                    .sum::<usize>()
            })
            .sum()
    }
}
fn step(input: &str) -> IResult<&str, Step> {
    let (input, label) = alpha1(input)?;
    let (input, operation) = alt((
        map(tag("-"), |_| Operation::Remove),
        map(preceded(tag("="), digit1), |s: &str| {
            Operation::Add(s.parse().unwrap())
        }),
    ))(input)?;
    Ok((input, Step::new(label, operation)))
}

fn parse_input_part2(input: &str) -> Vec<Step> {
    separated_list1(tag(","), step)(input).unwrap().1
}

fn parse_input_part1(input: &str) -> Vec<&str> {
    input.split(',').collect()
}

fn solve_part1(input: &str) -> usize {
    let to_hash = parse_input_part1(input);

    to_hash
        .iter()
        .map(|string| {
            string
                .chars()
                .fold(0, |acc, next_char| (acc + next_char as usize) * 17 % 256)
        })
        .sum()
}

fn solve_part2(input: &str) -> usize {
    let mut boxes = Boxes::new();
    let steps = parse_input_part2(input);
    boxes.do_steps(steps);
    boxes.focusion_power()
}
fn main() {
    let input = include_str!("input.txt");
    let now = std::time::Instant::now();
    println!("Part 1: {} in : {:?}", solve_part1(input), now.elapsed());
    let now = std::time::Instant::now();
    println!("Part 2: {} in : {:?}", solve_part2(input), now.elapsed());
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_parte1() {
        let input = include_str!("input_test.txt");
        let result = solve_part1(input);
        assert_eq!(result, 1320);
    }
    #[test]
    fn test_2() {
        let input = include_str!("input.txt");
        let result = solve_part1(input);
        assert_eq!(result, 518107);
    }
    #[test]
    fn test_part2() {
        let input = include_str!("input_test.txt");
        let result = solve_part2(input);
        assert_eq!(result, 145);
    }
}
