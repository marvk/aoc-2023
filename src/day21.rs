use std::collections::HashSet;
use std::ops::{Add, AddAssign, Mul, Sub};

use crate::harness::{Day, Part};

pub fn day21() -> Day<u64, u64> {
    Day::new(21, Box::new(Part1 {}), Box::new(Part2 {}))
}

struct Part1;

impl Part<u64> for Part1 {
    fn expect_test(&self) -> u64 {
        16
    }

    fn solve(&self, input: &[String]) -> u64 {
        let map = Map::from(input);

        let steps: usize = if map.width() < 12 {
            6
        } else {
            64
        };

        let vec = solve_rec(&map, &mut HashSet::new(), map.starting_position, steps);

        vec.into_iter().collect::<HashSet<_>>().len() as u64
    }
}

fn solve_rec(map: &Map, cache: &mut HashSet<(Vec2, usize)>, current_position: Vec2, steps_remaining: usize) -> Vec<Vec2> {
    if steps_remaining == 0 {
        return vec![current_position];
    }

    let key = (current_position, steps_remaining);

    if cache.contains(&key) {
        return vec![];
    }

    let mut result = vec![];

    for direction in &Vec2::DIRECTIONS {
        let next_position = current_position + *direction;

        if let Some(Tile::Plot) = map.get(&next_position) {
            result.append(&mut solve_rec(map, cache, next_position, steps_remaining - 1));
        }
    }

    cache.insert(key);

    result
}

fn solve_rec2(map: &Map, cache: &mut HashSet<(Vec2, usize)>, current_position: Vec2, steps_remaining: usize) -> Vec<Vec2> {
    if steps_remaining == 0 {
        return vec![current_position];
    }

    let key = (current_position, steps_remaining);

    if cache.contains(&key) {
        return vec![];
    }

    let mut result = vec![];

    for direction in &Vec2::DIRECTIONS {
        let next_position = current_position + *direction;

        if let Tile::Plot = map.get_infinite(&next_position) {
            result.append(&mut solve_rec2(map, cache, next_position, steps_remaining - 1));
        }
    }

    cache.insert(key);

    result
}

struct Part2;

impl Part<u64> for Part2 {
    fn expect_test(&self) -> u64 {
        167004
    }

    fn solve(&self, input: &[String]) -> u64 {
        let map = Map::from(input);

        let steps: usize = if map.width() < 12 {
            500
        } else {
            26501365
        };

        let vec = solve_rec2(&map, &mut HashSet::new(), map.starting_position, steps);

        let set = vec.into_iter().collect::<HashSet<_>>();

        set.len() as u64
    }
}

#[derive(Debug, Copy, Clone)]
enum Tile {
    Rock,
    Plot,
}

impl TryFrom<char> for Tile {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' | 'S' => Ok(Self::Plot),
            '#' => Ok(Self::Rock),
            _ => Err(())
        }
    }
}

struct Map {
    starting_position: Vec2,
    raw: Vec<Vec<Tile>>,
}

impl Map {
    fn width(&self) -> usize {
        self.raw[0].len()
    }

    fn height(&self) -> usize {
        self.raw.len()
    }

    fn get(&self, pos: &Vec2) -> Option<Tile> {
        if pos.y < 0 || pos.x < 0 {
            return None;
        }

        self.raw.get(pos.y as usize).and_then(|e| e.get(pos.x as usize)).copied()
    }

    fn get_infinite(&self, pos: &Vec2) -> Tile {
        let width = self.width() as i32;
        let x = ((pos.x % width) + width) % width;

        let height = self.height() as i32;
        let y = ((pos.y % height) + height) % height;


        self.raw[y as usize][x as usize]
    }
}

impl From<&[String]> for Map {
    fn from(value: &[String]) -> Self {
        let mut starting_position = None;

        let raw =
            value.iter()
                .filter(|l| !l.is_empty())
                .enumerate()
                .map(|(y, l)|
                    l.chars()
                        .enumerate()
                        .map(|(x, c)| {
                            if c == 'S' {
                                starting_position = Some(v(x as i32, y as i32));
                            }

                            c.try_into().unwrap()
                        })
                        .collect()
                )
                .collect();

        let starting_position = starting_position.unwrap();

        Self { raw, starting_position }
    }
}


const fn v(x: i32, y: i32) -> Vec2 {
    Vec2::new(x, y)
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, Default)]
struct Vec2 {
    x: i32,
    y: i32,
}

impl Vec2 {
    const NORTH: Self = v(0, -1);
    const EAST: Self = v(1, 0);
    const SOUTH: Self = v(0, 1);
    const WEST: Self = v(-1, 0);

    const DIRECTIONS: [Self; 4] = [Self::NORTH, Self::EAST, Self::SOUTH, Self::WEST];

    const fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    const fn abs(&self) -> i32 {
        self.x.abs() + self.y.abs()
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

impl Mul<i32> for Vec2 {
    type Output = Vec2;

    fn mul(self, rhs: i32) -> Self::Output {
        v(self.x * rhs, self.y * rhs)
    }
}

impl AddAssign for Vec2 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}
