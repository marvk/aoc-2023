use std::collections::{HashSet, VecDeque};
use std::fmt::Display;
use std::hash::Hash;
use std::ops::{Add, Sub};

use crate::harness::{Day, Part};

pub fn day16() -> Day<i32, i32> {
    Day::new(16, Box::new(Part1 {}), Box::new(Part2 {}))
}

pub struct Part1;

impl Part<i32> for Part1 {
    fn expect_test(&self) -> i32 {
        46
    }

    fn solve(&self, input: &[String]) -> i32 {
        let map = Map::from(input);

        let ray = Ray::new(v(0, 0), Vec2::EAST);

        calculate_energy(&map, ray)
    }
}

pub struct Part2;

impl Part<i32> for Part2 {
    fn expect_test(&self) -> i32 {
        51
    }

    fn solve(&self, input: &[String]) -> i32 {
        let map = Map::from(input);

        let mut i1 =
            (0..(map.width() as i32))
                .flat_map(|x| vec![Ray::new(v(x, 0), Vec2::SOUTH), Ray::new(v(x, map.height() as i32 - 1), Vec2::NORTH)])
                .collect::<Vec<_>>();

        let mut i2 =
            (0..(map.height() as i32))
                .flat_map(|y| vec![Ray::new(v(0, y), Vec2::EAST), Ray::new(v(map.width() as i32 - 1, y), Vec2::WEST)])
                .collect::<Vec<_>>();

        i1.append(&mut i2);

        i1.iter().map(|r| calculate_energy(&map, *r)).max().unwrap()
    }
}

fn calculate_energy(map: &Map, ray: Ray) -> i32 {
    let mut energized = HashSet::<Vec2>::new();

    let mut open = VecDeque::new();
    open.push_back(ray);

    let mut closed = HashSet::new();


    while let Some(current) = open.pop_front() {
        if !closed.contains(&current) && map.is_in_bounds(&current.position) {
            closed.insert(current);
            energized.insert(current.position);

            let char = map.raw[current.position.y as usize][current.position.x as usize];


            for x in current.next(char) {
                open.push_back(x);
            }
        }
    }

    energized.len() as i32
}

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
struct Ray {
    position: Vec2,
    direction: Vec2,
}

impl Ray {
    pub fn new(position: Vec2, direction: Vec2) -> Self {
        Self { position, direction }
    }

    fn next(&self, c: char) -> Vec<Self> {
        match (c, &self.direction) {
            ('.', _) |
            ('|', &Vec2::NORTH | &Vec2::SOUTH) |
            ('-', &Vec2::EAST | &Vec2::WEST) => vec![self.direction],
            ('/', &Vec2::NORTH) => vec![Vec2::EAST],
            ('/', &Vec2::EAST) => vec![Vec2::NORTH],
            ('/', &Vec2::SOUTH) => vec![Vec2::WEST],
            ('/', &Vec2::WEST) => vec![Vec2::SOUTH],
            ('\\', &Vec2::NORTH) => vec![Vec2::WEST],
            ('\\', &Vec2::EAST) => vec![Vec2::SOUTH],
            ('\\', &Vec2::SOUTH) => vec![Vec2::EAST],
            ('\\', &Vec2::WEST) => vec![Vec2::NORTH],
            ('|', &Vec2::EAST | &Vec2::WEST) => vec![Vec2::NORTH, Vec2::SOUTH],
            ('-', &Vec2::NORTH | &Vec2::SOUTH) => vec![Vec2::EAST, Vec2::WEST],
            _ => panic!()
        }.into_iter()
            .map(|new_direction| Ray::new(self.position + new_direction, new_direction))
            .collect()
    }
}

struct Map {
    raw: Vec<Vec<char>>,
}

impl Map {
    fn new(raw: Vec<Vec<char>>) -> Self {
        Self { raw }
    }

    fn width(&self) -> usize {
        self.raw[0].len()
    }

    fn height(&self) -> usize {
        self.raw.len()
    }

    fn is_in_bounds(&self, vec: &Vec2) -> bool {
        vec.x >= 0 && vec.y >= 0 && vec.x < self.width() as i32 && vec.y < self.height() as i32
    }
}

impl From<&[String]> for Map {
    fn from(value: &[String]) -> Self {
        let raw = value.iter()
            .filter(|l| !l.is_empty())
            .map(|l| l.chars().collect())
            .collect();

        Self::new(raw)
    }
}


const fn v(x: i32, y: i32) -> Vec2 {
    Vec2::new(x, y)
}

const fn v_usize(x: usize, y: usize) -> Vec2 {
    v(x as i32, y as i32)
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
struct Vec2 {
    x: i32,
    y: i32,
}

impl Vec2 {
    pub const NORTH: Self = v(0, -1);
    pub const EAST: Self = v(1, 0);
    pub const SOUTH: Self = v(0, 1);
    pub const WEST: Self = v(-1, 0);

    pub const fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub const fn manhattan_dist(&self, other: &Vec2) -> i32 {
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
