use std::ops::{Add, Sub};

use crate::harness::{Day, Part};

pub fn day11() -> Day<i64, i64> {
    Day::new(11, Box::new(Part1 {}), Box::new(Part2 {}))
}

pub struct Part1;

impl Part<i64> for Part1 {
    fn expect_test(&self) -> i64 {
        374
    }

    fn solve(&self, input: &[String]) -> i64 {
        solve(parse(input, 2))
    }
}

pub struct Part2;

impl Part<i64> for Part2 {
    fn expect_test(&self) -> i64 {
        82000210
    }

    fn solve(&self, input: &[String]) -> i64 {
        solve(parse(input, 1_000_000))
    }
}

fn solve(galaxies: Vec<Vec2>) -> i64 {
    galaxies[0..galaxies.len() - 1].iter()
        .enumerate()
        .flat_map(|(i1, g1)|
            galaxies.iter()
                .skip(i1)
                .map(|g2| g1.manhattan_dist(g2))
        )
        .sum::<i64>()
}

fn parse(input: &[String], scale: i64) -> Vec<Vec2> {
    let input = input.iter().filter(|l| !l.is_empty()).collect::<Vec<_>>();

    let width = input[0].len();
    let height = input.len();

    let v_expand =
        (0..height)
            .filter(|&y| input[y].chars().all(|c| c == '.'))
            .map(|y| y as i64)
            .collect::<Vec<_>>();

    let h_expand =
        (0..width)
            .filter(|&x| input.iter().all(|line| line.chars().nth(x).unwrap() == '.'))
            .map(|x| x as i64)
            .collect::<Vec<_>>();

    input.iter()
        .enumerate()
        .flat_map(|(y, l)|
            l.chars().enumerate()
                .filter(|(_, c)| *c == '#')
                .map(move |(x, _)| v(x as i64, y as i64))
        )
        .map(|galaxy|
            galaxy + v(
                h_expand.iter().filter(|&&x| x < galaxy.x).count() as i64 * (scale - 1),
                v_expand.iter().filter(|&&y| y < galaxy.y).count() as i64 * (scale - 1),
            )
        )
        .collect()
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
