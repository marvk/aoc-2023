use std::collections::HashMap;
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

        let polygon = map.calculate_polygon();

        calculate_area(&polygon) - calculate_trimmings(&polygon)
    }
}

fn calculate_area(polygon: &[Vec2]) -> i32 {
    (0..polygon.len()).map(|i|
        polygon[i].x * polygon[(i + 1) % polygon.len()].y
            - polygon[i].y * polygon[(i + 1) % polygon.len()].x
    ).sum::<i32>().abs() / 2
}

fn calculate_trimmings(polygon: &[Vec2]) -> i32 {
    polygon.len() as i32 / 2 - 1
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

        while let Some(next) =
            self.connections_map[polygon.last().unwrap()].iter()
                .find(|&to| Some(to) != polygon.get(polygon.len().wrapping_sub(2)))
                .filter(|&&next| next != self.start)
        {
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
        let mut connections_map = HashMap::new();
        let mut start = None;

        for (y, s) in value.iter().enumerate() {
            for (x, c) in s.chars().enumerate() {
                let p = v(x as i32, y as i32);

                if c == 'S' {
                    start = Some(p);
                }

                let neighbours = neighbours(c).into_iter().map(|e| e + p).collect::<Vec<_>>();
                connections_map.insert(p, neighbours);
            }
        }

        Map::new(start.unwrap(), connections_map)
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
    pub const EAST: Self = v(1, 0);
    pub const SOUTH: Self = v(0, 1);
    pub const WEST: Self = v(-1, 0);

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
