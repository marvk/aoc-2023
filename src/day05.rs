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
        let (seeds, mappings) = parse(input);

        mappings.iter()
            .fold(seeds, |seeds, mapping| {
                seeds.iter()
                    .map(|&num| mapping.convert(num))
                    .collect::<Vec<_>>()
            })
            .into_iter()
            .min()
            .unwrap()
    }
}

pub struct Part2;

impl Part<i64> for Part2 {
    fn expect_test(&self) -> i64 {
        46
    }

    fn solve(&self, input: &[String]) -> i64 {
        let (seeds, maps) = parse(input);

        let flattened_ranges = FlattenedRanges::from(maps.as_slice());
        let seed_ranges = build_seed_ranges(seeds.as_slice());

        seed_ranges
            .into_iter()
            .map(|s| flattened_ranges.min(s))
            .min()
            .unwrap()
    }
}

fn parse(input: &[String]) -> (Vec<i64>, Vec<Mapping>) {
    let mut elements = input.split(|l| l.is_empty()).filter(|arr| !arr.is_empty());

    let seeds = elements.next()
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

    let maps = elements.map(Mapping::from).collect::<Vec<_>>();

    (seeds, maps)
}

struct FlattenedRanges {
    ranges: Vec<(Range<i64>, i64)>,
}

impl FlattenedRanges {
    fn new(ranges: Vec<(Range<i64>, i64)>) -> Self {
        Self { ranges }
    }

    fn min(&self, seed_range: Range<i64>) -> i64 {
        self.ranges.iter()
            .filter_map(|(range, offset)|
                if seed_range.contains(&range.start) {
                    Some(range.start + offset)
                } else if seed_range.contains(&range.end) {
                    Some(seed_range.start + offset)
                } else {
                    None
                }
            )
            .min()
            .unwrap()
    }
}

#[allow(clippy::never_loop)]
impl From<&[Mapping]> for FlattenedRanges {
    fn from(value: &[Mapping]) -> Self {
        let ranges =
            value.iter()
                .fold(vec![(i64::MIN..i64::MAX, 0_i64)], |ranges, map| {
                    ranges.into_iter()
                        .flat_map(|r| split(r, &map.references))
                        .collect::<Vec<_>>()
                });

        Self::new(ranges)
    }
}

fn split(range_and_offset: (Range<i64>, i64), map_ranges: &[MappingRange]) -> Vec<(Range<i64>, i64)> {
    let mut result_from_ranges = vec![];

    let mut result =
        map_ranges.iter()
            .fold(vec![range_and_offset], |result, map_range| {
                result.iter()
                    .flat_map(|(range, offset)| {
                        let offset = *offset;

                        let mapping_start = map_range.source_range_start - offset;
                        let mapping_end = map_range.source_range_start + map_range.range_length - offset;
                        let mapping_offset = map_range.destination_range_start - map_range.source_range_start;

                        if range.contains(&mapping_start) && range.contains(&(mapping_end - 1)) {
                            result_from_ranges.push((mapping_start..mapping_end, offset + mapping_offset));

                            vec![
                                (range.start..mapping_start, offset),
                                (mapping_end..range.end, offset),
                            ]
                        } else if range.contains(&mapping_start) {
                            result_from_ranges.push((mapping_start..range.end, offset + mapping_offset));

                            vec![
                                (range.start..mapping_start, offset),
                            ]
                        } else if range.contains(&(mapping_end - 1)) {
                            result_from_ranges.push((range.start..mapping_end, offset + mapping_offset));

                            vec![
                                (mapping_end..range.end, offset),
                            ]
                        } else if range.start >= mapping_start && range.end <= mapping_end {
                            result_from_ranges.push((range.clone(), offset + mapping_offset));

                            vec![]
                        } else {
                            vec![
                                (range.clone(), offset),
                            ]
                        }
                    })
                    .filter(|(r, _)| !r.is_empty())
                    .collect::<Vec<_>>()
            });


    result.append(&mut result_from_ranges);

    result
}

fn build_seed_ranges(nums: &[i64]) -> Vec<Range<i64>> {
    nums.chunks(2)
        .map(|chunk| chunk[0]..(chunk[0] + chunk[1]))
        .collect::<Vec<_>>()
}

#[derive(Debug)]
struct Mapping {
    references: Vec<MappingRange>,
}

impl Mapping {
    fn new(references: Vec<MappingRange>) -> Self {
        Self { references }
    }

    fn convert(&self, num: i64) -> i64 {
        self.references.iter().filter_map(|r| r.try_convert(num)).next().unwrap_or(num)
    }
}

impl From<&[String]> for Mapping {
    fn from(value: &[String]) -> Self {
        Mapping::new(
            value.iter()
                .skip(1)
                .map(|l| MappingRange::from_str(l).unwrap())
                .collect::<Vec<_>>(),
        )
    }
}

#[derive(Debug)]
struct MappingRange {
    destination_range_start: i64,
    source_range_start: i64,
    range_length: i64,
}

impl MappingRange {
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

impl FromStr for MappingRange {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let vec =
            s.split(' ')
                .map(|s| s.parse::<i64>().unwrap())
                .collect::<Vec<_>>();

        Ok(MappingRange::new(vec[0], vec[1], vec[2]))
    }
}
