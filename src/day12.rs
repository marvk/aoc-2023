use std::collections::HashMap;
use std::str::FromStr;

use crate::harness::{Day, Part};

pub fn day12() -> Day<u64, u64> {
    Day::new(12, Box::new(Part1 {}), Box::new(Part2 {}))
}

pub struct Part1;

impl Part<u64> for Part1 {
    fn expect_test(&self) -> u64 {
        21
    }

    fn solve(&self, input: &[String]) -> u64 {
        parse(input).into_iter()
            .map(solve)
            .sum()
    }
}

pub struct Part2;

impl Part<u64> for Part2 {
    fn expect_test(&self) -> u64 {
        525152
    }

    fn solve(&self, input: &[String]) -> u64 {
        parse(input).into_iter()
            .map(|r| r.extend(5))
            .map(solve)
            .sum()
    }
}

fn solve(r: Record) -> u64 {
    solve_rec(r.chars.as_slice(), r.working_groups.as_slice(), &mut HashMap::new(), 0, 0, 0)
}

fn solve_rec(chars: &[char], groups: &[usize], cache: &mut HashMap<(usize, usize, usize), u64>, char_index: usize, group_index: usize, current_group_size: usize) -> u64 {
    let key = (char_index, group_index, current_group_size);

    if let Some(result) = cache.get(&key) {
        return *result;
    }

    let end_of_input = char_index == chars.len();

    if end_of_input {
        let all_groups_cleared = group_index == groups.len();
        let not_currently_on_group = current_group_size == 0;

        let on_last_group_and_current_size_is_last_group_size = group_index == (groups.len() - 1) && groups[group_index] == current_group_size;

        return if (all_groups_cleared && not_currently_on_group) || (on_last_group_and_current_size_is_last_group_size) {
            1
        } else {
            0
        };
    }

    let current = chars[char_index];

    let mut total = 0;

    if current == '.' || current == '?' {
        let not_currently_on_group = current_group_size == 0;

        let current_group_is_not_out_of_bounds_and_current_group_is_completed = group_index < groups.len() && groups[group_index] == current_group_size;

        if not_currently_on_group {
            total += solve_rec(chars, groups, cache, char_index + 1, group_index, 0);
        } else if current_group_is_not_out_of_bounds_and_current_group_is_completed {
            total += solve_rec(chars, groups, cache, char_index + 1, group_index + 1, 0);
        }
    }

    if current == '#' || current == '?' {
        total += solve_rec(chars, groups, cache, char_index + 1, group_index, current_group_size + 1);
    }

    cache.insert(key, total);

    total
}

fn parse(input: &[String]) -> Vec<Record> {
    input.iter()
        .filter(|l| !l.is_empty())
        .map(|l| Record::from_str(l).unwrap())
        .collect()
}

#[derive(Debug)]
struct Record {
    chars: Vec<char>,
    working_groups: Vec<usize>,
}

impl Record {
    fn new(chars: Vec<char>, working_groups: Vec<usize>) -> Self {
        Self { chars, working_groups }
    }

    fn extend(&self, factor: usize) -> Record {
        Record::from_str(
            &format!(
                "{} {}",
                vec![self.chars.iter().collect::<String>(); factor].join("?"),
                vec![self.working_groups.clone(); factor].iter().flatten().map(|e| e.to_string()).collect::<Vec<_>>().join(",")
            )
        ).unwrap()
    }
}

impl FromStr for Record {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split(' ');

        let chars =
            split.next().unwrap().chars()
                .collect::<Vec<_>>();

        let working_groups =
            split.next().unwrap().split(',')
                .map(|e| e.parse::<usize>().unwrap())
                .collect::<Vec<_>>();

        Ok(Record::new(
            chars,
            working_groups,
        ))
    }
}
