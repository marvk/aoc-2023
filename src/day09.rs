use std::iter::successors;

use crate::harness::{Day, Part};

pub fn day09() -> Day<i64, i64> {
    Day::new(9, Box::new(Part1 {}), Box::new(Part2 {}))
}

pub struct Part1;

impl Part<i64> for Part1 {
    fn expect_test(&self) -> i64 {
        114
    }

    fn solve(&self, input: &[String]) -> i64 {
        parse(input).into_iter()
            .map(solve)
            .sum()
    }
}

pub struct Part2;

impl Part<i64> for Part2 {
    fn expect_test(&self) -> i64 {
        2
    }

    fn solve(&self, input: &[String]) -> i64 {
        parse(input).into_iter()
            .map(|mut e| {
                e.reverse();
                e
            })
            .map(solve)
            .sum()
    }
}

fn parse(input: &[String]) -> Vec<Vec<i64>> {
    input.iter()
        .filter(|l| !l.is_empty())
        .map(|l| parse_line(l))
        .collect()
}

fn parse_line(line: &str) -> Vec<i64> {
    line.split(' ')
        .map(|e| e.parse::<i64>().unwrap())
        .collect()
}

fn solve(input: Vec<i64>) -> i64 {
    successors(Some(input), |last| Some(next(last)))
        .take_while(|l| l.iter().any(|&e| e != 0))
        .map(|l| *l.last().unwrap())
        .sum()
}

fn next(l: &[i64]) -> Vec<i64> {
    l.windows(2)
        .map(|e| e[1] - e[0])
        .collect()
}
