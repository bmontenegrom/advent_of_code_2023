use std::{cmp::Ordering, collections::HashMap, iter::zip};

#[derive(Debug, Eq)]
struct Card {
    name: char,
    value: u32,
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> Ordering {
        self.value.cmp(&other.value)
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Card {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl Card {
    fn new(name: char, value: u32) -> Card {
        Card { name, value }
    }
}

#[derive(Debug, Eq)]
struct Hand {
    cards: Vec<Card>,
    value: u32,
    bid: u32,
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.value == other.value {
            for (card, other_card) in zip(self.cards.iter(), other.cards.iter()) {
                if card.value != other_card.value {
                    return card.value.cmp(&other_card.value);
                }
            }
            return Ordering::Equal;
        } else {
            self.value.cmp(&other.value)
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        if self.value == other.value {
            for (card, other_card) in zip(self.cards.iter(), other.cards.iter()) {
                if card.value != other_card.value {
                    return false;
                }
            }
            return true;
        } else {
            false
        }
    }
}

impl Hand {
    fn new(cards: Vec<Card>, bid: u32) -> Hand {
        let mut map = HashMap::new();
        cards.iter().for_each(|card| {
            let counter = map.entry(card.name).or_insert(0);
            *counter += 1;
        });
        let value = match map.len() {
            1 => 7, //Five of a kind
            2 => {
                if map.values().any(|&value| value == 4) {
                    6 //Four of a kind
                } else {
                    5 //Full house
                }
            }
            3 => {
                if map.values().any(|&value| value == 3) {
                    4 //Three of a kind
                } else {
                    3 //Two pair
                }
            }
            4 => 2, //One pair
            5 => 1, //High card
            _ => panic!("Invalid hand"),
        };
        Hand { cards, value, bid }
    }
}

fn parse_input(input: &str) -> Vec<Hand> {
    input
        .lines()
        .map(|line| {
            let line_split: Vec<_> = line.split(" ").collect();
            let cards_str = line_split[0];
            let bid_str = line_split[1];

            let bid = bid_str.parse::<u32>().expect("Invalid input");
            let cards = cards_str
                .chars()
                .map(|c| match c {
                    'A' => Card::new('A', 14),
                    'K' => Card::new('K', 13),
                    'Q' => Card::new('Q', 12),
                    'J' => Card::new('J', 11),
                    'T' => Card::new('T', 10),
                    _ => Card::new(c, c.to_digit(10).expect("invalid card")),
                })
                .collect::<Vec<Card>>();
            Hand::new(cards, bid)
        })
        .collect()
}

fn process_input(input: &str) -> Result<u32, String> {
    let mut hands = parse_input(input);
    hands.sort();
    let result = hands
        .iter()
        .enumerate()
        .map(|(i, hand)| {
            (i + 1) as u32 * hand.bid
        })
        .sum();
    Ok(result)
}

fn main() {
    let input = include_str!("input1.txt");
    println!("{}", process_input(input).unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_input() {
        let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
        assert_eq!(process_input(input).unwrap(), 6440);
    }
}
