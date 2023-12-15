use std::collections::HashMap;
use std::fmt::Display;
use std::hash::Hash;

use crate::harness::{Day, Part};

pub fn day15() -> Day<i32, i32> {
    Day::new(15, Box::new(Part1 {}), Box::new(Part2 {}))
}

pub struct Part1;

impl Part<i32> for Part1 {
    fn expect_test(&self) -> i32 {
        1320
    }

    fn solve(&self, input: &[String]) -> i32 {
        parse(input).iter().map(|&e| hash(e)).sum()
    }
}

pub struct Part2;

impl Part<i32> for Part2 {
    fn expect_test(&self) -> i32 {
        145
    }

    fn solve(&self, input: &[String]) -> i32 {
        let mut map = HashMap::new();

        for i in 0..256 {
            map.insert(i, Vec::<(&str, i32)>::new());
        }

        for x in input.first().unwrap().split(',') {
            if x.contains('-') {
                let str = x.split('-').next().unwrap();
                let b = hash(str);
                if let Some(vec) = map.get_mut(&b) {
                    vec.retain(|e| e.0 != str);
                }
            } else {
                let mut split = x.split("=");
                let a = split.next().unwrap();
                let b = split.next().unwrap();

                let option = map.get_mut(&hash(a)).unwrap();
                let e = (a, b.parse::<i32>().unwrap());
                let mut inserted = false;
                for i in 0..option.len() {
                    let x1 = option[i];
                    if x1.0 == a {
                        option[i] = e;
                        inserted = true;
                    }
                }
                if !inserted {
                    option.push(e);
                }
            }
        }

        for (i, m) in &map {
            if !m.is_empty() {
                println!("a");
                println!("{}: {:?}", i, m);
            }
        }

        map.iter().map(|(i, vec)| {
            (*i + 1) * vec.iter().enumerate().map(|(j, e)| (j as i32 +1) * e.1).sum::<i32>()
        }).sum()
    }
}

fn parse(input: &[String]) -> Vec<&str> {
    input.first().unwrap().split(',').collect()
}

fn hash(str: &str) -> i32 {
    str.chars()
        .map(|c| c as i32)
        .fold(0, |acc, e| ((acc + e) * 17) % 256)
}
