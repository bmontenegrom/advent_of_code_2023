use std::ops::RangeInclusive;

use nom::{
    bytes::complete::tag,
    character::complete::{self, alpha1, newline, space1},
    multi::{many1, separated_list1},
    sequence::{pair, preceded, separated_pair, terminated, tuple},
    IResult,
};

#[derive(Debug, PartialEq, Clone)]
struct RangeMap<'a> {
    source: &'a str,
    destination: &'a str,
    data: Vec<(RangeInclusive<u64>, u64)>,
}

impl RangeMap<'_> {
    fn new<'a>(source: &'a str, destination: &'a str) -> RangeMap<'a> {
        RangeMap {
            source,
            destination,
            data: Vec::new(),
        }
    }

    fn insert(&mut self, range: RangeInclusive<u64>, value: u64) {
        self.data.push((range, value));
    }

    fn get(&self, index_range: RangeInclusive<u64>) -> Vec<RangeInclusive<u64>> {
        if let Some((range, value)) = self
            .data
            .iter()
            .find(|(range, _)| range.contains(index_range.start()))
        {
            if index_range.end() <= range.end() {
                let start = range.start();
                vec![(value + index_range.start() - start)..=(value + index_range.end() - start)]
            } else {
                let start = range.start();
                let mut result =
                    vec![(value + index_range.start() - start)..=(value + range.end() - start)];
                result.append(&mut self.get((*range.end() + 1)..=*index_range.end()));
                result
            }
        } else {
            match self.data.iter().find(|(range, _)| {
                index_range.start() < range.start() && range.start() < index_range.end()
            }) {
                Some((range, _)) => {
                    let mut retval = vec![*index_range.start()..=(range.start() - 1)];
                    retval.append(&mut self.get(*range.start()..=*index_range.end()));
                    retval
                }
                None => {
                    vec![index_range]
                }
            }
        }
    }
}

//seeds: 79 14 55 13
fn seeds(input: &str) -> IResult<&str, Vec<RangeInclusive<u64>>> {
    let (input, ranges) = preceded(
        tag("seeds: "),
        separated_list1(space1, separated_pair(complete::u64, space1, complete::u64)),
    )(input)?;
    Ok((
        input,
        ranges
            .into_iter()
            .map(|(start, length)| start..=(start + length - 1))
            .collect(),
    ))
}

//seed-to-soil map:
fn range_map_title(input: &str) -> IResult<&str, (&str, &str)> {
    terminated(
        separated_pair(alpha1, tag("-to-"), alpha1),
        pair(space1, tag("map:")),
    )(input)
}

//50 98 2
fn inclusive_range_value(input: &str) -> IResult<&str, (RangeInclusive<u64>, u64)> {
    let (input, (valor, min, largo)) = tuple((
        terminated(complete::u64, space1),
        terminated(complete::u64, space1),
        terminated(complete::u64, newline),
    ))(input)?;
    Ok((input, (min..=min + largo - 1, valor)))
}

fn range_map(input: &str) -> IResult<&str, RangeMap> {
    let (input, ((source, destination), rango_valor)) =
        separated_pair(range_map_title, newline, many1(inclusive_range_value))(input)?;
    let mut range_map = RangeMap::new(source, destination);
    for (range, value) in rango_valor {
        range_map.insert(range, value);
    }
    Ok((input, range_map))
}

fn parse_input(input: &str) -> IResult<&str, (Vec<RangeInclusive<u64>>, Vec<RangeMap>)> {
    separated_pair(
        seeds,
        pair(newline, newline),
        separated_list1(newline, range_map),
    )(input)
}

fn follow_map<'a>(
    ranges: Vec<RangeInclusive<u64>>,
    destination: &'a str,
    range_maps: Vec<RangeMap<'a>>,
) -> (Vec<RangeInclusive<u64>>, &'a str) {
    if let Some(range_map) = range_maps
        .iter()
        .find(|range_map| range_map.source == destination)
    {
        let new_ranges = ranges
            .iter()
            .flat_map(|range| range_map.get(range.clone()))
            .collect();
        let new_destination = range_map.destination;
        follow_map(new_ranges, new_destination, range_maps)
    } else {
        (ranges, destination)
    }
}

pub fn process_input(input: &str) -> Result<u64, String> {
    let (_, (seeds, range_maps)) = parse_input(input).expect("Should Parse");
    let result = *seeds
        .into_iter()
        .flat_map(|value| follow_map(vec![value], "seed", range_maps.clone()).0)
        .min_by(|a, b| a.start().cmp(b.start()))
        .unwrap()
        .start();
    Ok(result)
}
fn main() {
    let input = include_str!("input1.txt");
    println!("{:?}", process_input(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_seeds() {
        let result = seeds("seeds: 79 14 55 13").expect("Should parse");
        assert_eq!(result.0, "");
        assert_eq!(result.1, vec![79..=92, 55..=67]);
    }

    #[test]
    fn test_range_map_title() {
        assert_eq!(
            range_map_title("seed-to-soil map:"),
            Ok(("", ("seed", "soil")))
        );
    }

    #[test]
    fn test_inclusive_range_value() {
        assert_eq!(inclusive_range_value("50 98 2\n"), Ok(("", (98..=99, 50))));
    }

    #[test]
    fn test_process_input() {
        let input = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

        assert_eq!(process_input(input), Ok(46));
    }
}
