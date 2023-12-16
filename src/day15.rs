use std::collections::HashMap;

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
        let mut map = HashMap::<i32, Vec<(&str, i32)>>::new();

        for element in input.first().unwrap().split(',') {
            if element.contains('-') {
                let name = element.split('-').next().unwrap();
                let hash_number = hash(name);

                let bucket = map.entry(hash_number).or_default();

                bucket.retain(|e| e.0 != name);
            } else {
                let mut split = element.split('=');

                let name = split.next().unwrap();
                let hash_number = hash(name);
                let number = split.next().unwrap();

                let bucket = map.entry(hash_number).or_default();

                let new_element = (name, number.parse::<i32>().unwrap());
                let mut inserted = false;
                for i in 0..bucket.len() {
                    if bucket[i].0 == name {
                        bucket[i] = new_element;
                        inserted = true;
                        break;
                    }
                }
                if !inserted {
                    bucket.push(new_element);
                }
            }
        }

        map.iter()
            .map(|(bucket_index, bucket)|
                (*bucket_index + 1) * bucket.iter()
                    .enumerate()
                    .map(|(element_index, element)| (element_index as i32 + 1) * element.1)
                    .sum::<i32>()
            )
            .sum()
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
