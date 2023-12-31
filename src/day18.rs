use std::ops::{Add, AddAssign, Mul, Sub};
use std::str::FromStr;

use crate::harness::{Day, Part};

pub fn day18() -> Day<u64, u64> {
    Day::new(18, Box::new(Part1 {}), Box::new(Part2 {}))
}

pub struct Part1;

impl Part<u64> for Part1 {
    fn expect_test(&self) -> u64 {
        62
    }

    fn solve(&self, input: &[String]) -> u64 {
        solve(&parse(input).0)
    }
}

pub struct Part2;

impl Part<u64> for Part2 {
    fn expect_test(&self) -> u64 {
        952408144115
    }

    fn solve(&self, input: &[String]) -> u64 {
        solve(&parse(input).1)
    }
}

fn solve(instructions: &[Vec2]) -> u64 {
    let polygon = build_polygon(instructions);
    calculate_area(&polygon)
}

fn build_polygon(instructions: &[Vec2]) -> Vec<Vec2> {
    instructions.iter()
        .scan(v(0, 0), |current, &instruction| {
            *current += instruction;
            Some(*current)
        })
        .collect::<Vec<_>>()
}

fn calculate_area(polygon: &[Vec2]) -> u64 {
    (0..polygon.len()).map(|i| {
        let p1 = polygon[i];
        let p2 = polygon[(i + 1) % polygon.len()];

        p1.x * p2.y -
            p1.y * p2.x +
            (p1 - p2).abs()
    }).sum::<i64>().unsigned_abs() / 2 + 1
}

fn parse(input: &[String]) -> (Vec<Vec2>, Vec<Vec2>) {
    input.iter()
        .filter(|l| !l.is_empty())
        .map(|l| parse_line(l))
        .unzip()
}

fn parse_line(line: &str) -> (Vec2, Vec2) {
    let mut split = line.split(' ');

    let direction = Vec2::from_str(split.next().unwrap()).unwrap();
    let distance = split.next().unwrap().parse::<i64>().unwrap();

    let color =
        split.next().unwrap()[2..8].chars()
            .map(|e| e.to_digit(16).unwrap())
            .rev()
            .enumerate()
            .map(|(idx, e)| 16_usize.pow(idx as u32) * e as usize)
            .sum::<usize>();

    let distance2 = (color >> 4) as i64;
    let direction2 = Vec2::DIRECTIONS[color % 16];

    (
        direction * distance,
        direction2 * distance2,
    )
}

const fn v(x: i64, y: i64) -> Vec2 {
    Vec2::new(x, y)
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, Default)]
struct Vec2 {
    x: i64,
    y: i64,
}

impl Vec2 {
    const NORTH: Self = v(0, -1);
    const EAST: Self = v(1, 0);
    const SOUTH: Self = v(0, 1);
    const WEST: Self = v(-1, 0);

    const DIRECTIONS: [Self; 4] = [Self::NORTH, Self::EAST, Self::SOUTH, Self::WEST];

    const fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }

    const fn abs(&self) -> i64 {
        self.x.abs() + self.y.abs()
    }
}

impl FromStr for Vec2 {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "U" => Ok(Vec2::NORTH),
            "R" => Ok(Vec2::EAST),
            "D" => Ok(Vec2::SOUTH),
            "L" => Ok(Vec2::WEST),
            _ => Err(())
        }
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

impl Mul<i64> for Vec2 {
    type Output = Vec2;

    fn mul(self, rhs: i64) -> Self::Output {
        v(self.x * rhs, self.y * rhs)
    }
}

impl AddAssign for Vec2 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}
