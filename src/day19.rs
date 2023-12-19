use std::collections::HashMap;
use std::fmt::Debug;
use std::str::FromStr;

use crate::harness::{Day, Part};

pub fn day19() -> Day<u32, u64> {
    Day::new(19, Box::new(Part1 {}), Box::new(Part2 {}))
}

struct Part1;

impl Part<u32> for Part1 {
    fn expect_test(&self) -> u32 {
        19114
    }

    fn solve(&self, input: &[String]) -> u32 {
        let (workflows, mut machine_parts) = parse(input);

        machine_parts.retain(|machine_part| {
            let mut next = &Next::Workflow("in".to_string());

            while let Next::Workflow(name) = next {
                next = workflows[name].next(machine_part);
            }

            matches!(next, Next::Accept)
        });

        machine_parts.iter().map(|e| e.x + e.m + e.a + e.s).sum()
    }
}

struct Part2;

impl Part<u64> for Part2 {
    fn expect_test(&self) -> u64 {
        167409079868000
    }

    fn solve(&self, input: &[String]) -> u64 {
        solve_part_2(&parse(input).0)
            .iter()
            .map(RecRanges::count_combinations)
            .sum()
    }
}

fn solve_part_2(map: &HashMap<String, Workflow>) -> Vec<RecRanges> {
    solve_part_2_rec(map, &Next::Workflow("in".to_string()), RecRanges::default())
}

#[derive(Clone)]
struct MyRange([bool; 4000]);

impl MyRange {
    pub fn new(min: u32, max_inclusive: u32) -> Self {
        let mut x = [false; 4000];

        for i in min..=max_inclusive {
            x[i as usize - 1] = true;
        }

        Self(x)
    }

    fn count(&self) -> u64 {
        self.0.iter().filter(|&&e| e).count() as u64
    }

    fn except(&mut self, rhs: &Self) {
        for (lhs, rhs) in self.0.iter_mut().zip(&rhs.0) {
            *lhs = *lhs && !rhs
        }
    }

    fn intersect(&mut self, rhs: &Self) {
        for (lhs, &rhs) in self.0.iter_mut().zip(&rhs.0) {
            *lhs = *lhs && rhs
        }
    }
}

impl From<&Rule> for MyRange {
    fn from(rule: &Rule) -> Self {
        match rule.operator {
            Operator::GreaterThan => MyRange::new(rule.value + 1, 4000),
            Operator::LessThan => MyRange::new(1, rule.value - 1),
        }
    }
}

impl Default for MyRange {
    fn default() -> Self {
        Self([true; 4000])
    }
}

#[derive(Clone, Default)]
struct RecRanges {
    x: MyRange,
    m: MyRange,
    a: MyRange,
    s: MyRange,
}

impl RecRanges {
    fn count_combinations(&self) -> u64 {
        self.x.count() * self.m.count() * self.a.count() * self.s.count()
    }

    fn get_range_mut(&mut self, rule: &Rule) -> &mut MyRange {
        match rule.variable {
            'x' => &mut self.x,
            'm' => &mut self.m,
            'a' => &mut self.a,
            's' => &mut self.s,
            _ => panic!(),
        }
    }
}

fn solve_part_2_rec(workflows: &HashMap<String, Workflow>, next: &Next, mut state: RecRanges) -> Vec<RecRanges> {
    match next {
        Next::Accept => vec![state],
        Next::Reject => vec![],
        Next::Workflow(name) => {
            let mut result = vec![];

            let workflow = &workflows[name];

            for rule in &workflow.rules {
                let mut next_state = state.clone();

                let rule_range = rule.into();

                next_state.get_range_mut(rule).intersect(&rule_range);
                state.get_range_mut(rule).except(&rule_range);

                result.append(&mut solve_part_2_rec(workflows, &rule.next, next_state));
            }

            result.append(&mut solve_part_2_rec(workflows, &workflow.on_all_false, state));

            result
        }
    }
}

fn parse(input: &[String]) -> (HashMap<String, Workflow>, Vec<MachinePart>) {
    let mut split = input.split(|e| e.is_empty());

    let workflows =
        split.next().unwrap().iter()
            .map(|e| Workflow::from_str(e).unwrap())
            .map(|e| (e.name.clone(), e))
            .collect::<HashMap<_, _>>();

    let machine_parts =
        split.next().unwrap().iter()
            .map(|e| MachinePart::from_str(e).unwrap())
            .collect::<Vec<_>>();

    (workflows, machine_parts)
}

#[derive(Debug)]
enum Next {
    Accept,
    Reject,
    Workflow(String),
}

impl FromStr for Next {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Self::Accept),
            "R" => Ok(Self::Reject),
            _ => Ok(Self::Workflow(s.to_string()))
        }
    }
}

#[derive(Debug)]
enum Operator {
    GreaterThan,
    LessThan,
}

impl TryFrom<char> for Operator {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '<' => Ok(Self::LessThan),
            '>' => Ok(Self::GreaterThan),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
struct Workflow {
    name: String,
    rules: Vec<Rule>,
    on_all_false: Next,
}

impl Workflow {
    fn new(name: String, rules: Vec<Rule>, on_all_false: Next) -> Self {
        Self { name, rules, on_all_false }
    }

    fn next(&self, machine_part: &MachinePart) -> &Next {
        for rule in &self.rules {
            if rule.check(machine_part) {
                return &rule.next;
            }
        }

        &self.on_all_false
    }
}

impl FromStr for Workflow {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split('{');
        let name = split.next().unwrap();

        let remainder = split.next().unwrap();
        let remainder =
            remainder[0..(remainder.len() - 1)]
                .split(',')
                .collect::<Vec<_>>();

        let rules =
            remainder.iter()
                .rev()
                .skip(1)
                .rev()
                .map(|e| Rule::from_str(e).unwrap())
                .collect::<Vec<_>>();

        let accept =
            remainder
                .last().unwrap()
                .parse().unwrap();

        Ok(Self::new(name.to_string(), rules, accept))
    }
}

#[derive(Debug)]
struct Rule {
    variable: char,
    operator: Operator,
    value: u32,
    next: Next,
}

impl Rule {
    fn new(variable: char, operator: Operator, value: u32, next: Next) -> Self {
        Self { variable, operator, value, next }
    }

    fn check(&self, machine_part: &MachinePart) -> bool {
        let part_value = match self.variable {
            'x' => machine_part.x,
            'm' => machine_part.m,
            'a' => machine_part.a,
            's' => machine_part.s,
            _ => panic!(),
        };

        match self.operator {
            Operator::GreaterThan => part_value > self.value,
            Operator::LessThan => part_value < self.value,
        }
    }
}

impl FromStr for Rule {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars();
        let variable = chars.next().unwrap();
        let condition = chars.next().unwrap().try_into().unwrap();

        let mut split = s[2..].split(':');
        let value = split.next().unwrap().parse().unwrap();
        let next_workflow = split.next().unwrap().parse().unwrap();

        Ok(Self::new(variable, condition, value, next_workflow))
    }
}

#[derive(Debug)]
struct MachinePart {
    x: u32,
    m: u32,
    a: u32,
    s: u32,
}

impl MachinePart {
    fn new(x: u32, m: u32, a: u32, s: u32) -> Self {
        Self { x, m, a, s }
    }
}

impl FromStr for MachinePart {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let vec =
            s[1..(s.len() - 1)]
                .split(',')
                .map(|e| &e[2..])
                .map(|e| e.parse().unwrap())
                .collect::<Vec<_>>();

        Ok(Self::new(vec[0], vec[1], vec[2], vec[3]))
    }
}
