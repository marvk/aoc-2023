use std::collections::HashSet;
use std::str::FromStr;

use crate::harness::{Day, Part};

pub fn day07() -> Day<u64, u64> {
    Day::new(7, Box::new(Part1 {}), Box::new(Part2 {}))
}

pub struct Part1;

impl Part<u64> for Part1 {
    fn expect_test(&self) -> u64 {
        6440
    }

    fn solve(&self, input: &[String]) -> u64 {
        let mut hands = parse(input);

        sort(&mut hands, calculate_card_value_part_1, HandType::calculate_part_1);

        solve(&hands)
    }
}

pub struct Part2;

impl Part<u64> for Part2 {
    fn expect_test(&self) -> u64 {
        5905
    }

    fn solve(&self, input: &[String]) -> u64 {
        let mut hands = parse(input);

        sort(&mut hands, calculate_card_value_part_2, HandType::calculate_part_2);

        solve(&hands)
    }
}

fn parse(input: &[String]) -> Vec<Hand> {
    input.iter()
        .filter(|s| !s.is_empty())
        .map(|l| Hand::from_str(l).unwrap())
        .collect::<Vec<_>>()
}

fn sort(hands: &mut [Hand], calculate_card_value: fn(char) -> u32, calculate_hand_type: fn(&str) -> HandType) {
    hands.sort_by_cached_key(|h| {
        let card_values = h.cards.chars().map(calculate_card_value).collect::<Vec<_>>();
        (calculate_hand_type(&h.cards), card_values[0], card_values[1], card_values[2], card_values[3], card_values[4])
    })
}

fn solve(hands: &[Hand]) -> u64 {
    hands.iter()
        .enumerate()
        .map(|(i, &Hand { bid, .. })| (i + 1) as u64 * bid as u64)
        .sum()
}

fn calculate_card_value_part_1(c: char) -> u32 {
    match c {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => 11,
        'T' => 10,
        _ if c.is_numeric() => c.to_digit(10).unwrap(),
        _ => panic!(),
    }
}

fn calculate_card_value_part_2(c: char) -> u32 {
    match c {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'T' => 10,
        _ if c.is_numeric() => c.to_digit(10).unwrap(),
        'J' => 0,
        _ => panic!(),
    }
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
enum HandType {
    FiveOfAKind = 6,
    FourOfAKind = 5,
    FullHouse = 4,
    ThreeOfAKind = 3,
    TwoPair = 2,
    OnePair = 1,
    HighCard = 0,
}

impl HandType {
    fn calculate_part_1(cards: &str) -> Self {
        let distinct_chars = cards.chars().collect::<HashSet<_>>();

        match distinct_chars.len() {
            1 => Self::FiveOfAKind,
            2 => {
                let query_char = distinct_chars.iter().next().unwrap();
                let query_count = cards.chars().filter(|c| c == query_char).count();

                if query_count == 1 || query_count == 4 {
                    Self::FourOfAKind
                } else {
                    Self::FullHouse
                }
            }
            3 => {
                let any_threes = distinct_chars.iter().map(|c1| cards.chars().filter(|c2| c1 == c2).count()).any(|v| v == 3);

                if any_threes {
                    Self::ThreeOfAKind
                } else {
                    Self::TwoPair
                }
            }
            4 => Self::OnePair,
            5 => Self::HighCard,
            _ => panic!(),
        }
    }

    fn calculate_part_2(cards: &str) -> Self {
        let distinct_chars = cards.chars().filter(|&c| c != 'J').collect::<HashSet<_>>();
        let distinct_count = distinct_chars.len();
        let joker_count = cards.chars().filter(|&c| c == 'J').count();

        match (joker_count, distinct_count) {
            (5, _) | (4, _) | (_, 1) => Self::FiveOfAKind,
            (3 | 2, 2) => Self::FourOfAKind,
            (2 | 1, 3) => Self::ThreeOfAKind,
            (1, 2) => {
                let query_char = distinct_chars.iter().next().unwrap();
                let query_count = cards.chars().filter(|c| c == query_char).count();

                if query_count == 1 || query_count == 3 {
                    Self::FourOfAKind
                } else {
                    Self::FullHouse
                }
            }
            (1, 4) => Self::OnePair,
            (0, _) => Self::calculate_part_1(cards),
            _ => panic!(),
        }
    }
}

#[derive(Debug)]
struct Hand {
    cards: String,
    bid: u32,
}

impl Hand {
    pub fn new(cards: String, bid: u32) -> Self {
        Self { cards, bid }
    }
}

impl FromStr for Hand {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split(' ');

        Ok(Self::new(
            split.next().unwrap().to_string(),
            split.next().unwrap().parse().unwrap(),
        ))
    }
}
