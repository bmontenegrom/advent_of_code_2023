use nom::{
    bytes::complete::is_not,
    character::complete::{self, line_ending, space1},
    multi::separated_list1,
    sequence::{preceded, separated_pair},
    IResult,
};

//Time:        56     71     79     99
fn numeros(input: &str) -> IResult<&str, Vec<u32>> {
    preceded(is_not("0123456789"), separated_list1(space1, complete::u32))(input)
}

//Time:        56     71     79     99
//Distance:   334   1135   1350   2430
fn parse_input(input: &str) -> IResult<&str, (Vec<u32>, Vec<u32>)> {
    let (input, (times, distances)) = separated_pair(numeros, line_ending, numeros)(input)?;
    Ok((input, (times, distances)))
}

fn process_input(input: &str) -> Result<u32, String> {
    let (_, (times, distances)) = parse_input(input).expect("no se pudo parsear el input");
    let resultado: usize = times
        .iter()
        .zip(distances.iter())
        .map(|(time, distance)| {
            (0..*time)
                .filter_map(|speed| {
                    let my_distance = (time - speed) * speed;
                    (my_distance > *distance).then_some(my_distance)
                })
                .count()
        })
        .product();
    Ok(resultado as u32)
}

fn main() {
    let input = include_str!("input1.txt");
    println!("{:?}", process_input(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_input() {
        let input = "Time:      7  15   30
Distance:  9  40  200";
        assert_eq!(process_input(input).unwrap(), 288);
    }
}
