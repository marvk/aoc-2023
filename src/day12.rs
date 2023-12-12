use std::fmt::Display;
use std::str::FromStr;

use regex::Regex;

use crate::harness::{Day, Part};

pub fn day12() -> Day<usize, usize> {
    Day::new(12, Box::new(Part1 {}), Box::new(Part2 {}))
}

pub struct Part1;

impl Part<usize> for Part1 {
    fn expect_test(&self) -> usize {
        21
    }

    fn solve(&self, input: &[String]) -> usize {
        let records = parse(input);

        let mut total = 0;

        for x in &records {
            // dbg!(x);
            // dbg!(x.unknown_count());
            // dbg!(x.unknown_working_count());

            let permutations = permutations(x.unknown_working_count(), x.unknown_count());


            let mut count = 0;


            for permutation in permutations {
                let mut current = x.chars.clone();
                permutation.iter()
                    .map(|&i| x.unknown_positions[i])
                    .for_each(|i| current[i] = '#');

                // println!("{:?}", current);
                let check1 = check(current.iter().collect::<String>().as_str(), &x.working_groups);

                if check1 {
                    count += 1;
                }
            }

            total += count;
        }

        total
    }
}

fn check(cogs: &str, working_groups: &[usize]) -> bool {
    let lens = Regex::new(r"[^#]")
        .unwrap()
        .split(cogs)
        .filter(|e| !e.is_empty())
        .map(|e| e.len())
        .collect::<Vec<_>>();

    lens == working_groups
}

pub struct Part2;

impl Part<usize> for Part2 {
    fn expect_test(&self) -> usize {
        todo!()
    }

    fn solve(&self, input: &[String]) -> usize {
        todo!()
    }
}

fn parse(input: &[String]) -> Vec<Record> {
    input.iter()
        .filter(|l| !l.is_empty())
        .map(|l| Record::from_str(l).unwrap())
        .collect()
}

fn permutations(selected: usize, n: usize) -> Vec<Vec<usize>> {
    permutations_rec(vec![], 0, selected, n)
}

fn permutations_rec(previous: Vec<usize>, index: usize, remaining_selected: usize, total_length: usize) -> Vec<Vec<usize>> {
    if remaining_selected == 0 {
        return vec![previous];
    } else if index == total_length {
        return vec![];
    }

    let mut results = vec![];

    results.extend_from_slice(&permutations_rec(previous.clone(), index + 1, remaining_selected, total_length));


    let mut with_appended = previous.clone();
    with_appended.push(index);
    results.extend_from_slice(&permutations_rec(with_appended, index + 1, remaining_selected - 1, total_length));

    results
}

#[derive(Debug)]
struct Record {
    raw: String,
    chars: Vec<char>,
    working_groups: Vec<usize>,
    unknown_positions: Vec<usize>,
    working_count: usize,
    known_working_count: usize,
}

impl Record {
    pub fn new(raw: String, chars: Vec<char>, working_groups: Vec<usize>, unknown_positions: Vec<usize>, working_count: usize, known_working_count: usize) -> Self {
        Self { raw, chars, working_groups, unknown_positions, working_count, known_working_count }
    }

    fn unknown_working_count(&self) -> usize {
        self.working_count - self.known_working_count
    }

    fn unknown_count(&self) -> usize {
        self.unknown_positions.len()
    }
}

impl FromStr for Record {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split(' ');
        let chars = split.next().unwrap().chars().collect::<Vec<_>>();
        let unknown_positions = chars.iter().enumerate().filter(|(_, c)| **c == '?').map(|(i, _)| i).collect::<Vec<_>>();
        let working_groups = split.next().unwrap().split(',').map(|e| e.parse::<usize>().unwrap()).collect::<Vec<_>>();

        let working_count = working_groups.iter().sum::<usize>();
        let known_working_count = chars.iter().filter(|c| **c == '#').count();


        Ok(Record::new(
            s.to_string(),
            chars,
            working_groups,
            unknown_positions,
            working_count,
            known_working_count,
        ))
    }
}
