use std::cmp::{max, min};
use std::collections::HashSet;
use std::str::FromStr;
use std::sync::atomic::{AtomicUsize, Ordering};

use crate::harness::{Day, Part};

pub fn day22() -> Day<u32, u32> {
    Day::new(22, Box::new(Part1 {}), Box::new(Part2 {}))
}

struct Part1;

impl Part<u32> for Part1 {
    fn expect_test(&self) -> u32 {
        5
    }

    fn solve(&self, input: &[String]) -> u32 {
        let mut bricks = parse(input);
        drop_bricks(&mut bricks);

        bricks.iter()
            .filter(|&brick| {
                let mut x = bricks.clone();
                x.retain(|e| e != brick);
                is_stable(&mut x)
            })
            .count() as u32
    }
}

struct Part2;

impl Part<u32> for Part2 {
    fn expect_test(&self) -> u32 {
        7
    }

    fn solve(&self, input: &[String]) -> u32 {
        let mut bricks = parse(input);
        drop_bricks(&mut bricks);

        bricks.iter()
            .map(|brick| {
                let mut x = bricks.clone();
                x.retain(|e| e != brick);
                drop_bricks(&mut x)
            })
            .sum::<usize>() as u32
    }
}

fn is_stable(vec: &mut Vec<Cuboid>) -> bool {
    _drop_bricks(vec, true) == 0
}

fn drop_bricks(vec: &mut Vec<Cuboid>) -> usize {
    _drop_bricks(vec, false)
}

fn _drop_bricks(vec: &mut Vec<Cuboid>, short_circuit: bool) -> usize {
    let mut bricks_that_fell = HashSet::new();

    loop {
        let mut any_dropped = false;

        for i in 0..vec.len() {
            if vec[i].min.z > 1 {
                loop {
                    let any_blocking =
                        vec.iter()
                            .filter(|other| other.max.z == vec[i].min.z - 1)
                            .filter(|other| other.id != vec[i].id)
                            .any(|other| vec[i].vertical_projection_overlaps(other));

                    if !any_blocking {
                        any_dropped = true;

                        bricks_that_fell.insert(vec[i].id);
                        vec[i].max.z -= 1;
                        vec[i].min.z -= 1;

                        if short_circuit {
                            return 1;
                        }

                        if vec[i].min.z == 1 {
                            break;
                        }
                    } else {
                        break;
                    }
                }
            }
        }

        if !any_dropped {
            break;
        }
    }

    bricks_that_fell.len()
}


fn parse(input: &[String]) -> Vec<Cuboid> {
    let mut result =
        input.iter()
            .filter(|e| !e.is_empty())
            .map(|e| Cuboid::from_str(e).unwrap())
            .collect::<Vec<_>>();

    result.sort_by_key(|e| e.max.z);

    result
}

static NEXT_CUBOID_ID: AtomicUsize = AtomicUsize::new(0);

#[derive(Debug, Clone, Eq, PartialEq)]
struct Cuboid {
    id: usize,
    min: Vec3,
    max: Vec3,
}

impl Cuboid {
    pub fn new(vec1: Vec3, vec2: Vec3) -> Self {
        let min = v(
            min(vec1.x, vec2.x),
            min(vec1.y, vec2.y),
            min(vec1.z, vec2.z),
        );
        let max = v(
            max(vec1.x, vec2.x),
            max(vec1.y, vec2.y),
            max(vec1.z, vec2.z),
        );

        Self { id: NEXT_CUBOID_ID.fetch_add(1, Ordering::SeqCst), min, max }
    }

    fn vertical_projection_overlaps(&self, rhs: &Cuboid) -> bool {
        max(self.min.x, rhs.min.x) <= min(self.max.x, rhs.max.x) && max(self.min.y, rhs.min.y) <= min(self.max.y, rhs.max.y)
    }
}

impl FromStr for Cuboid {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut vecs =
            s.split('~')
                .map(|g| {
                    let mut coords =
                        g.split(',')
                            .map(|e| e.parse::<i32>().unwrap());

                    v(coords.next().unwrap(), coords.next().unwrap(), coords.next().unwrap())
                });


        Ok(Self::new(vecs.next().unwrap(), vecs.next().unwrap()))
    }
}

const fn v(x: i32, y: i32, z: i32) -> Vec3 {
    Vec3::new(x, y, z)
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, Default)]
struct Vec3 {
    x: i32,
    y: i32,
    z: i32,
}

impl Vec3 {
    const fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }
}
