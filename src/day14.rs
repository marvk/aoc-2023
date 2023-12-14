use std::collections::HashMap;
use std::hash::Hash;
use std::ops::{Add, Sub};

use crate::harness::{Day, Part};

pub fn day14() -> Day<u64, u64> {
    Day::new(14, Box::new(Part1 {}), Box::new(Part2 {}))
}

pub struct Part1;

impl Part<u64> for Part1 {
    fn expect_test(&self) -> u64 {
        136
    }

    fn solve(&self, input: &[String]) -> u64 {
        let mut map = Map::from(input);
        map.make_step(Vec2::NORTH);
        map.count_load()
    }
}

pub struct Part2;

impl Part<u64> for Part2 {
    fn expect_test(&self) -> u64 {
        64
    }

    fn solve(&self, input: &[String]) -> u64 {
        let mut map = Map::from(input);
        map.make_cycles(1_000_000_000);
        map.count_load()
    }
}

struct Map {
    raw: Vec<Vec<char>>,
    width: usize,
    height: usize,
}

impl Map {
    fn new(raw: Vec<Vec<char>>) -> Self {
        let width = raw[0].len();
        let height = raw.len();

        Self { raw, width, height }
    }

    fn make_cycle(&mut self) {
        self.make_step(Vec2::NORTH);
        self.make_step(Vec2::WEST);
        self.make_step(Vec2::SOUTH);
        self.make_step(Vec2::EAST);
    }

    fn make_step(&mut self, delta: Vec2) {
        match delta {
            Vec2::NORTH | Vec2::WEST => {
                for y in 0..self.height {
                    for x in 0..self.width {
                        self.slide_rock(x, y, &delta);
                    }
                }
            }
            Vec2::SOUTH | Vec2::EAST => {
                for y in (0..self.height).rev() {
                    for x in (0..self.width).rev() {
                        self.slide_rock(x, y, &delta);
                    }
                }
            }
            _ => panic!()
        }
    }

    fn slide_rock(&mut self, x: usize, y: usize, step: &Vec2) {
        if self.raw[y][x] == ROCK {
            let mut next_x = x as i64;
            let mut next_y = y as i64;

            loop {
                next_x += step.x;
                next_y += step.y;

                if !self.is_in_bounds(next_x, next_y) || self.raw[next_y as usize][next_x as usize] != EMPTY {
                    break;
                }
            }

            self.raw[y][x] = EMPTY;
            self.raw[(next_y - step.y) as usize][(next_x - step.x) as usize] = ROCK;
        }
    }

    fn is_in_bounds(&self, next_x: i64, next_y: i64) -> bool {
        next_y >= 0 && next_y < self.height as i64 && next_x >= 0 && next_x < self.width as i64
    }

    fn make_cycles(&mut self, times: usize) {
        let mut cache = HashMap::<Vec<Vec<char>>, usize>::new();
        cache.insert(self.raw.clone(), 0);

        let mut i = 0;

        let mut cycle_found = false;

        while i < times {
            self.make_cycle();

            let new_rocks = self.raw.clone();

            let option = cache.get(&new_rocks);

            if let Some(&cycle_start) = option {
                if !cycle_found {
                    let cycle_length = i - cycle_start;

                    while i <= times {
                        i += cycle_length;
                    }

                    i -= cycle_length;

                    cycle_found = true;
                }
            } else {
                cache.insert(new_rocks, i);
            }

            i += 1;
        }
    }

    fn count_load(&self) -> u64 {
        self.raw.iter()
            .enumerate()
            .map(|(y, line)| line.iter().filter(|&&e| e == ROCK).count() as i64 * (self.height as i64 - y as i64))
            .map(|e| e as u64)
            .sum()
    }
}

const ROCK: char = 'O';
const EMPTY: char = '.';

impl From<&[String]> for Map {
    fn from(value: &[String]) -> Self {
        let raw =
            value.iter()
                .filter(|l| !l.is_empty())
                .map(|line| line.chars().collect())
                .collect();

        Map::new(raw)
    }
}


const fn v(x: i64, y: i64) -> Vec2 {
    Vec2::new(x, y)
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
struct Vec2 {
    x: i64,
    y: i64,
}

impl Vec2 {
    pub const NORTH: Self = v(0, -1);
    pub const EAST: Self = v(1, 0);
    pub const SOUTH: Self = v(0, 1);
    pub const WEST: Self = v(-1, 0);

    pub const fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }
}

impl Add<Vec2> for Vec2 {
    type Output = Self;

    fn add(self, rhs: Vec2) -> Self::Output {
        v(self.x + rhs.x, self.y + rhs.y)
    }
}

impl Sub<Vec2> for Vec2 {
    type Output = Self;

    fn sub(self, rhs: Vec2) -> Self::Output {
        v(self.x - rhs.x, self.y - rhs.y)
    }
}
