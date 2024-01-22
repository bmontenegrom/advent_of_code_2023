use std::collections::HashMap;

use nom::{
    bytes::complete::tag,
    character::complete::{digit1, newline, one_of, space1},
    multi::{many1, separated_list1},
    IResult,
};

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
enum SpringType {
    Operational,
    Damaged,
    Unknown,
}

impl From<char> for SpringType {
    fn from(c: char) -> Self {
        match c {
            '.' => SpringType::Operational,
            '#' => SpringType::Damaged,
            _ => SpringType::Unknown,
        }
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
struct Record {
    springs: Vec<SpringType>,
    groups: Vec<usize>,
}

impl Record {
    fn new(springs: Vec<SpringType>, groups: Vec<usize>) -> Self {
        Record { springs, groups }
    }

    fn parse(input: &str) -> IResult<&str, Record> {
        let (input, springs) = many1(one_of(".#?"))(input)?;
        let (input, _) = space1(input)?;
        let (input, grops) = separated_list1(tag(","), digit1)(input)?;
        Ok((
            input,
            Record::new(
                springs.into_iter().map(|c| c.into()).collect(),
                grops.into_iter().map(|s| s.parse().unwrap()).collect(),
            ),
        ))
    }

    fn parse_all(input: &str) -> Vec<Record> {
        separated_list1(newline, Record::parse)(input).unwrap().1
    }

    fn expand(&self) -> Record {
        let sptrings = self
            .springs
            .iter()
            .cloned()
            .chain([SpringType::Unknown].iter().cloned())
            .cycle()
            .take(self.springs.len() * 5 + 4)
            .collect();
        let groups = self
            .groups
            .iter()
            .cloned()
            .cycle()
            .take(self.groups.len() * 5)
            .collect();
        Record::new(sptrings, groups)
    }
}

fn posible_solutions(memo: &mut HashMap<Record, usize>, record: &Record) -> usize {
    if let Some(&value) = memo.get(record) {
        return value;
    }

    //si no quedan numeros, o tengo una solución o no es valido. Si queda algun dañado no tiene solucion
    if record.groups.is_empty() {
        let value = match record.springs.iter().any(|c| *c == SpringType::Damaged) {
            true => 0,
            false => 1,
        };
        memo.insert(record.clone(), value);
        return value;
    };

    //chequeo si hay suficiente espacio para colocar los dañados restantes
    if record.springs.len() < record.groups.iter().sum::<usize>() + record.groups.len() - 1 {
        memo.insert(record.clone(), 0);
        return 0;
    }

    //si el primero es operativo, no puedo colocar un dañado, armo el nuevo record ignorando el primero
    if record.springs[0] == SpringType::Operational {
        let solutions = posible_solutions(
            memo,
            &Record::new(record.springs[1..].to_vec(), record.groups.clone()),
        );
        memo.insert(record.clone(), solutions);
        return solutions;
    }

    let mut solutions = 0;
    let current_group = record.groups[0];
    let all_non_operational = record.springs[0..current_group]
        .iter()
        .all(|c| *c != SpringType::Operational);
    //el final va a ser el que este despues o el final del spring
    let end = (current_group + 1).min(record.springs.len());

    //si se cumplen las condiciones puedo colocar el grupo de dañados en la posicion actual y moverme a la siguiente
    //La siguiente posicion es la que esta uno mas despues del grupo ej: en ##..## la posicion luego del primer
    //par de ## es ##.[.]##. Esta posicion tiene que ser un . o fin de string
    if all_non_operational
        && ((record.springs.len() > current_group
            && record.springs[current_group] != SpringType::Damaged)
            || record.springs.len() <= current_group)
    {
        solutions = posible_solutions(
            memo,
            &Record::new(record.springs[end..].to_vec(), record.groups[1..].to_vec()),
        );
    }

    //si el primer spring es desconocido, puedo no usar el grupo en esta posicion
    if record.springs[0] == SpringType::Unknown {
        solutions += posible_solutions(
            memo,
            &Record::new(record.springs[1..].to_vec(), record.groups.clone()),
        );
    }
    memo.insert(record.clone(), solutions);
    solutions
}

fn main() {
    let input = include_str!("input.txt");
    let records = Record::parse_all(input);
    let mut memo = HashMap::new();
    let solutions = records
        .iter()
        .map(|r| posible_solutions(&mut memo, r))
        .sum::<usize>();
    let now = std::time::Instant::now();
    println!("solutions part 1: {}\n time: {:?} ", solutions, now.elapsed());
    let now = std::time::Instant::now();
    let solutions = records
        .iter()
        .map(|r| posible_solutions(&mut memo, &r.expand()))
        .sum::<usize>();
    println!("solutions part 2: {}\n time: {:?}", solutions, now.elapsed());

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = include_str!("input_test.txt");
        let records = Record::parse_all(input);
        let mut memo = HashMap::new();
        let solutions = records
            .iter()
            .map(|r| posible_solutions(&mut memo, r))
            .sum::<usize>();
        assert_eq!(solutions, 21);
    }
    #[test]
    fn test_part2() {
        let input = include_str!("input_test.txt");
        let records = Record::parse_all(input);
        let mut memo = HashMap::new();
        let solutions = records
            .iter()
            .map(|r| posible_solutions(&mut memo, &r.expand()))
            .sum::<usize>();
        assert_eq!(solutions, 525152);
    }
}
