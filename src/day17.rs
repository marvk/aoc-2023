use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::fmt::Display;
use std::hash::{Hash, Hasher};
use std::iter::successors;
use std::ops::{Add, Mul, Neg, Sub};

use crate::harness::{Day, Part};

pub fn day17() -> Day<u16, u16> {
    Day::new(17, Box::new(Part1 {}), Box::new(Part2 {}))
}

pub struct Part1;

impl Part<u16> for Part1 {
    fn expect_test(&self) -> u16 {
        102
    }

    fn solve(&self, input: &[String]) -> u16 {
        solvify(input, 1, 3)
    }
}

pub struct Part2;

impl Part<u16> for Part2 {
    fn expect_test(&self) -> u16 {
        94
    }

    fn solve(&self, input: &[String]) -> u16 {
        solvify(input, 4, 10)
    }
}

fn solvify(input: &[String], min_step: usize, max_step: usize) -> u16 {
    let map = Map::from(input, min_step, max_step);

    let start = v(0, 0);
    let goal = v_usize(map.width - 1, map.height - 1);

    map.find_path(start, goal) as u16
}

#[derive(Debug, Clone)]
struct Edge {
    axis: Vec2,
    to: Vec2,
    cost: i16,
}

impl Hash for Edge {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.axis.hash(state);
        self.to.hash(state);
    }
}

impl Eq for Edge {}

impl PartialEq for Edge {
    fn eq(&self, other: &Self) -> bool {
        self.axis.eq(&other.axis) && self.to.eq(&other.to)
    }
}

impl Edge {
    fn new(axis: Vec2, to: Vec2, cost: i16) -> Self {
        Self { axis, to, cost }
    }
}

struct Map {
    edges: HashMap<Vec2, Vec<Edge>>,
    width: usize,
    height: usize,
}

impl Map {
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
                        .flat_map(|e| (min_step..=max_step).map(|step| (*e, *e * step as i16)))
                        .filter_map(|(normalized_direction, direction)| {
                            let from = current;
                            let to = current + direction;

                            if to.x < 0 || to.y < 0 || to.x >= width as i16 || to.y >= height as i16 {
                                None
                            } else {
                                let cost =
                                    successors(Some(to), |&e| Some(e - normalized_direction))
                                        .take_while(|&e| e != from)
                                        .map(|e| vec[e.y as usize][e.x as usize] as i16)
                                        .sum();

                                Some(Edge::new(normalized_direction.abs(), to, cost))
                            }
                        })
                        .collect();

                result.insert(current, edges);
            }
        }

        Map { edges: result, width, height }
    }

    fn find_path(&self, start: Vec2, goal: Vec2) -> i16 {
        let start_edge = Edge::new(Vec2::default(), start, 0);
        let goal_edge = Edge::new(Vec2::default(), goal, 0);
        let goal_edge_vec = vec![goal_edge.clone()];

        let mut open_set = BinaryHeap::<HeapEdge>::new();
        open_set.push(HeapEdge(&start_edge, 0));

        let mut closed_set = HashSet::<&Edge>::new();

        let mut g_scores = HashMap::<&Edge, i16>::new();
        g_scores.insert(&start_edge, 0);

        loop {
            let current = open_set.pop().unwrap().0;

            if closed_set.contains(current) {
                continue;
            }

            closed_set.insert(current);

            if current == &goal_edge {
                return g_scores[&goal_edge];
            }

            let previous_axis = current.axis;

            let edges =
                if current.to == goal {
                    &goal_edge_vec
                } else {
                    &self.edges[&current.to]
                }.iter().filter(|&e| e.axis != previous_axis || previous_axis == Vec2::default());

            for next in edges {
                let d_score = next.cost;

                let tentative_g_score = g_scores[current] + d_score;

                let x = g_scores.get(next).unwrap_or(&i16::MAX);

                if tentative_g_score < *x {
                    g_scores.insert(next, tentative_g_score);

                    open_set.push(HeapEdge(next, tentative_g_score));
                }
            };
        }
    }
}

struct HeapEdge<'a>(&'a Edge, i16);

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

const fn v(x: i16, y: i16) -> Vec2 {
    Vec2::new(x, y)
}

const fn v_usize(x: usize, y: usize) -> Vec2 {
    v(x as i16, y as i16)
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, Default)]
struct Vec2 {
    x: i16,
    y: i16,
}

impl Vec2 {
    const NORTH: Self = v(0, -1);
    const EAST: Self = v(1, 0);
    const SOUTH: Self = v(0, 1);
    const WEST: Self = v(-1, 0);

    const DIRECTIONS: [Self; 4] = [Self::NORTH, Self::EAST, Self::SOUTH, Self::WEST];

    const fn new(x: i16, y: i16) -> Self {
        Self { x, y }
    }

    const fn len(&self) -> i16 {
        self.x.abs() + self.y.abs()
    }

    const fn abs(&self) -> Vec2 {
        v(self.x.abs(), self.y.abs())
    }

    const fn manhattan_dist(&self, other: &Vec2) -> i16 {
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

impl Mul<i16> for Vec2 {
    type Output = Vec2;

    fn mul(self, rhs: i16) -> Self::Output {
        v(self.x * rhs, self.y * rhs)
    }
}
