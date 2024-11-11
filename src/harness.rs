#![allow(dead_code)]

use std::fmt::Debug;
use std::fs;
use std::path::Path;
use std::time::{Duration, Instant};

use colored::Colorize;

pub trait AocResult: Debug + PartialEq + Sized {}

impl<T: Debug + PartialEq> AocResult for T {}

pub trait Part<R: AocResult> {
    fn expect_test(&self) -> R;
    fn solve(&self, input: &[String]) -> R;
}

trait Run<R: AocResult> {
    fn run_test(&self, id: u8, input: &[String]) -> Duration;
    fn run_actual(&self, id: u8, input: &[String]) -> Duration;
    fn run_all(&self, id: u8, test_input: &[String], actual_input: &[String]) -> Duration;
}

impl<T: Part<R> + ?Sized, R: AocResult> Run<R> for T {
    fn run_test(&self, id: u8, input: &[String]) -> Duration {
        let (actual, duration) = timed(|| { self.solve(input) });
        let expected = self.expect_test();
        assert_eq!(actual, expected, "Part {} test failed after {:?}: Expected {:?} but got {:?}", id, duration, expected, actual);
        println!("Part {} test was {} {:>10}", id, "successful".on_bright_green(), format!("{:?}", duration).purple());
        duration
    }

    fn run_actual(&self, id: u8, input: &[String]) -> Duration {
        let (actual, duration) = timed(|| { self.solve(input) });
        println!("{}", format!("Part {} output {:>12} {:>10}", id, format!("{:?}", actual).blue(), format!("{:?}", duration).purple()).on_blue());
        duration
    }

    fn run_all(&self, id: u8, test_input: &[String], actual_input: &[String]) -> Duration {
        self.run_test(id, test_input);
        self.run_actual(id, actual_input)
    }
}

pub struct EmptyPart {}

const NOT_IMPLEMENTED: &str = "NOT_IMPLEMENTED";

impl Part<&str> for EmptyPart {
    fn expect_test(&self) -> &'static str {
        NOT_IMPLEMENTED
    }

    fn solve(&self, _: &[String]) -> &'static str {
        NOT_IMPLEMENTED
    }
}

pub struct Day<R1: AocResult, R2: AocResult> {
    id: u8,
    test_input1: Vec<String>,
    test_input2: Vec<String>,
    actual_input: Vec<String>,
    part1: Box<dyn Part<R1>>,
    part2: Box<dyn Part<R2>>,
}

impl<R1: AocResult + 'static, R2: AocResult + 'static> Day<R1, R2> {
    pub fn new(id: u8, part1: Box<dyn Part<R1>>, part2: Box<dyn Part<R2>>) -> Self {
        Self {
            id,
            test_input1: Self::read_test_input(id, 1),
            test_input2: Self::read_test_input(id, 2),
            actual_input: read_input(format!("input/{:0>2}.txt", id).as_str()),
            part1,
            part2,
        }
    }

    fn read_test_input(id: u8, test_index: usize) -> Vec<String> {
        let test_input_name_with_id = Self::get_test_input_file_name(id, Some(test_index));

        if Path::new(&test_input_name_with_id).is_file() {
            read_input(&test_input_name_with_id)
        } else {
            read_input(&Self::get_test_input_file_name(id, None))
        }
    }

    fn get_test_input_file_name(id: u8, test_id: Option<usize>) -> String {
        format!("input/{:0>2}_test{}.txt", id, test_id.map(|i| i.to_string()).unwrap_or("".to_string()))
    }

    pub fn run_part1_test(&self) -> Duration {
        self.part1.run_test(1, &self.test_input1)
    }

    pub fn run_part2_test(&self) -> Duration {
        self.part2.run_test(2, &self.test_input2)
    }

    pub fn run_part1_actual(&self) -> Duration {
        self.part1.run_actual(1, &self.actual_input)
    }

    pub fn run_part2_actual(&self) -> Duration {
        self.part2.run_actual(2, &self.actual_input)
    }

    pub fn run(&self) -> (Duration, Duration) {
        println!("~~~~~~~~{{ {} }} ~~~~~~~~", format!("Day{:0>2}", self.id).yellow());
        (
            self.part1.run_all(1, &self.test_input1, &self.actual_input),
            self.part2.run_all(2, &self.test_input2, &self.actual_input),
        )
    }

    pub fn f(self) -> DayRunner {
        DayRunner::new(Box::new(move || self.run()))
    }
}

pub struct DayRunner {
    pub f: Box<dyn Fn() -> (Duration, Duration)>,
}

impl DayRunner {
    pub fn new(f: Box<dyn Fn() -> (Duration, Duration)>) -> Self {
        Self { f }
    }
}

fn timed<R: AocResult, F: Fn() -> R>(f: F) -> (R, Duration) {
    let start = Instant::now();
    let result = f();
    (result, start.elapsed())
}

fn read_input(path: &str) -> Vec<String> {
    fs::read_to_string(path).unwrap().split('\n').map(String::from).collect::<Vec<_>>()
}
