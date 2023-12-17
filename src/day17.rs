use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::fmt::Display;
use std::hash::Hash;
use std::iter::{once, successors};
use std::ops::{Add, Mul, Neg, Sub};
use std::time::{Duration, Instant};

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
            e.to.manhattan_dist(&goal)
        };

        let start_edge = Edge::new(start, start, Vec2::default(), Vec2::default(), 0);
        let start_edge_f_score = h(&start_edge);

        let mut open_set2 = BinaryHeap::<HeapEdge>::new();
        open_set2.push(HeapEdge(start_edge, start_edge_f_score));

        let goal_edge = Edge::new(goal, goal, Vec2::default(), Vec2::default(), 0);

        let mut came_from = HashMap::<Edge, Edge>::new();

        let mut closed_set = HashSet::<Edge>::new();

        let mut g_scores = HashMap::<Edge, i32>::new();
        g_scores.insert(start_edge, 0);

        let mut f_scores = HashMap::<Edge, i32>::new();
        f_scores.insert(start_edge, start_edge_f_score);

        let mut hash_set_search_duration = Duration::ZERO;

        let search_start = Instant::now();

        loop {
            let now = Instant::now();

            let current = open_set2.pop().unwrap().0;

            if closed_set.contains(&current) {
                continue;
            }

            closed_set.insert(current);

            hash_set_search_duration += now.elapsed();

            if current == goal_edge {
                return g_scores[&goal_edge];
            }

            let previous_direction = current.direction;

            let vec: Box<dyn Iterator<Item=_>> =
                if current.to == goal {
                    Box::new(once(&goal_edge))
                } else {
                    Box::new(
                        self.edges[&current.to].iter()
                            .filter(|&e| (e.direction.x != previous_direction.x && e.direction.y != previous_direction.y) || previous_direction == Vec2::default())
                    )
                };

            for &next in vec {
                let d_score = next.cost;

                let tentative_g_score = g_scores[&current] + d_score;

                if tentative_g_score < *g_scores.get(&next).unwrap_or(&i32::MAX) {
                    came_from.insert(next, current);
                    g_scores.insert(next, tentative_g_score);
                    let f_score = tentative_g_score + h(&next);
                    f_scores.insert(next, f_score);

                    open_set2.push(HeapEdge(next, f_score));
                }
            };
        }
    }
}

struct HeapEdge(Edge, i32);

impl Eq for HeapEdge {}

impl PartialEq<Self> for HeapEdge {
    fn eq(&self, other: &Self) -> bool {
        other.1.eq(&self.1)
    }
}

impl PartialOrd<Self> for HeapEdge {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        other.1.partial_cmp(&self.1)
    }
}

impl Ord for HeapEdge {
    fn cmp(&self, other: &Self) -> Ordering {
        other.1.cmp(&self.1)
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
