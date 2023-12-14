use std::collections::{HashMap, HashSet};
use std::fmt::Display;
use std::hash::{Hash, Hasher};
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

#[derive(Eq, PartialEq, Debug)]
struct Rocks(HashSet<Vec2>);

impl Hash for Rocks {
    fn hash<H: Hasher>(&self, state: &mut H) {
        for x in &self.0 {
            x.hash(state);
        }
    }
}

struct Map {
    obstacles: HashSet<Vec2>,
    rocks: HashSet<Vec2>,
    width: usize,
    height: usize,
}

impl Map {
    fn new(obstacles: HashSet<Vec2>, rocks: HashSet<Vec2>, width: usize, height: usize) -> Self {
        Self { obstacles, rocks, width, height }
    }

    fn make_cycle(&mut self) {
        self.make_step(Vec2::NORTH);
        self.make_step(Vec2::WEST);
        self.make_step(Vec2::SOUTH);
        self.make_step(Vec2::EAST);
    }

    fn make_step(&mut self, direction: Vec2) {
        loop {
            let mut changes = false;

            self.rocks = self.rocks.iter()
                .map(|r| {
                    let next = *r + direction;

                    if next.y >= 0 && next.y < self.height as i64 && next.x >= 0 && next.x < self.width as i64 && !self.rocks.contains(&next) && !self.obstacles.contains(&next) {
                        changes = true;
                        next
                    } else {
                        *r
                    }
                })
                .collect();

            if !changes {
                break;
            }
        }
    }

    fn make_cycles(&mut self, times: usize) {
        let mut cache = HashMap::<Rocks, usize>::new();
        cache.insert(Rocks(self.rocks.clone()), 0);

        let mut i = 0;

        while i < times {
            self.make_cycle();

            let new_rocks = Rocks(self.rocks.clone());

            if let Some(&last_occurrence) = cache.get(&new_rocks) {
                let cycle_length = i - last_occurrence;

                while i < times {
                    i += cycle_length;
                }

                i -= cycle_length;
            } else {
                cache.insert(new_rocks, i);
            }

            i += 1;
        }
    }

    fn count_load(&self) -> u64 {
        self.rocks.iter()
            .map(|r| r.y)
            .map(|y| self.height as i64 - y)
            .map(|x| x as u64)
            .sum()
    }
}

impl From<&[String]> for Map {
    fn from(value: &[String]) -> Self {
        let value = value.iter().filter(|l| !l.is_empty()).collect::<Vec<_>>();

        let mut obstacles = HashSet::new();
        let mut rocks = HashSet::new();

        for (y, line) in value.iter().enumerate() {
            for (x, char) in line.chars().enumerate() {
                match char {
                    'O' => rocks.insert(v_usize(x, y)),
                    '#' => obstacles.insert(v_usize(x, y)),
                    '.' => false,
                    _ => panic!()
                };
            }
        }

        Map::new(obstacles, rocks, value[0].len(), value.len())
    }
}


const fn v(x: i64, y: i64) -> Vec2 {
    Vec2::new(x, y)
}

const fn v_usize(x: usize, y: usize) -> Vec2 {
    v(x as i64, y as i64)
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

    pub const fn manhattan_dist(&self, other: &Vec2) -> i64 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
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
