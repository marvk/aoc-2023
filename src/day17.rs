use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::hash::{Hash, Hasher};
use std::ops::{Add, Mul, Sub};

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

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
enum Axis {
    None,
    Vertical,
    Horizontal,
}

impl From<&Vec2> for Axis {
    fn from(value: &Vec2) -> Self {
        match value {
            Vec2 { x: 0, y: 0 } => Axis::None,
            Vec2 { x: _, y: 0 } => Axis::Horizontal,
            Vec2 { x: 0, y: _ } => Axis::Vertical,
            _ => panic!(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Edge {
    axis: Axis,
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
    fn new(axis: Axis, to: Vec2, cost: i16) -> Self {
        Self { axis, to, cost }
    }
}

struct Map {
    edges: HashMap<Vec2, Vec<Edge>>,
    width: usize,
    height: usize,
}

impl Map {
    pub fn new(edges: HashMap<Vec2, Vec<Edge>>, width: usize, height: usize) -> Self {
        Self { edges, width, height }
    }

    fn from(value: &[String], min_step: usize, max_step: usize) -> Self {
        let raw =
            value.iter()
                .filter(|l| !l.is_empty())
                .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect::<Vec<_>>())
                .collect::<Vec<_>>();

        let width = raw[0].len();
        let height = raw.len();

        let mut edge_map = HashMap::new();

        for y in 0..height {
            for x in 0..width {
                let current = v_usize(x, y);

                let edges =
                    Vec2::DIRECTIONS.iter()
                        .flat_map(|e| (min_step as i16..=max_step as i16).map(|step| (*e, step)))
                        .filter_map(|(direction, steps)| {
                            let to = current + direction * steps;

                            if to.x < 0 || to.y < 0 || to.x >= width as i16 || to.y >= height as i16 {
                                None
                            } else {
                                let cost =
                                    (1..=steps)
                                        .map(|step|
                                            raw[(current.y + direction.y * step) as usize][(current.x + direction.x * step) as usize] as i16
                                        )
                                        .sum();

                                Some(Edge::new(Axis::from(&direction), to, cost))
                            }
                        })
                        .collect();

                edge_map.insert(current, edges);
            }
        }

        Map::new(edge_map, width, height)
    }

    fn find_path(&self, start: Vec2, goal: Vec2) -> i16 {
        let start_edge = Edge::new(Axis::None, start, 0);
        let goal_edge = Edge::new(Axis::None, goal, 0);
        let goal_edge_vec = vec![goal_edge];

        let mut open = BinaryHeap::<HeapEdge>::new();
        open.push(HeapEdge(start_edge, 0));

        let mut closed = HashSet::<Edge>::new();

        let mut scores = HashMap::<Edge, i16>::new();
        scores.insert(start_edge, 0);

        loop {
            let current = open.pop().unwrap().0;

            if closed.contains(&current) {
                continue;
            }

            if current == goal_edge {
                return scores[&goal_edge];
            }

            closed.insert(current);

            let previous_axis = current.axis;

            let edges =
                if current.to == goal {
                    &goal_edge_vec
                } else {
                    &self.edges[&current.to]
                }.iter().filter(|&e| e.axis != previous_axis || previous_axis == Axis::None);

            for next in edges {
                let d_score = next.cost;

                let tentative_g_score = scores[&current] + d_score;

                let x = scores.get(next).unwrap_or(&i16::MAX);

                if tentative_g_score < *x {
                    scores.insert(*next, tentative_g_score);

                    open.push(HeapEdge(*next, tentative_g_score));
                }
            }
        }
    }
}

struct HeapEdge(Edge, i16);

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

impl Mul<i16> for Vec2 {
    type Output = Vec2;

    fn mul(self, rhs: i16) -> Self::Output {
        v(self.x * rhs, self.y * rhs)
    }
}
