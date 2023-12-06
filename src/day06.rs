#![allow(warnings, unused)]

use std::cmp::min;
use std::ops::Range;
use std::str::FromStr;

use regex::Regex;

use crate::harness::{Day, Part};

pub fn day06() -> Day<u64, u64> {
    Day::new(6, Box::new(Part1 {}), Box::new(Part2 {}))
}

pub struct Part1;

impl Part<u64> for Part1 {
    fn expect_test(&self) -> u64 {
        288
    }

    fn solve(&self, input: &[String]) -> u64 {
        solve(input)
    }
}

pub struct Part2;

impl Part<u64> for Part2 {
    fn expect_test(&self) -> u64 {
        71503
    }

    fn solve(&self, input: &[String]) -> u64 {
        let input =
            input.iter()
                .map(|s| s.replace(" ", "").replace(":", " "))
                .collect::<Vec<_>>();

        solve(&input)
    }
}

fn solve(input: &[String]) -> u64 {
    Races::from(input)
        .races
        .into_iter()
        .map(|race|
            (0..=race.time)
                .filter(|time_held| (race.time - time_held) * time_held > race.distance_record)
                .count() as u64
        )
        .product()
}

#[derive(Debug)]
struct Races {
    races: Vec<Race>,
}

impl Races {
    pub fn new(races: Vec<Race>) -> Self {
        Self { races }
    }
}

impl From<&[String]> for Races {
    fn from(value: &[String]) -> Self {
        let times = parse_line(&value[0]);
        let distances = parse_line(&value[1]);

        let races =
            times.into_iter()
                .zip(distances)
                .map(|(time, distance)| Race::new(time, distance))
                .collect();

        Self::new(races)
    }
}

#[derive(Debug)]
struct Race {
    time: u64,
    distance_record: u64,
}

impl Race {
    pub fn new(time: u64, distance: u64) -> Self {
        Self { time, distance_record: distance }
    }
}


fn parse_line(s: &str) -> Vec<u64> {
    Regex::new(r" +").unwrap()
        .split(s)
        .skip(1)
        .map(|s| s.parse::<u64>().unwrap())
        .collect::<Vec<_>>()
}
