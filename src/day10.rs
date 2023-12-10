use std::collections::{HashMap, HashSet};
use std::fmt::Display;
use std::ops::{Add, Sub};

use crate::harness::{Day, Part};

pub fn day10() -> Day<i32, i32> {
    Day::new(10, Box::new(Part1 {}), Box::new(Part2 {}))
}

pub struct Part1;

impl Part<i32> for Part1 {
    fn expect_test(&self) -> i32 {
        8
    }

    fn solve(&self, input: &[String]) -> i32 {
        let mut map = Map::from(input);

        map.cull_dead_ends();

        (map.num_edges() / 4) as i32
    }
}

pub struct Part2;

impl Part<i32> for Part2 {
    fn expect_test(&self) -> i32 {
        8
    }

    fn solve(&self, input: &[String]) -> i32 {
        let mut map = Map::from(input);

        map.cull_dead_ends();

        let mut polygon = map.calculate_polygon();

        let area = calculate_area(&polygon);

        if calculate_direction(&polygon) > 0 {
            polygon.reverse()
        }

        polygon.push(polygon[0]);
        polygon.push(polygon[1]);

        let trimmings: f64 = trimmings(&polygon);

        (area as f64 - trimmings) as i32
    }
}

fn calculate_area(polygon: &[Vec2]) -> i32 {
    let n = polygon.len();

    let mut sum1 = 0;
    let mut sum2 = 0;

    for i in 0..(n - 1) {
        sum1 += polygon[i].x * polygon[i + 1].y;
        sum2 += polygon[i].y * polygon[i + 1].x;
    }

    sum1 += polygon[n - 1].x * polygon[0].y;
    sum2 += polygon[n - 1].y * polygon[0].x;

    (sum1 - sum2).abs() / 2
}

fn calculate_direction(polygon: &[Vec2]) -> i32 {
    polygon.windows(2)
        .map(|points| (points[1].x - points[0].x) * (points[1].y + points[0].y))
        .sum()
}

fn trimmings(polygon: &[Vec2]) -> f64 {
    polygon.windows(3)
        .map(|x| {
            let from_direction = x[0] - x[1];
            let to_direction = x[2] - x[1];

            match (from_direction, to_direction) {
                (Vec2::NORTH, Vec2::SOUTH) |
                (Vec2::SOUTH, Vec2::NORTH) |
                (Vec2::EAST, Vec2::WEST) |
                (Vec2::WEST, Vec2::EAST) => 0.5,
                (Vec2::WEST, Vec2::SOUTH) |
                (Vec2::NORTH, Vec2::WEST) |
                (Vec2::EAST, Vec2::NORTH) |
                (Vec2::SOUTH, Vec2::EAST) => 0.25,
                (Vec2::WEST, Vec2::NORTH) |
                (Vec2::NORTH, Vec2::EAST) |
                (Vec2::EAST, Vec2::SOUTH) |
                (Vec2::SOUTH, Vec2::WEST) => 0.75,
                _ => panic!(),
            }
        })
        .sum()
}

struct Map {
    start: Vec2,
    connections_map: HashMap<Vec2, Vec<Vec2>>,
}

impl Map {
    pub fn new(start: Vec2, connections2: HashMap<Vec2, Vec<Vec2>>) -> Self {
        Self { start, connections_map: connections2 }
    }

    fn calculate_polygon(&self) -> Vec<Vec2> {
        let mut polygon = vec![self.start];

        while let Some(next) = self.connections_map[polygon.last().unwrap()].iter().find(|&to| Some(to) != polygon.get(polygon.len().saturating_sub(2))) {
            if *next == self.start {
                break;
            }
            polygon.push(*next);
        }

        polygon
    }

    fn num_edges(&self) -> usize {
        self.connections_map.values().map(Vec::len).sum()
    }

    fn cull_dead_ends(&mut self) {
        loop {
            let previous_size = self.num_edges();

            self.connections_map =
                self.connections_map.clone().into_iter()
                    .map(|(from, tos)| (
                        from,
                        tos.into_iter()
                            .filter(|to|
                                self.connections_map[&from].contains(to) &&
                                    self.connections_map.get(to).map(|l| l.contains(&from)).unwrap_or(false)
                            )
                            .collect()
                    ))
                    .collect();

            if self.num_edges() == previous_size {
                break;
            }
        }
    }
}

impl From<&[String]> for Map {
    fn from(value: &[String]) -> Self {
        let connections_map =
            value.iter()
                .filter(|l| !l.is_empty())
                .enumerate()
                .flat_map(|(y, s)|
                    s.chars().enumerate()
                        .flat_map(move |(x, c)| {
                            let p = v(x as i32, y as i32);
                            neighbours(c).into_iter().map(move |n| (p, n + p))
                        })
                )
                .fold(HashMap::<Vec2, Vec<Vec2>>::new(), |mut acc, e| {
                    acc.entry(e.0).or_insert_with(Vec::new).push(e.1);
                    acc
                });

        let start =
            *connections_map.iter()
                .filter(|(_, v)| v.len() == 4)
                .map(|(k, _)| k)
                .next()
                .unwrap();

        Map::new(start, connections_map)
    }
}

fn neighbours(c: char) -> Vec<Vec2> {
    match c {
        '|' => vec![Vec2::NORTH, Vec2::SOUTH],
        '-' => vec![Vec2::EAST, Vec2::WEST],
        'L' => vec![Vec2::NORTH, Vec2::EAST],
        'J' => vec![Vec2::NORTH, Vec2::WEST],
        '7' => vec![Vec2::SOUTH, Vec2::WEST],
        'F' => vec![Vec2::EAST, Vec2::SOUTH],
        '.' => vec![],
        'S' => vec![Vec2::NORTH, Vec2::EAST, Vec2::SOUTH, Vec2::WEST],
        _ => panic!(),
    }
}


const fn v(x: i32, y: i32) -> Vec2 {
    Vec2::new(x, y)
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
struct Vec2 {
    x: i32,
    y: i32,
}

impl Vec2 {
    pub const NORTH: Self = v(0, -1);
    pub const NORTH_EAST: Self = v(1, -1);
    pub const EAST: Self = v(1, 0);
    pub const SOUTH_EAST: Self = v(1, 1);
    pub const SOUTH: Self = v(0, 1);
    pub const SOUTH_WEST: Self = v(-1, 1);
    pub const WEST: Self = v(-1, 0);
    pub const NORTH_WEST: Self = v(-1, -1);

    pub const DIRECTIONS: [Self; 8] = [Self::NORTH, Self::NORTH_EAST, Self::EAST, Self::SOUTH_EAST, Self::SOUTH, Self::SOUTH_WEST, Self::WEST, Self::NORTH_WEST];

    pub const fn new(x: i32, y: i32) -> Self {
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
