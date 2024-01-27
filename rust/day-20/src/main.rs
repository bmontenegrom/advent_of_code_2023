use std::collections::{HashMap, VecDeque};

trait HandleCommunication {
    fn handle_comunication(&mut self, comunication: &Communication) -> Vec<Communication>;
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Pulse {
    High,
    Low,
}
#[derive(Debug)]
struct Communication {
    source: String,
    destination: String,
    pulse: Pulse,
}

impl Communication {
    fn new(source: String, destination: String, pulse: Pulse) -> Self {
        Self {
            source,
            destination,
            pulse,
        }
    }
}

#[derive(Debug)]
enum State {
    On,
    Off,
}

#[derive(Debug)]
struct FlipFlop {
    state: State,
    id: String,
    destinations: Vec<String>,
}

impl FlipFlop {
    fn new(id: String, destinations: Vec<String>) -> Self {
        Self {
            state: State::Off,
            id,
            destinations,
        }
    }
}

impl HandleCommunication for FlipFlop {
    fn handle_comunication(&mut self, comunication: &Communication) -> Vec<Communication> {
        match comunication.pulse {
            Pulse::High => Vec::new(),
            Pulse::Low => match self.state {
                State::On => {
                    self.state = State::Off;
                    self.destinations
                        .iter()
                        .map(|destination| Communication {
                            source: self.id.to_owned(),
                            destination: destination.to_owned(),
                            pulse: Pulse::Low,
                        })
                        .collect()
                }
                State::Off => {
                    self.state = State::On;
                    self.destinations
                        .iter()
                        .map(|destination| Communication {
                            source: self.id.to_owned(),
                            destination: destination.to_owned(),
                            pulse: Pulse::High,
                        })
                        .collect()
                }
            },
        }
    }
}

#[derive(Debug)]
struct Broadcaster {
    id: String,
    destinations: Vec<String>,
}

impl Broadcaster {
    fn new(id: String, destinations: Vec<String>) -> Self {
        Self { id, destinations }
    }
}
impl HandleCommunication for Broadcaster {
    fn handle_comunication(&mut self, comunication: &Communication) -> Vec<Communication> {
        self.destinations
            .iter()
            .map(|destination| Communication {
                source: self.id.to_owned(),
                destination: destination.to_owned(),
                pulse: comunication.pulse,
            })
            .collect()
    }
}

#[derive(Debug)]
struct Conjunction {
    id: String,
    destinations: Vec<String>,
    inputs: HashMap<String, Pulse>,
}

impl Conjunction {
    fn new(id: String, inputs: Vec<String>, destinations: Vec<String>) -> Self {
        Self {
            id,
            destinations,
            inputs: inputs
                .into_iter()
                .map(|input| (input.to_string(), Pulse::Low))
                .collect(),
        }
    }
}

impl HandleCommunication for Conjunction {
    fn handle_comunication(&mut self, comunication: &Communication) -> Vec<Communication> {
        self.inputs
            .entry(comunication.source.clone())
            .and_modify(|pulse| {
                *pulse = comunication.pulse;
            });
        let pulse_to_send = if self.inputs.values().all(|pulse| *pulse == Pulse::High) {
            Pulse::Low
        } else {
            Pulse::High
        };
        self.destinations
            .iter()
            .map(|destination| Communication {
                source: self.id.to_owned(),
                destination: destination.to_owned(),
                pulse: pulse_to_send,
            })
            .collect()
    }
}

struct Configuration {
    map: HashMap<String, Box<dyn HandleCommunication>>,
    modules: HashMap<String, Vec<String>>,
}

impl Configuration {
    fn parse_module(input: &str) -> (char, String, Vec<String>) {
        let parts = input.split(" -> ").collect::<Vec<_>>();
        let (type_, name) = match parts[0].chars().next().unwrap() {
            'b' => ('b', parts[0].to_owned()),
            '%' => ('%', parts[0][1..].to_owned()),
            '&' => ('&', parts[0][1..].to_owned()),
            _ => panic!("unknown type"),
        };
        let destinations = parts[1]
            .split(", ")
            .map(|s| s.to_owned())
            .collect::<Vec<_>>();
        (type_, name, destinations)
    }

    fn parse(input: &str) -> Configuration {
        let mut map: HashMap<String, Box<dyn HandleCommunication>> = HashMap::new();
        let modules = input.lines().map(Self::parse_module).collect::<Vec<_>>();
        let mut inputs: HashMap<String, Vec<String>> = HashMap::new();
        for (_, name, destinations) in &modules {
            for destination in destinations {
                inputs
                    .entry(destination.to_string())
                    .or_default()
                    .push(name.to_string());
            }
        }
        for (type_, name, destinations) in modules.clone() {
            let module: Box<dyn HandleCommunication> = match type_ {
                'b' => Box::new(Broadcaster::new(name.to_owned(), destinations)),
                '%' => Box::new(FlipFlop::new(name.to_owned(), destinations)),
                '&' => Box::new(Conjunction::new(
                    name.to_owned(),
                    inputs.get(&name).unwrap().clone(),
                    destinations,
                )),
                _ => panic!("unknown type {}", type_),
            };
            map.insert(name, module);
        }
        let modules = modules
            .into_iter()
            .map(|(_, name, destinations)| (name, destinations))
            .collect::<HashMap<_, _>>();
        Configuration { map, modules }
    }

    fn count_h_l(&mut self) -> (usize, usize) {
        let mut highs = 0;
        let mut lows = 0;
        let mut queue: VecDeque<Communication> = VecDeque::new();
        queue.push_back(Communication::new(
            "button".to_owned(),
            "broadcaster".to_owned(),
            Pulse::Low,
        ));
        while let Some(communication) = queue.pop_front() {
            match communication.pulse {
                Pulse::High => highs += 1,
                Pulse::Low => lows += 1,
            }
            let module = match self.map.get_mut(&communication.destination) {
                Some(m) => m,
                None => continue,
            };
            let new_comunications = module.handle_comunication(&communication);
            queue.extend(new_comunications);
        }
        (highs, lows)
    }

    fn solve_part_1(&mut self) -> usize {
        let (highs, lows) = (0..1_000)
            .map(|_| self.count_h_l())
            .fold((0, 0), |(h1, l1), (h2, l2)| (h1 + h2, l1 + l2));
        highs * lows
    }

    fn solve_part_2(&mut self) -> usize {
        //rx is connected to a conjuction, gf,
        //witch is connected to 4 conjuctions qk, kr, kf and zs.
        let final_node = "rx".to_string();
        let penultimate_node = self
            .modules
            .iter()
            .find_map(|(id, destinations)| destinations.contains(&final_node).then_some(id))
            .unwrap();
        let mut antepenultimate_nodes = self
            .modules
            .iter()
            .filter_map(|(id, destinations)| destinations.contains(penultimate_node).then_some(id))
            .collect::<Vec<_>>();
        let mut lcm_tocalculate = Vec::new();
        for i in 0.. {
            if lcm_tocalculate.len() == 4 {
                break;
            }
            let mut queue: VecDeque<Communication> = VecDeque::new();
            queue.push_back(Communication::new(
                "button".to_owned(),
                "broadcaster".to_owned(),
                Pulse::Low,
            ));
            while let Some(communication) = queue.pop_front() {
                if antepenultimate_nodes.contains(&&communication.source) && communication.pulse == Pulse::High {
                    lcm_tocalculate.push(i+1);
                    println!("{}: {}",communication.source, i+1);
                    let index = antepenultimate_nodes
                        .iter()
                        .position(|&id| *id == communication.source)
                        .unwrap();
                    antepenultimate_nodes.remove(index);
                }
                let module = match self.map.get_mut(&communication.destination) {
                    Some(m) => m,
                    None => continue,
                };
                let new_comunications = module.handle_comunication(&communication);
                queue.extend(new_comunications);
            }
        }
        Configuration::lcm(&lcm_tocalculate)
    }
    fn lcm(nums: &[usize]) -> usize {
        if nums.len() == 1 {
            return nums[0];
        }
        let a = nums[0];
        let b = Configuration::lcm(&nums[1..]);
        a * b / Configuration::gcd_of_two_numbers(a, b)
    }

    fn gcd_of_two_numbers(a: usize, b: usize) -> usize {
        if b == 0 {
            return a;
        }
        Configuration::gcd_of_two_numbers(b, a % b)
    }
}

fn main() {
    let input = include_str!("input.txt");
    let mut configuration = Configuration::parse(input);
    let now = std::time::Instant::now();
    println!("Part 1: {} in {:?}", configuration.solve_part_1(), now.elapsed());
    println!("Part 2: {} in {:?}", configuration.solve_part_2(), now.elapsed());

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = include_str!("input_test.txt");
        let mut configuration = Configuration::parse(input);
        assert_eq!(configuration.solve_part_1(), 32000000);
    }
}
