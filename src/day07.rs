#![allow(warnings, unused)]

use std::cmp::{max, min};
use std::collections::HashSet;
use std::ops::Range;
use std::str::FromStr;

use regex::Regex;

use crate::harness::{Day, Part};

pub fn day07() -> Day<u64, u64> {
    Day::new(7, Box::new(Part1 {}), Box::new(Part2 {}))
}

pub struct Part1;

impl Part<u64> for Part1 {
    fn expect_test(&self) -> u64 {
        6440
    }

    fn solve(&self, input: &Vec<String>) -> u64 {
        let mut vec = input.into_iter()
            .filter(|s| !s.is_empty())
            .map(|l| {
                let mut split = l.split(" ");
                Hand::new_part_1(
                    split.next().unwrap().to_string(),
                    split.next().unwrap().parse().unwrap(),
                )
            })
            .collect::<Vec<_>>();

        vec.sort_by_cached_key(|h| (h.hand_type, h.card_values[0], h.card_values[1], h.card_values[2], h.card_values[3], h.card_values[4]));

        vec.iter()
            .enumerate()
            .map(|(i, &Hand { bid, .. })| (i + 1) as u64 * bid as u64)
            .sum()
    }
}

pub struct Part2;

impl Part<u64> for Part2 {
    fn expect_test(&self) -> u64 {
        5905
    }

    fn solve(&self, input: &Vec<String>) -> u64 {
        let mut vec = input.into_iter()
            .filter(|s| !s.is_empty())
            .map(|l| {
                let mut split = l.split(" ");
                Hand::new_part_2(
                    split.next().unwrap().to_string(),
                    split.next().unwrap().parse().unwrap(),
                )
            })
            .collect::<Vec<_>>();

        vec.sort_by_cached_key(|h| (h.hand_type, h.card_values[0], h.card_values[1], h.card_values[2], h.card_values[3], h.card_values[4]));

        vec.iter()
            .enumerate()
            .map(|(i, &Hand { bid, .. })| (i + 1) as u64 * bid as u64)
            .sum()

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

#[derive(Debug)]
struct Hand {
    cards: String,
    card_values: Vec<u32>,
    bid: u32,
    hand_type: HandType,
}

impl Hand {
    pub fn new_part_1(cards: String, bid: u32) -> Self {
        let hand_type = Self::calculate_hand_type_part_1(&cards);

        let card_values = Self::calculate_card_values_part_1(&cards);
        Self { cards, card_values, bid, hand_type }
    }

    pub fn new_part_2(cards: String, bid: u32) -> Self {
        let hand_type = Self::calculate_hand_type_part_2(&cards);

        let card_values = Self::calculate_card_values_part_2(&cards);
        Self { cards, card_values, bid, hand_type }
    }

    fn calculate_card_values_part_1(cards: &str) -> Vec<u32> {
        cards.chars().map(|c| {
            match c {
                'A' => 14,
                'K' => 13,
                'Q' => 12,
                'J' => 11,
                'T' => 10,
                _ if c.is_numeric() => c.to_digit(10).unwrap(),
                _ => panic!(),
            }
        }).collect()
    }

    fn calculate_hand_type_part_1(cards: &str) -> HandType {
        let distinct_chars = cards.chars().collect::<HashSet<_>>();

        match distinct_chars.len() {
            1 => HandType::FiveOfAKind,
            2 => {
                let query_char = distinct_chars.iter().next().unwrap();
                let query_count = cards.chars().filter(|c| c == query_char).count();

                if query_count == 1 || query_count == 4 {
                    HandType::FourOfAKind
                } else {
                    HandType::FullHouse
                }
            }
            3 => {
                let any_threes = distinct_chars.iter().map(|c1| cards.chars().filter(|c2| c1 == c2).count()).any(|v| v == 3);

                if any_threes {
                    HandType::ThreeOfAKind
                } else {
                    HandType::TwoPair
                }
            }
            4 => HandType::OnePair,
            5 => HandType::HighCard,
            _ => panic!(),
        }
    }

    fn calculate_card_values_part_2(cards: &str) -> Vec<u32> {
        cards.chars().map(|c| {
            match c {
                'A' => 14,
                'K' => 13,
                'Q' => 12,
                'T' => 10,
                _ if c.is_numeric() => c.to_digit(10).unwrap(),
                'J' => 0,
                _ => panic!(),
            }
        }).collect()
    }

    fn calculate_hand_type_part_2(cards: &str) -> HandType {
        let distinct_chars = cards.chars().filter(|&c| c != 'J').collect::<HashSet<_>>();
        let distinct_count = distinct_chars.len();
        let joker_count = cards.chars().filter(|&c| c == 'J').count();

        match joker_count {
            5 | 4 => HandType::FiveOfAKind,
            3 => {
                match distinct_chars.len() {
                    1 => HandType::FiveOfAKind,
                    2 => HandType::FourOfAKind,
                    _ => panic!(),
                }
            }
            2 => {
                match distinct_chars.len() {
                    1 => HandType::FiveOfAKind,
                    2 => HandType::FourOfAKind,
                    3 => HandType::ThreeOfAKind,
                    _ => panic!(),
                }
            }
            1 => {
                match distinct_chars.len() {
                    1 => HandType::FiveOfAKind,
                    2 => {
                        let query_char = distinct_chars.iter().next().unwrap();
                        let query_count = cards.chars().filter(|c| c == query_char).count();

                        if query_count == 1 || query_count == 3 {
                            HandType::FourOfAKind
                        } else {
                            HandType::FullHouse
                        }
                    },
                    3 => HandType::ThreeOfAKind,
                    4 => HandType::OnePair,
                    _ => panic!(),
                }
            }
            0 => Self::calculate_hand_type_part_1(cards),
            _ => panic!()
        }
    }
}
