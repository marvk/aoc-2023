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
                .map(|s| s.replace(' ', "").replace(':', " "))
                .collect::<Vec<_>>();

        solve(&input)
    }
}

fn solve(input: &[String]) -> u64 {
    Races::from(input).races
        .iter()
        .map(solve_race_algebraic)
        .product()
}

fn solve_race_algebraic(race: &Race) -> u64 {
    let a = -1.0;
    let b = race.time as f64;
    let c = -(race.distance_record as f64 + 1.0);

    let sqrt_component = (b.powi(2) - 4.0 * a * c).sqrt();
    let r1 = (-b + sqrt_component) / 2.0 * a;
    let r2 = (-b - sqrt_component) / 2.0 * a;

    let min = r1.min(r2).ceil() as u64;
    let max = r1.max(r2).floor() as u64;

    max - min + 1
}

#[allow(dead_code)]
fn solve_race_brute_force(race: &Race) -> u64 {
    (0..=race.time)
        .filter(|time_held| (race.time - time_held) * time_held > race.distance_record)
        .count() as u64
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
