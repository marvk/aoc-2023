use std::collections::{HashMap, HashSet};
use std::ops::{Add, Sub};

use crate::harness::{Day, Part};

pub fn day03() -> Day<i32, i32> {
    Day::new(3, Box::new(Part1 {}), Box::new(Part2 {}))
}

pub struct Part1;

impl Part<i32> for Part1 {
    fn expect_test(&self) -> i32 {
        4361
    }

    fn solve(&self, input: &[String]) -> i32 {
        Map::from(input)
            .get_parts_and_nonparts()
            .iter()
            .filter(|p| p.has_parts())
            .map(|p| p.number)
            .sum()
    }
}

pub struct Part2;

impl Part<i32> for Part2 {
    fn expect_test(&self) -> i32 {
        467835
    }

    fn solve(&self, input: &[String]) -> i32 {
        let parts = Map::from(input).get_parts_and_nonparts();

        parts.iter()
            .flat_map(|p|
                p.parts.iter()
                    .clone()
                    .filter(|&(_, part)| *part == '*')
                    .map(|(&pos, _)| pos)
            )
            .collect::<HashSet<_>>()
            .iter()
            .map(|g|
                parts.iter()
                    .filter(|p| p.parts.contains_key(g))
                    .collect::<Vec<_>>()
            )
            .filter(|parts| parts.len() == 2)
            .map(|parts| parts[0].number * parts[1].number)
            .sum()
    }
}

struct Map {
    raw: HashMap<Vec2, char>,
    width: usize,
    height: usize,
}

impl Map {
    fn new(raw: HashMap<Vec2, char>, width: usize, height: usize) -> Self {
        Self { raw, width, height }
    }

    fn get_parts_and_nonparts(&self) -> Vec<PartNumber> {
        let mut current_number = String::new();
        let mut current_parts = HashMap::new();

        let mut parts = Vec::new();

        for y in 0..self.height {
            // +1 to the right edge so numbers are terminated without checking after the loop
            for x in 0..=self.width {
                let cur_pos = p(x as i32, y as i32);
                let possible_cur = self.raw.get(&cur_pos);

                if possible_cur.is_some() && possible_cur.unwrap().is_numeric() {
                    let cur = *possible_cur.unwrap();
                    current_number.push(cur);

                    for d in Vec2::DIRECTIONS {
                        let part_pos = cur_pos + d;
                        let possible_part = self.raw.get(&part_pos);

                        if let Some(&part) = possible_part {
                            if !part.is_numeric() && part != '.' {
                                current_parts.insert(part_pos, part);
                            }
                        }
                    }
                } else if !current_number.is_empty() {
                    parts.push(PartNumber::new(current_number.parse::<i32>().unwrap(), current_parts));
                    current_number = String::new();
                    current_parts = HashMap::new();
                }
            }
        }

        parts
    }
}

impl From<&[String]> for Map {
    fn from(value: &[String]) -> Self {
        let mut result = HashMap::new();

        value.iter().enumerate().for_each(|(y, arr)| {
            arr.chars().enumerate().for_each(|(x, c)| {
                result.insert(p(x as i32, y as i32), c);
            })
        });

        Map::new(result, value[0].len(), value.len())
    }
}

const fn p(x: i32, y: i32) -> Vec2 {
    Vec2::new(x, y)
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
struct Vec2 {
    x: i32,
    y: i32,
}

impl Vec2 {
    pub const NORTH: Self = p(0, -1);
    pub const NORTH_EAST: Self = p(1, -1);
    pub const EAST: Self = p(1, 0);
    pub const SOUTH_EAST: Self = p(1, 1);
    pub const SOUTH: Self = p(0, 1);
    pub const SOUTH_WEST: Self = p(-1, 1);
    pub const WEST: Self = p(-1, 0);
    pub const NORTH_WEST: Self = p(-1, -1);

    pub const DIRECTIONS: [Self; 8] = [Self::NORTH, Self::NORTH_EAST, Self::EAST, Self::SOUTH_EAST, Self::SOUTH, Self::SOUTH_WEST, Self::WEST, Self::NORTH_WEST, ];

    pub const fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

impl Add<Vec2> for Vec2 {
    type Output = Self;

    fn add(self, rhs: Vec2) -> Self::Output {
        p(self.x + rhs.x, self.y + rhs.y)
    }
}

impl Sub<Vec2> for Vec2 {
    type Output = Self;

    fn sub(self, rhs: Vec2) -> Self::Output {
        p(self.x - rhs.x, self.y - rhs.y)
    }
}

#[derive(Debug)]
struct PartNumber {
    number: i32,
    parts: HashMap<Vec2, char>,
}

impl PartNumber {
    pub fn new(number: i32, parts: HashMap<Vec2, char>) -> Self {
        Self { number, parts }
    }

    fn has_parts(&self) -> bool {
        !self.parts.is_empty()
    }
}
