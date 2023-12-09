use crate::harness::{Day, Part};

pub fn day09() -> Day<i64, i64> {
    Day::new(9, Box::new(Part1 {}), Box::new(Part2 {}))
}

pub struct Part1;

impl Part<i64> for Part1 {
    fn expect_test(&self) -> i64 {
        114
    }

    fn solve(&self, input: &[String]) -> i64 {
        parse(input).into_iter()
            .map(solve)
            .sum()
    }
}

pub struct Part2;

impl Part<i64> for Part2 {
    fn expect_test(&self) -> i64 {
        2
    }

    fn solve(&self, input: &[String]) -> i64 {
        parse(input).into_iter()
            .map(|mut e| {
                e.reverse();
                e
            })
            .map(solve)
            .sum()
    }
}

fn parse(input: &[String]) -> Vec<Vec<i64>> {
    input.iter()
        .filter(|l| !l.is_empty())
        .map(|l|
            l.split(' ')
                .map(|e| e.parse::<i64>().unwrap())
                .collect()
        )
        .collect()
}

fn solve(input: Vec<i64>) -> i64 {
    let mut history = vec![input];

    while history.last().unwrap().iter().any(|&e| e != 0) {
        let next_step =
            history.last().unwrap().windows(2)
                .map(|e| e[1] - e[0])
                .collect();

        history.push(next_step);
    }

    history.last_mut().unwrap().push(0);

    for i in (1..history.len()).rev() {
        let difference = history[i].last().unwrap();
        let subtrahend = history[i - 1].last().unwrap();
        let minuend = difference + subtrahend;
        history[i - 1].push(minuend);
    }

    *history.first().unwrap().last().unwrap()
}
