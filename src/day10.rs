use std::collections::{HashMap, HashSet, VecDeque};
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

        map.cull_connections();

        let mut open = VecDeque::new();
        open.push_back((0, map.start));

        let mut closed = HashSet::new();
        let mut result = vec![];

        while !open.is_empty() {
            let (dist_from_start, position) = open.pop_front().unwrap();

            closed.insert(position);

            map.connections_map[&position].iter()
                .cloned()
                .filter(|to| !closed.contains(to))
                .map(|to| (dist_from_start + 1, to))
                .for_each(|next| {
                    open.push_back(next);
                    result.push(next);
                });
        }

        result.iter().map(|(dist, _)| *dist).max().unwrap()
    }
}

pub struct Part2;

impl Part<i32> for Part2 {
    fn expect_test(&self) -> i32 {
        8
    }

    fn solve(&self, input: &[String]) -> i32 {
        let mut map = Map::from(input);

        map.cull_connections();

        let mut polygon = vec![map.start];

        let mut closed = HashSet::new();
        closed.insert(map.start);

        while let Some(next) = map.connections_map[polygon.last().unwrap()].iter().find(|to| !closed.contains(to)) {
            if *next == map.start {
                break;
            }
            polygon.push(*next);
            closed.insert(*next);
        }

        let n = polygon.len();

        let mut sum1 = 0;
        let mut sum2 = 0;

        for i in 0..(n - 1) {
            sum1 += polygon[i].x * polygon[i + 1].y;
            sum2 += polygon[i].y * polygon[i + 1].x;
        }

        sum1 += polygon[n - 1].x * polygon[0].y;
        sum2 += polygon[n - 1].y * polygon[0].x;

        let area = (sum1 - sum2).abs() / 2;

        let direction: i32 = polygon.windows(2).map(|points| {
            let p1 = points[0];
            let p2 = points[1];

            (p2.x - p1.x) * (p2.y + p1.y)
        }).sum();

        if direction > 0 {
            polygon.reverse()
        }

        polygon.push(polygon[0]);
        polygon.push(polygon[1]);

        let trimmings: f64 = polygon.windows(3)
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
            .sum();

        let x1 = area as f64 - trimmings;

        x1 as i32
    }
}

struct Map {
    start: Vec2,
    connections_map: HashMap<Vec2, Vec<Vec2>>,
}

impl Map {
    pub fn new(start: Vec2, connections2: HashMap<Vec2, Vec<Vec2>>) -> Self {
        Self { start, connections_map: connections2 }
    }

    fn num_edges(&self) -> usize {
        self.connections_map.values().map(Vec::len).sum()
    }

    fn cull_connections(&mut self) {
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
        let start = value.iter()
            .enumerate()
            .find_map(|(y, l)| l.chars().enumerate().find_map(|(x, c)| {
                if c == 'S' {
                    Some(v(x as i32, y as i32))
                } else {
                    None
                }
            }))
            .unwrap();

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
