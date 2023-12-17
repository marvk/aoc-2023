use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::fmt::Display;
use std::hash::{Hash, Hasher};
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
    let goal = v_usize(map.width - 1, map.height - 1);

    let came_from = map.find_path(start, goal);

    came_from as usize
}

struct Map {
    raw: Vec<Vec<char>>,
}

#[derive(Debug, Copy, Clone)]
struct Edge {
    from: Vec2,
    to: Vec2,
    direction: Vec2,
    normalized_direction: Vec2,
    cost: i32,
}

impl Hash for Edge {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.from.hash(state);
        self.to.hash(state);
    }
}

impl Eq for Edge {}

impl PartialEq for Edge {
    fn eq(&self, other: &Self) -> bool {
        self.from.eq(&other.from) && self.to.eq(&other.to)
    }
}

impl Edge {
    fn new(from: Vec2, to: Vec2, direction: Vec2, normalized_direction: Vec2, cost: i32) -> Self {
        Self { from, to, direction, normalized_direction, cost }
    }
}

struct Map2 {
    edges: HashMap<Vec2, Vec<Edge>>,
    width: usize,
    height: usize,
}

impl Map2 {
    fn from(value: &[String], min_step: usize, max_step: usize) -> Self {
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


                            if to.x < 0 || to.y < 0 || to.x >= width as i32 || to.y >= height as i32 {
                                None
                            } else {
                                let cost: u32 =
                                    successors(Some(to), |&e| Some(e - normalized))
                                        .take_while(|&e| e != from)
                                        .map(|e| vec[e.y as usize][e.x as usize])
                                        .sum();

                                Some(Edge::new(from, to, d, normalized, cost as i32))
                            }
                        })
                        .collect();

                result.insert(current, edges);
            }
        }

        Map2 { edges: result, width, height }
    }

    fn find_path(&self, start: Vec2, goal: Vec2) -> i32 {
        let start_edge = Edge::new(start, start, Vec2::default(), Vec2::default(), 0);

        let mut open_set2 = BinaryHeap::<HeapEdge>::new();
        open_set2.push(HeapEdge(&start_edge, 0));

        let goal_edge = Edge::new(goal, goal, Vec2::default(), Vec2::default(), 0);

        let mut came_from = HashMap::<&Edge, &Edge>::new();

        let mut closed_set = HashSet::<&Edge>::new();

        let mut g_scores = HashMap::<&Edge, i32>::new();
        g_scores.insert(&start_edge, 0);

        let goal_edge_vec = vec![goal_edge];

        loop {
            let current = open_set2.pop().unwrap().0;

            if closed_set.contains(current) {
                continue;
            }

            closed_set.insert(current);

            if current == &goal_edge {
                return g_scores[&goal_edge];
            }

            let previous_direction = current.direction;

            let edges =
                if current.to == goal {
                    &goal_edge_vec
                } else {
                    &self.edges[&current.to]
                }.iter()
                    .filter(|&e| (e.direction.x != previous_direction.x && e.direction.y != previous_direction.y) || previous_direction == Vec2::default() || e == &goal_edge);

            for next in edges {
                let d_score = next.cost;

                let tentative_g_score = g_scores[current] + d_score;

                let x = g_scores.get(next).unwrap_or(&i32::MAX);

                if tentative_g_score < *x {
                    came_from.insert(next, current);
                    g_scores.insert(next, tentative_g_score);

                    open_set2.push(HeapEdge(next, tentative_g_score));
                }
            };
        }
    }
}

struct HeapEdge<'a>(&'a Edge, i32);

impl<'a> Eq for HeapEdge<'a> {}

impl<'a> PartialEq<Self> for HeapEdge<'a> {
    fn eq(&self, other: &Self) -> bool {
        other.1.eq(&self.1)
    }
}

impl<'a> PartialOrd<Self> for HeapEdge<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        other.1.partial_cmp(&self.1)
    }
}

impl<'a> Ord for HeapEdge<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        other.1.cmp(&self.1)
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
