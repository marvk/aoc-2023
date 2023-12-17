use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::fmt::Display;
use std::hash::Hash;
use std::iter::successors;
use std::ops::{Add, Mul, Neg, Sub};

use crate::harness::{Day, Part};

pub fn day17() -> Day<usize, usize> {
    Day::new(17, Box::new(Part1 {}), Box::new(Part2 {}))
}

pub struct Part1;

impl Part<usize> for Part1 {
    fn expect_test(&self) -> usize {
        102
    }

    fn solve(&self, input: &[String]) -> usize {
        solvify(input, 1, 3)
    }
}

pub struct Part2;

impl Part<usize> for Part2 {
    fn expect_test(&self) -> usize {
        94
    }

    fn solve(&self, input: &[String]) -> usize {
        solvify(input, 4, 10)
    }
}

fn solvify(input: &[String], min_step: usize, max_step: usize) -> usize {
    let map = Map2::from(input, min_step, max_step);

    let option = map.edges.get(&v(0, 0));

    let start = v(0, 0);
    let goal = v_usize(map.width() - 1, map.height() - 1);

    let came_from = map.find_path(start, goal);

    came_from as usize
}

struct Map {
    raw: Vec<Vec<char>>,
}

struct Node(Vec2, i32);

impl Eq for Node {}

impl PartialEq<Self> for Node {
    fn eq(&self, other: &Self) -> bool {
        other.1.eq(&self.1)
    }
}

impl PartialOrd<Self> for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        other.1.partial_cmp(&self.1)
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other.1.cmp(&self.1)
    }
}

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone)]
struct Edge {
    from: Vec2,
    to: Vec2,
    direction: Vec2,
    normalized_direction: Vec2,
    cost: i32,
}

impl Edge {
    fn new(from: Vec2, to: Vec2, direction: Vec2, normalized_direction: Vec2, cost: i32) -> Self {
        Self { from, to, direction, normalized_direction, cost }
    }
}

struct Map2 {
    edges: HashMap<Vec2, Vec<Edge>>,
    raw: Vec<Vec<char>>,
}


impl Map2 {
    fn from(value: &[String], min_step: usize, max_step: usize) -> Self {
        let map = Map::from(value);

        let vec =
            value.iter()
                .filter(|l| !l.is_empty())
                .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect::<Vec<_>>())
                .collect::<Vec<_>>();

        let width = vec[0].len();
        let height = vec.len();

        let mut result = HashMap::new();

        for y in 0..height {
            for x in 0..width {
                let current = v_usize(x, y);

                let edges =
                    Vec2::DIRECTIONS.iter()
                        .flat_map(|e| (min_step..=max_step).map(|step| (*e, *e * step as i32)))
                        .filter_map(|(normalized, d)| {
                            let from = current;
                            let to = current + d;

                            let cost: u32 =
                                successors(Some(to), |&e| Some(e - normalized))
                                    .take_while(|&e| e != from)
                                    .filter_map(|e| map.get(&e))
                                    .sum();

                            if cost == 0 || map.get(&to).is_none() {
                                None
                            } else {
                                Some(Edge::new(from, to, d, normalized, cost as i32))
                            }
                        })
                        .collect::<Vec<_>>();

                result.insert(current, edges);
            }
        }

        Map2 { edges: result, raw: map.raw }
    }


    fn width(&self) -> usize {
        self.raw[0].len()
    }

    fn height(&self) -> usize {
        self.raw.len()
    }

    fn get(&self, position: &Vec2) -> Option<u32> {
        self.raw
            .get(position.y as usize)
            .and_then(|vec| vec.get(position.x as usize))
            .map(|e| e.to_digit(10).unwrap())
    }

    fn find_path(&self, start: Vec2, goal: Vec2) -> i32 {
        let h = |e: &Edge| {
            if e.direction.len() != 0 {
                e.to.manhattan_dist(&goal) + e.cost / e.direction.len()
            } else {
                e.to.manhattan_dist(&goal)
            }
        };

        let goal_edge = Edge::new(goal, goal, Vec2::default(), Vec2::default(), 0);

        let mut came_from = HashMap::<Edge, Edge>::new();

        let mut open_set = HashSet::<Edge>::new();
        let start_edge = Edge::new(start, start, Vec2::default(), Vec2::default(), 0);
        open_set.insert(start_edge);

        let mut g_score = HashMap::<Edge, i32>::new();
        g_score.insert(start_edge, 0);

        let mut f_score = HashMap::<Edge, i32>::new();
        f_score.insert(start_edge, h(&start_edge));

        let mut iterations = 1;

        let mut max = v(0, 0);

        loop {
            let current = open_set.iter().min_by_key(|&e| f_score[e]).copied();


            if current.is_none() {
                panic!();
            }

            let current = current.unwrap();
            open_set.remove(&current);

            if current.to.len() > max.len() {
                max = current.to;
                println!("{:?}", max);
            }

            if current == goal_edge {
                let option = g_score.iter().find(|e| e.0 == &goal_edge);

                dbg!(option.unwrap().1);

                // for x in &g_score {
                //     println!("{:?}", x);
                // }

                println!("{}", "~".repeat(100));
                println!("{}", "~".repeat(100));
                println!("{}", "~".repeat(100));

                return *option.unwrap().1;
            }

            let previous_direction = current.direction;

            let vec = &self.edges[&current.to];
            if (current.to == goal) {
                let next = goal_edge;

                let d_score = next.cost;

                let tentative_g_score = g_score[&current] + d_score;

                if tentative_g_score < *g_score.get(&next).unwrap_or(&i32::MAX) {
                    came_from.insert(next, current);
                    g_score.insert(next, tentative_g_score);
                    f_score.insert(next, tentative_g_score + h(&next));

                    open_set.insert(next);
                }
            } else {
                for &next in vec.iter()
                    .filter(|&e| (e.direction.x != previous_direction.x && e.direction.y != previous_direction.y) || previous_direction == Vec2::default()) {
                    let d_score = next.cost;

                    let tentative_g_score = g_score[&current] + d_score;

                    if tentative_g_score < *g_score.get(&next).unwrap_or(&i32::MAX) {
                        came_from.insert(next, current);
                        g_score.insert(next, tentative_g_score);
                        f_score.insert(next, tentative_g_score + h(&next));

                        open_set.insert(next);
                    }
                };
            }


            // println!("came_from");
            // for x in &came_from {
            //     println!("{:?}", x);
            // }
            // println!("open_set");
            // for x in &open_set {
            //     println!("{:?}", x);
            // }
            // println!("g_score");
            // for x in &g_score {
            //     println!("{:?}", x);
            // }
            // println!("f_score");
            // let mut vec1 = f_score.iter().collect::<Vec<_>>();
            // vec1.sort_by_key(|e| e.1);
            // for x in &vec1 {
            //     println!("{:?}", x);
            // }

            // if iterations == 2 {
            //     panic!();
            // }
            //
            // iterations += 1;
        }
    }
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

    fn get(&self, position: &Vec2) -> Option<u32> {
        self.raw
            .get(position.y as usize)
            .and_then(|vec| vec.get(position.x as usize))
            .map(|e| e.to_digit(10).unwrap())
    }

    fn find_path(&self, start: Vec2, goal: Vec2) -> HashMap<Vec2, Vec2> {
        let h = |v: &Vec2| 100000;// v.manhattan_dist(&goal);
        let d = |v: &Vec2| self.get(v);

        let mut came_from = HashMap::<Vec2, Vec2>::new();

        // let mut open_set = BinaryHeap::<Node>::new();
        // open_set.push(Node(start, h(&start)));

        let mut open_set = HashSet::<Vec2>::new();
        open_set.insert(start);

        let mut g_score = HashMap::<Vec2, i32>::new();
        g_score.insert(start, self.get(&start).unwrap() as i32);

        let mut f_score = HashMap::<Vec2, i32>::new();
        f_score.insert(start, h(&start));

        loop {
            let current = open_set.iter().min_by_key(|&e| f_score[e]).copied();

            if current.is_none() {
                break;
            }


            let current = current.unwrap();
            open_set.remove(&current);

            if current == goal {
                return came_from;
            }

            let can_go_straight = false;
            let current_pos = current;
            let previous_pos_1 = came_from.get(&current_pos);
            let previous_pos_2 = previous_pos_1.and_then(|e| came_from.get(e));
            let previous_pos_3 = previous_pos_2.and_then(|e| came_from.get(e));

            let previous_direction_1 = previous_pos_1.map(|&e1| current_pos - e1).unwrap_or_default();
            let previous_direction_2 = previous_pos_1.and_then(|&e1| previous_pos_2.map(|&e2| e1 - e2)).unwrap_or_default();
            let previous_direction_3 = previous_pos_2.and_then(|&e2| previous_pos_3.map(|&e3| e2 - e3)).unwrap_or_default();

            let mut x = Vec2::DIRECTIONS.to_vec();
            x.retain(|&e| e != -previous_direction_1);

            if previous_direction_1 == previous_direction_2 && previous_direction_2 == previous_direction_3 && previous_direction_1 != Vec2::default() {
                x.retain(|&e| e != previous_direction_1);
            }

            for direction in x {
                let neighbour = current_pos + direction;

                if let Some(d_score) = d(&neighbour) {
                    let tentative_g_score = g_score[&current_pos] + d_score as i32;

                    if tentative_g_score < *g_score.get(&neighbour).unwrap_or(&i32::MAX) {
                        came_from.insert(neighbour, current_pos);
                        g_score.insert(neighbour, tentative_g_score);
                        f_score.insert(neighbour, tentative_g_score + h(&neighbour));

                        open_set.insert(neighbour);
                    }
                }
            };
        }


        todo!()
    }
}

impl From<&[String]> for Map {
    fn from(value: &[String]) -> Self {
        let vec = value.iter()
            .filter(|l| !l.is_empty())
            .map(|l| l.chars().collect())
            .collect();

        Map::new(vec)
    }
}

const fn v(x: i32, y: i32) -> Vec2 {
    Vec2::new(x, y)
}

const fn v_usize(x: usize, y: usize) -> Vec2 {
    v(x as i32, y as i32)
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, Default)]
struct Vec2 {
    x: i32,
    y: i32,
}

impl Vec2 {
    pub const NORTH: Self = v(0, -1);
    pub const EAST: Self = v(1, 0);
    pub const SOUTH: Self = v(0, 1);
    pub const WEST: Self = v(-1, 0);

    pub const DIRECTIONS: [Self; 4] = [Self::NORTH, Self::EAST, Self::SOUTH, Self::WEST];

    pub const fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub const fn len(&self) -> i32 {
        self.x.abs() + self.y.abs()
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

impl Neg for Vec2 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        v(-self.x, -self.y)
    }
}

impl Mul<i32> for Vec2 {
    type Output = Vec2;

    fn mul(self, rhs: i32) -> Self::Output {
        v(self.x * rhs, self.y * rhs)
    }
}
