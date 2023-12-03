use std::cmp::max;
use std::collections::HashMap;
use std::str::FromStr;

use crate::harness::{Day, Part};

pub fn day02() -> Day<i32, i32> {
    Day::new(2, Box::new(Part1 {}), Box::new(Part2 {}))
}

pub struct Part1;

impl Part<i32> for Part1 {
    fn expect_test(&self) -> i32 {
        8
    }

    fn solve(&self, input: &[String]) -> i32 {
        parse(input).iter()
            .filter(|g|
                g.rounds.iter()
                    .all(|r| r.red <= 12 && r.green <= 13 && r.blue <= 14)
            )
            .map(|g| g.id as i32)
            .sum()
    }
}

pub struct Part2;

impl Part<i32> for Part2 {
    fn expect_test(&self) -> i32 {
        2286
    }

    fn solve(&self, input: &[String]) -> i32 {
        parse(input)
            .into_iter()
            .map(|g|
                g.rounds.into_iter()
                    .reduce(|r1, r2|
                        Round::new(
                            max(r1.red, r2.red),
                            max(r1.green, r2.green),
                            max(r1.blue, r2.blue),
                        )
                    )
                    .unwrap()
            )
            .map(|min| (min.red * min.green * min.blue) as i32)
            .sum()
    }
}

#[derive(Debug, Copy, Clone)]
struct Round {
    red: usize,
    green: usize,
    blue: usize,
}

impl Round {
    pub fn new(red: usize, green: usize, blue: usize) -> Self {
        Self { red, green, blue }
    }
}

impl FromStr for Round {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let map = s
            .split(',')
            .map(|s| s.trim())
            .map(|e| e.split(' ').collect::<Vec<_>>())
            .map(|e| (e[1], e[0].parse::<usize>().unwrap()))
            .collect::<HashMap<_, _>>();

        Ok(
            Round::new(
                *map.get("red").unwrap_or(&0),
                *map.get("green").unwrap_or(&0),
                *map.get("blue").unwrap_or(&0),
            )
        )
    }
}

#[derive(Debug)]
struct Game {
    id: usize,
    rounds: Vec<Round>,
}

impl Game {
    pub fn new(id: usize, rounds: Vec<Round>) -> Self {
        Self { id, rounds }
    }
}

impl FromStr for Game {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let option = s.split_once(':').unwrap();
        let id = option.0.split(' ').last().unwrap().parse::<usize>().unwrap();
        let rounds = option.1.split(';').map(Round::from_str).collect::<Result<Vec<_>, ()>>().unwrap();
        Ok(Game::new(id, rounds))
    }
}

fn parse(input: &[String]) -> Vec<Game> {
    input.iter()
        .filter(|l| !l.is_empty())
        .map(|l| Game::from_str(l))
        .collect::<Result<Vec<Game>, ()>>()
        .unwrap()
}
