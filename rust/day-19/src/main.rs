use std::collections::HashMap;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, digit1, line_ending, multispace1, one_of},
    combinator::map,
    multi::separated_list1,
    sequence::tuple,
    IResult,
};

#[derive(Debug)]
struct Part {
    x: isize,
    m: isize,
    a: isize,
    s: isize,
}
impl Part {
    fn parse(input: &str) -> IResult<&str, Part> {
        let (input, _) = tag("{x=")(input)?;
        let (input, x) = digit1(input)?;
        let x = x.parse().unwrap();
        let (input, _) = tag(",m=")(input)?;
        let (input, m) = digit1(input)?;
        let m = m.parse().unwrap();
        let (input, _) = tag(",a=")(input)?;
        let (input, a) = digit1(input)?;
        let a = a.parse().unwrap();
        let (input, _) = tag(",s=")(input)?;
        let (input, s) = digit1(input)?;
        let s = s.parse().unwrap();
        let (input, _) = tag("}")(input)?;
        Ok((input, Part { x, m, a, s }))
    }
    fn set(&mut self, var: char, val: isize) {
        match var {
            'x' => self.x = val,
            'm' => self.m = val,
            'a' => self.a = val,
            's' => self.s = val,
            _ => unreachable!(),
        }
    }
    fn get_value(&self, var: char) -> isize {
        match var {
            'x' => self.x,
            'm' => self.m,
            'a' => self.a,
            's' => self.s,
            _ => panic!("Unknown variable {}", var),
        }
    }
    fn rating(&self) -> isize {
        self.x + self.m + self.a + self.s
    }

    fn process(&self, workflows: &HashMap<String, Workflow>) -> Destination {
        let mut current_workflow = workflows.get("in").unwrap(); //start
        loop {
            match current_workflow.evaluate(self) {
                Destination::Accept => return Destination::Accept,
                Destination::Reject => return Destination::Reject,
                Destination::Workflow(name) => {
                    current_workflow = workflows.get(&name).unwrap();
                }
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Condition {
    LessThan(char, isize),
    GreaterThan(char, isize),
    LessThanEqual(char, isize),
    GreaterThanEqual(char, isize),
}

impl Condition {
    fn parse(input: &str) -> IResult<&str, Condition> {
        let (input, (var, op, val)) = tuple(((one_of("xmsa")), one_of("<>"), digit1))(input)?;
        let val = val.parse().unwrap();
        let cond = match op {
            '<' => Condition::LessThan(var, val),
            '>' => Condition::GreaterThan(var, val),
            _ => panic!("Unknown operator {}", op),
        };
        Ok((input, cond))
    }
    fn evaluate(&self, part: &Part) -> bool {
        match self {
            Condition::LessThan(var, val) => part.get_value(*var) < *val,
            Condition::GreaterThan(var, val) => part.get_value(*var) > *val,
            Condition::LessThanEqual(var, val) => part.get_value(*var) <= *val,
            Condition::GreaterThanEqual(var, val) => part.get_value(*var) >= *val,
        }
    }

    fn opposite(&self) -> Condition {
        match self {
            Condition::LessThan(var, val) => Condition::GreaterThanEqual(*var, *val),
            Condition::GreaterThan(var, val) => Condition::LessThanEqual(*var, *val),
            Condition::LessThanEqual(var, val) => Condition::GreaterThan(*var, *val),
            Condition::GreaterThanEqual(var, val) => Condition::LessThan(*var, *val),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Destination {
    Accept,
    Reject,
    Workflow(String),
}

impl Destination {
    fn parse(input: &str) -> IResult<&str, Destination> {
        let (input, destination) = alt((tag("A"), tag("R"), alpha1))(input)?;
        let destination = match destination {
            "A" => Destination::Accept,
            "R" => Destination::Reject,
            s => Destination::Workflow(s.to_string()),
        };
        Ok((input, destination))
    }
}

#[derive(Debug)]
enum Rule {
    Evaluation(Condition, Destination),
    Fallthrough(Destination),
}

impl Rule {
    fn parse(input: &str) -> IResult<&str, Rule> {
        alt((
            map(
                tuple((Condition::parse, tag(":"), Destination::parse)),
                |(c, _, d)| Rule::Evaluation(c, d),
            ),
            map(Destination::parse, Rule::Fallthrough),
        ))(input)
    }
    fn evaluate(&self, part: &Part) -> Option<Destination> {
        match self {
            Rule::Evaluation(cond, dest) => {
                if cond.evaluate(part) {
                    Some(dest.clone())
                } else {
                    None
                }
            }
            Rule::Fallthrough(dest) => Some(dest.clone()),
        }
    }
}

#[derive(Debug)]
struct Workflow {
    name: String,
    rules: Vec<Rule>,
}
impl Workflow {
    fn parse(input: &str) -> IResult<&str, Workflow> {
        let (input, name) = alpha1(input)?;
        let name = name.to_string();
        let (input, _) = tag("{")(input)?;
        let (input, rules) = separated_list1(tag(","), Rule::parse)(input)?;
        let (input, _) = tag("}")(input)?;
        Ok((input, Workflow { name, rules }))
    }

    fn evaluate(&self, part: &Part) -> Destination {
        self.rules.iter().find_map(|r| r.evaluate(part)).unwrap()
    }
}

fn workflows(input: &str) -> IResult<&str, HashMap<String, Workflow>> {
    let (input, workflows) = separated_list1(line_ending, Workflow::parse)(input)?;
    Ok((
        input,
        workflows.into_iter().map(|w| (w.name.clone(), w)).collect(),
    ))
}

fn parts(input: &str) -> IResult<&str, Vec<Part>> {
    separated_list1(line_ending, Part::parse)(input)
}

fn parse_input(input: &str) -> IResult<&str, (Vec<Part>, HashMap<String, Workflow>)> {
    let (input, workflows) = workflows(input)?;
    let (input, _) = multispace1(input)?;
    let (input, parts) = parts(input)?;
    Ok((input, (parts, workflows)))
}

fn solve_part1(input: &str) -> isize {
    let (_, (parts, workflows)) = parse_input(input).unwrap();
    parts
        .iter()
        .filter(|p| p.process(&workflows) == Destination::Accept)
        .map(|p| p.rating())
        .sum()
}

fn generate_paths(
    workflows: &HashMap<String, Workflow>,
    current: &str,
    paretns: &[Condition],
) -> Vec<Vec<Condition>> {
    let mut paths = Vec::new();
    let workflow = workflows.get(current).unwrap();
    let mut previus_condition = Vec::new();
    for rule in &workflow.rules {
        let mut new_parents = paretns.to_vec();
        new_parents.extend(previus_condition.clone());
        match rule {
            Rule::Evaluation(cond, dest) => {
                new_parents.push(*cond);
                previus_condition.push(cond.opposite());
                match dest {
                    Destination::Accept => {
                        paths.push(new_parents);
                    }
                    Destination::Reject => {}
                    Destination::Workflow(name) => {
                        paths.extend(generate_paths(workflows, name, &new_parents));
                    }
                }
            }
            Rule::Fallthrough(dest) => match dest {
                Destination::Accept => {
                    paths.push(new_parents);
                }
                Destination::Reject => {}
                Destination::Workflow(name) => {
                    paths.extend(generate_paths(workflows, name, &new_parents));
                }
            },
        }
    }

    paths
}

fn calculate_possible_combinations(path: &[Condition]) -> isize {
    // Track our minimum and maximum values.
    let mut min_part = Part {
        x: 1,
        m: 1,
        a: 1,
        s: 1,
    };
    let mut max_part = Part {
        x: 4000,
        m: 4000,
        a: 4000,
        s: 4000,
    };

    for condition in path {
        match condition {
            Condition::LessThan(var, val) => {
                max_part.set(*var, max_part.get_value(*var).min(val - 1));
            }
            Condition::LessThanEqual(var, val) => {
                max_part.set(*var, max_part.get_value(*var).min(*val));
            }
            Condition::GreaterThan(var, val) => {
                min_part.set(*var, min_part.get_value(*var).max(val + 1));
            }
            Condition::GreaterThanEqual(var, val) => {
                min_part.set(*var, min_part.get_value(*var).max(*val));
            }
        }
    }

    (max_part.x - min_part.x + 1)
        * (max_part.m - min_part.m + 1)
        * (max_part.a - min_part.a + 1)
        * (max_part.s - min_part.s + 1)
}

fn solve_part2(input: &str) -> isize {
    let (_, (_, workflows)) = parse_input(input).unwrap();
    let paths = generate_paths(&workflows, "in", &[]);
    paths
        .iter()
        .map(|p| calculate_possible_combinations(p))
        .sum()
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
        assert_eq!(solve_part1(input), 19114);
    }

    #[test]
    fn test_part2() {
        let input = include_str!("input_test.txt");
        assert_eq!(solve_part2(input), 167409079868000);
    }
}
