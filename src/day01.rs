use crate::harness::{Day, Part};

pub fn day01() -> Day<i32, i32> {
    Day::new(1, Box::new(Part1 {}), Box::new(Part2 {}))
}

pub struct Part1;

impl Part<i32> for Part1 {
    fn expect_test(&self) -> i32 {
        142
    }

    fn solve(&self, input: &Vec<String>) -> i32 {
        solve(input, &(1..=9).map(|i| i.to_string()).collect::<Vec<_>>())
    }
}

pub struct Part2;

impl Part<i32> for Part2 {
    fn expect_test(&self) -> i32 {
        281
    }

    fn solve(&self, input: &Vec<String>) -> i32 {
        solve(input, &DIGITS.into_iter().map(|s| s.to_string()).collect::<Vec<_>>())
    }
}

const DIGITS: [&str; 18] = [
    "one",
    "two",
    "three",
    "four",
    "five",
    "six",
    "seven",
    "eight",
    "nine",
    "1",
    "2",
    "3",
    "4",
    "5",
    "6",
    "7",
    "8",
    "9",
];


fn solve(input: &[String], digits: &[String]) -> i32 {
    input
        .iter()
        .filter(|l| !l.is_empty())
        .map(|l| {
            let min = digits.iter()
                .enumerate()
                .map(|(idx, s)| (idx, l.find(s)))
                .filter_map(|(idx, str_idx)| str_idx.map(|str_idx| (idx, str_idx)))
                .min_by_key(|&(_, str_idx)| str_idx)
                .unwrap()
                .0;

            let max = digits.iter()
                .enumerate()
                .map(|(idx, s)| (idx, l.rfind(s)))
                .filter_map(|(idx, str_idx)| str_idx.map(|str_idx| (idx, str_idx)))
                .max_by_key(|&(_, str_idx)| str_idx)
                .unwrap()
                .0;

            format!("{}{}", (min % 9) + 1, (max % 9) + 1).parse::<i32>().unwrap()
        })
        .sum::<i32>()
}
