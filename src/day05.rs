#![allow(warnings, unused)]

use std::cmp::min;
use std::ops::Range;
use std::str::FromStr;

use crate::harness::{Day, Part};

pub fn day05() -> Day<i64, i64> {
    Day::new(5, Box::new(Part1 {}), Box::new(Part2 {}))
}

pub struct Part1;

impl Part<i64> for Part1 {
    fn expect_test(&self) -> i64 {
        35
    }

    fn solve(&self, input: &[String]) -> i64 {
        let (mut seeds, maps) = parse(input);

        for map in maps {
            seeds = seeds.iter().map(|&num| map.convert(num)).collect::<Vec<_>>()
        }

        seeds.into_iter().min().unwrap()
    }
}

pub struct Part2;

impl Part<i64> for Part2 {
    fn expect_test(&self) -> i64 {
        46
    }

    fn solve(&self, input: &[String]) -> i64 {
        let (seeds, maps) = parse(input);
        let converter = Converter::from(maps.as_slice());
        let seed_ranges = build_seed_ranges(seeds.as_slice());

        seed_ranges
            .into_iter()
            .map(|s| converter.min(s))
            .min()
            .unwrap()
    }
}

fn parse(input: &[String]) -> (Vec<i64>, Vec<Map>) {
    let mut split = input.split(|l| l.is_empty()).filter(|arr| !arr.is_empty());


    let option = split.next();
    let seeds = option
        .unwrap()
        .first()
        .unwrap()
        .split(':')
        .last()
        .unwrap()
        .trim()
        .split(' ')
        .map(|s| s.parse::<i64>().unwrap())
        .collect::<Vec<_>>();

    let vec = split.map(Map::from).collect::<Vec<_>>();

    (seeds, vec)
}

struct Converter {
    ranges: Vec<(Range<i64>, i64)>,
}

impl Converter {
    fn new(ranges: Vec<(Range<i64>, i64)>) -> Self {
        Self { ranges }
    }

    fn min(&self, seed_range: Range<i64>) -> i64 {
        let mut result = i64::MAX;

        for (range, offset) in &self.ranges {
            let seed_range_start = seed_range.start;
            let seed_range_end_exclusive = seed_range.end;
            let seed_range_end_inclusive = seed_range.end - 1;

            if seed_range.contains(&range.start) {
                result = min(result, range.start + offset);
            } else if seed_range.contains(&range.end) {
                result = min(result, seed_range.start + offset)
            }

        }

        result
    }
}

#[allow(clippy::never_loop)]
impl From<&[Map]> for Converter {
    fn from(value: &[Map]) -> Self {
        let start = -(u32::MAX as i64) * 10;
        let end = (u32::MAX as i64) * 10;
        let mut ranges = vec![(start..end, 0_i64)];



        for map in value {
            ranges = ranges.into_iter().flat_map(|r| split(r, &map.references)).collect::<Vec<_>>();
        }

        Self::new(ranges)
    }
}

fn split(range_and_offset: (Range<i64>, i64), map_ranges: &[MapRange]) -> Vec<(Range<i64>, i64)> {
    let mut result = vec![range_and_offset];

    let mut result_from_ranges = vec![];

    for map_range in map_ranges {
        let map_range_start = map_range.source_range_start;
        let map_range_end_exclusive = map_range.source_range_start + map_range.range_length;
        let map_range_end_inclusive = map_range_end_exclusive - 1;
        let map_range_offset = map_range.destination_range_start - map_range_start;

        result =
            result.iter()
                .flat_map(|(range, offset)| {
                    let test_range = (range.start + offset)..(range.end + offset);

                    let r = if test_range.contains(&map_range_start) && test_range.contains(&map_range_end_inclusive) {
                        result_from_ranges.push((map_range_start - *offset..map_range_end_exclusive - *offset, *offset + map_range_offset));

                        vec![
                            (test_range.start..map_range_start, *offset),
                            (map_range_end_exclusive..test_range.end, *offset),
                        ]
                    } else if test_range.contains(&map_range_start) {
                        result_from_ranges.push((map_range_start - *offset..test_range.end - *offset, *offset + map_range_offset));

                        vec![
                            (test_range.start..map_range_start, *offset),
                        ]
                    } else if test_range.contains(&map_range_end_inclusive) {
                        result_from_ranges.push((test_range.start - *offset..map_range_end_exclusive - *offset, *offset + map_range_offset));

                        vec![
                            (map_range_end_exclusive..test_range.end, *offset),
                        ]
                    } else if test_range.start >= map_range_start && test_range.end <= map_range_end_exclusive {
                        result_from_ranges.push((test_range.start - *offset..test_range.end - *offset, *offset + map_range_offset));

                        vec![]
                    } else {
                        vec![
                            (test_range.clone(), *offset),
                        ]
                    };

                    r.into_iter().map(|(r, o)| (r.start - *offset..r.end - *offset, o))
                })
                .filter(|(r, _)| !r.is_empty())
                .collect::<Vec<_>>();
    }

    result.append(&mut result_from_ranges);

    result.sort_by_key(|r| r.0.start);

    result
}

fn build_seed_ranges(nums: &[i64]) -> Vec<Range<i64>> {
    nums.chunks(2)
        .map(|chunk| chunk[0]..(chunk[0] + chunk[1]))
        .collect::<Vec<_>>()
}

#[derive(Debug)]
struct Map {
    from: String,
    to: String,
    references: Vec<MapRange>,
}

impl Map {
    fn new(from: String, to: String, references: Vec<MapRange>) -> Self {
        Self { from, to, references }
    }

    fn convert(&self, num: i64) -> i64 {
        self.references.iter().filter_map(|r| r.try_convert(num)).next().unwrap_or(num)
    }
}

impl From<&[String]> for Map {
    fn from(value: &[String]) -> Self {
        let mut names =
            value.first()
                .unwrap()
                .split(' ')
                .next()
                .unwrap()
                .split('-');

        Map::new(
            names.next().unwrap().to_string(),
            names.next_back().unwrap().to_string(),
            value.iter()
                .skip(1)
                .map(|l| MapRange::from_str(l).unwrap())
                .collect::<Vec<_>>(),
        )
    }
}

#[derive(Debug)]
struct MapRange {
    destination_range_start: i64,
    source_range_start: i64,
    range_length: i64,
}

impl MapRange {
    pub fn new(destination_range_start: i64, source_range_start: i64, range_length: i64) -> Self {
        Self { destination_range_start, source_range_start, range_length }
    }

    fn try_convert(&self, num: i64) -> Option<i64> {
        let distance_from_start = num - self.source_range_start;


        if distance_from_start >= 0 && distance_from_start < self.range_length {
            Some(self.destination_range_start + distance_from_start)
        } else {
            None
        }
    }
}

impl FromStr for MapRange {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let vec =
            s.split(' ')
                .map(|s| s.parse::<i64>().unwrap())
                .collect::<Vec<_>>();

        Ok(MapRange::new(vec[0], vec[1], vec[2]))
    }
}
