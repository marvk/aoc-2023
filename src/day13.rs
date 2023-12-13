use crate::harness::{Day, Part};

pub fn day13() -> Day<u64, u64> {
    Day::new(13, Box::new(Part1 {}), Box::new(Part2 {}))
}

pub struct Part1;

impl Part<u64> for Part1 {
    fn expect_test(&self) -> u64 {
        405
    }

    fn solve(&self, input: &[String]) -> u64 {
        solve(input, 0)
    }
}


pub struct Part2;

impl Part<u64> for Part2 {
    fn expect_test(&self) -> u64 {
        400
    }

    fn solve(&self, input: &[String]) -> u64 {
        solve(input, 1)
    }
}

fn solve(input: &[String], smudge_count: usize) -> u64 {
    parse(input).iter()
        .map(|map|
            map.find_vertical_mirror(smudge_count)
                .map(|e| e * 100)
                .unwrap_or_else(||
                    map.rotate().find_vertical_mirror(smudge_count).unwrap()
                ) as u64
        )
        .sum()
}

fn parse(input: &[String]) -> Vec<Map> {
    input.split(|line| line.is_empty())
        .filter(|arr| !arr.is_empty())
        .map(Map::from)
        .collect()
}

fn diff(s1: &[char], s2: &[char]) -> usize {
    s1.iter().zip(s2)
        .filter(|(e1, e2)| e1 != e2)
        .count()
}

struct Map {
    lines: Vec<Vec<char>>,
}

impl Map {
    fn new(lines: Vec<Vec<char>>) -> Self {
        Self { lines }
    }

    fn height(&self) -> usize {
        self.lines.len()
    }

    fn width(&self) -> usize {
        self.lines[0].len()
    }

    fn rotate(&self) -> Self {
        let new_height = self.width();
        let mut result = vec![vec![]; new_height];

        for line in &self.lines {
            for (y, char) in line.iter().rev().enumerate() {
                result[new_height - y - 1].push(*char);
            }
        }

        Map::new(result)
    }

    fn find_vertical_mirror(&self, required_smudges: usize) -> Option<usize> {
        for i in 1..self.height() {
            let mut current_smudges = 0;

            for j in 0.. {
                let upper =
                    i.checked_sub(1)
                        .and_then(|i| i.checked_sub(j))
                        .and_then(|i| self.lines.get(i));

                let lower = self.lines.get(i + j);

                if let (Some(upper), Some(lower)) = (upper, lower) {
                    current_smudges += diff(upper, lower);

                    if current_smudges > required_smudges {
                        break;
                    }
                } else {
                    break;
                }
            }

            if current_smudges == required_smudges {
                return Some(i);
            }
        }

        None
    }
}

impl From<&[String]> for Map {
    fn from(value: &[String]) -> Self {
        Map::new(value.iter().map(|line| line.chars().collect()).collect())
    }
}
