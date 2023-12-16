use std::collections::HashSet;
use std::hash::Hash;
use std::ops::{Add, Neg};

use crate::harness::{Day, Part};

pub fn day16() -> Day<usize, usize> {
    Day::new(16, Box::new(Part1 {}), Box::new(Part2 {}))
}

pub struct Part1;

impl Part<usize> for Part1 {
    fn expect_test(&self) -> usize {
        46
    }

    fn solve(&self, input: &[String]) -> usize {
        Map::from(input)
            .calculate_energy2(Ray::new(v(0, 0), Vec2::EAST))
    }
}

pub struct Part2;

impl Part<usize> for Part2 {
    fn expect_test(&self) -> usize {
        51
    }

    fn solve(&self, input: &[String]) -> usize {
        let map = Map::from(input);

        let i1 =
            (0..(map.width() as i32))
                .flat_map(|x| vec![Ray::new(v(x, 0), Vec2::SOUTH), Ray::new(v(x, map.height() as i32 - 1), Vec2::NORTH)]);

        let i2 =
            (0..(map.height() as i32))
                .flat_map(|y| vec![Ray::new(v(0, y), Vec2::EAST), Ray::new(v(map.width() as i32 - 1, y), Vec2::WEST)]);

        i1.chain(i2)
            .map(|r| map.calculate_energy2(r))
            .max()
            .unwrap()
    }
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

    fn shift(&self, direction: Vec2) -> Self {
        Ray::new(self.position + direction, direction)
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

    fn get(&self, position: &Vec2) -> char {
        self.raw[position.y as usize][position.x as usize]
    }

    fn get2(&self, position: &Vec2) -> Option<&char> {
        self.raw
            .get(position.y as usize)
            .and_then(|vec| vec.get(position.x as usize))
    }

    fn calculate_energy2(&self, ray: Ray) -> usize {
        let mut set = HashSet::new();
        self.calculate_energy2_rec(&mut set, ray);
        set.iter().map(|r| r.position).collect::<HashSet<_>>().len()
    }

    fn calculate_energy2_rec(&self, closed: &mut HashSet<Ray>, ray: Ray) {
        let mut cur_ray = ray;
        let mut cur_char;

        loop {
            cur_char = self.get2(&cur_ray.position);

            if cur_char.is_none() || !closed.insert(cur_ray) {
                return;
            }

            if cur_char.unwrap() != &'.' {
                break;
            }

            cur_ray = cur_ray.shift(cur_ray.direction);
        }

        let ray = cur_ray;
        let char = cur_char.unwrap();

        match (char, &ray.direction) {
            ('.', _) |
            ('|', &Vec2::NORTH | &Vec2::SOUTH) |
            ('-', &Vec2::EAST | &Vec2::WEST) => self.calculate_energy2_rec(closed, ray.shift(ray.direction)),
            ('/', &Vec2::NORTH) | ('\\', &Vec2::SOUTH) => self.calculate_energy2_rec(closed, ray.shift(Vec2::EAST)),
            ('/', &Vec2::EAST) | ('\\', &Vec2::WEST) => self.calculate_energy2_rec(closed, ray.shift(Vec2::NORTH)),
            ('/', &Vec2::SOUTH) | ('\\', &Vec2::NORTH) => self.calculate_energy2_rec(closed, ray.shift(Vec2::WEST)),
            ('/', &Vec2::WEST) | ('\\', &Vec2::EAST) => self.calculate_energy2_rec(closed, ray.shift(Vec2::SOUTH)),
            ('|', &Vec2::EAST | &Vec2::WEST) => {
                self.calculate_energy2_rec(closed, ray.shift(Vec2::NORTH));
                self.calculate_energy2_rec(closed, ray.shift(Vec2::SOUTH));
            }
            ('-', &Vec2::NORTH | &Vec2::SOUTH) => {
                self.calculate_energy2_rec(closed, ray.shift(Vec2::EAST));
                self.calculate_energy2_rec(closed, ray.shift(Vec2::WEST));
            }
            _ => panic!()
        };
    }

    #[allow(dead_code)]
    fn print_visited(&self, set1: &HashSet<Vec2>) {
        for y in 0..self.height() {
            for x in 0..self.width() {
                if set1.contains(&v_usize(x, y)) {
                    print!("#");
                } else {
                    print!("{}", self.get(&v_usize(x, y)));
                }
            }
            println!();
        }
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
}

impl Add<Vec2> for Vec2 {
    type Output = Self;

    fn add(self, rhs: Vec2) -> Self::Output {
        v(self.x + rhs.x, self.y + rhs.y)
    }
}

impl Neg for Vec2 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        v(-self.x, -self.y)
    }
}
