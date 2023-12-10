use std::collections::{HashSet, VecDeque};
use std::fmt::{Display, Formatter};
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

            map.connections.iter()
                .filter(|(from, _)| *from == position)
                .map(|(_, to)| *to)
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

        while let Some((_, next)) = map.connections.iter().filter(|(from, to)| !closed.contains(to)).find(|(from, to)| from == polygon.last().unwrap()) {
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

        let trimmings: f64 = polygon.windows(3).map(|x| {
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
        }).sum();

        let x1 = area as f64 - trimmings;

        x1 as i32
    }
}

struct Map {
    start: Vec2,
    connections: Vec<(Vec2, Vec2)>,
}

impl Map {
    fn new(start: Vec2, connections: Vec<(Vec2, Vec2)>) -> Self {
        Self { start, connections }
    }

    fn cull_connections(&mut self) {
        loop {
            let previous_size = self.connections.len();

            self.connections =
                self.connections.clone().into_iter()
                    .filter(|(from, to)|
                        self.connections.iter()
                            .any(|(other_from, other_to)| to == other_from && from == other_to)
                    )
                    .collect::<Vec<_>>();

            if self.connections.len() == previous_size {
                break;
            }
        }
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let vec1 = self.connections.iter().flat_map(|e| vec![e.0, e.1]).collect::<HashSet<_>>();

        let min_x = vec1.iter().map(|p| p.x).min().unwrap();
        let min_y = vec1.iter().map(|p| p.y).min().unwrap();
        let max_x = vec1.iter().map(|p| p.x).max().unwrap();
        let max_y = vec1.iter().map(|p| p.y).max().unwrap();

        let min = v(min_x, min_y);
        let max = v(max_x, max_y);

        // let width = max_x - min_x;

        todo!()
    }
}

impl From<&[String]> for Map {
    fn from(value: &[String]) -> Self {
        let connections = value.iter()
            .filter(|l| !l.is_empty())
            .enumerate()
            .flat_map(|(y, s)|
                s.chars().enumerate().flat_map(move |(x, c)| {
                    let p = v(x as i32, y as i32);
                    neighbours(c).into_iter().map(move |n| (p, n + p))
                })
            ).collect::<Vec<_>>();

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

        Map::new(start, connections)
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
