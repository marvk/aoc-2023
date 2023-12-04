use std::cmp::min;
use std::collections::HashSet;
use std::str::FromStr;

use crate::harness::{Day, Part};

pub fn day04() -> Day<u32, u32> {
    Day::new(4, Box::new(Part1 {}), Box::new(Part2 {}))
}

pub struct Part1;

impl Part<u32> for Part1 {
    fn expect_test(&self) -> u32 {
        13
    }

    fn solve(&self, input: &[String]) -> u32 {
        parse(input).iter().map(|e| e.score()).sum()
    }
}

pub struct Part2;

impl Part<u32> for Part2 {
    fn expect_test(&self) -> u32 {
        30
    }

    fn solve(&self, input: &[String]) -> u32 {
        let cards = parse(input);

        let mut counts = vec![1; cards.len()];

        for card in cards {
            let n = card.count_matches();

            for i in 1..min(n as usize + 1, counts.len()) {
                counts[card.id + i] += counts[card.id];
            }
        }

        counts.iter().sum()
    }
}

#[derive(Debug)]
struct ScratchCard {
    id: usize,
    winning_numbers: Vec<u32>,
    numbers_you_have: Vec<u32>,
}

impl ScratchCard {
    fn new(id: usize, winning_numbers: Vec<u32>, numbers_you_have: Vec<u32>) -> Self {
        Self { id, winning_numbers, numbers_you_have }
    }

    fn score(&self) -> u32 {
        let power = self.count_matches();

        if power > 0 {
            2_u32.pow(power - 1)
        } else {
            0
        }
    }

    fn count_matches(&self) -> u32 {
        let winning_numbers: HashSet<u32> = HashSet::from_iter(self.winning_numbers.iter().cloned());
        let numbers_you_have: HashSet<u32> = HashSet::from_iter(self.numbers_you_have.iter().cloned());

        winning_numbers.intersection(&numbers_you_have).count() as u32
    }
}

impl FromStr for ScratchCard {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split(':');
        let id =
            split.next()
                .unwrap()
                .split(' ')
                .last()
                .unwrap()
                .parse::<usize>()
                .unwrap();

        let mut map =
            split
                .next()
                .unwrap()
                .split('|')
                .map(|s| s.trim())
                .map(|e|
                    e.split(' ')
                        .map(|s| s.trim())
                        .filter(|e| !e.is_empty())
                        .map(|s| s.parse::<u32>().unwrap())
                        .collect::<Vec<_>>())
                .collect::<Vec<_>>();

        Ok(ScratchCard::new(
            id - 1,
            map.remove(0),
            map.remove(0),
        ))
    }
}

fn parse(input: &[String]) -> Vec<ScratchCard> {
    input.iter()
        .filter(|l| !l.is_empty())
        .map(|l| ScratchCard::from_str(l))
        .collect::<Result<Vec<_>, _>>()
        .unwrap()
}
