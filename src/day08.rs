use std::collections::HashMap;
use std::str::FromStr;

use crate::harness::{Day, Part};

pub fn day08() -> Day<u64, u64> {
    Day::new(8, Box::new(Part1 {}), Box::new(Part2 {}))
}

pub struct Part1;

impl Part<u64> for Part1 {
    fn expect_test(&self) -> u64 {
        2
    }

    fn solve(&self, input: &[String]) -> u64 {
        let Map { instructions, nodes } = Map::from(input);

        let mut current = "AAA";

        let mut step = 0;

        loop {
            current = nodes.get(current).unwrap().neighbour(&instructions[step % instructions.len()]);

            step += 1;

            if current == "ZZZ" {
                break;
            }
        }

        step as u64
    }
}

pub struct Part2;

impl Part<u64> for Part2 {
    fn expect_test(&self) -> u64 {
        6
    }

    fn solve(&self, input: &[String]) -> u64 {
        let Map { instructions, nodes } = Map::from(input);

        let mut currents =
            nodes.keys()
                .map(|name| name.as_str())
                .filter(|name| name.ends_with('A'))
                .collect::<Vec<_>>();

        let mut end_steps = HashMap::<usize, usize>::new();

        let mut step = 0;

        loop {
            currents =
                currents.into_iter()
                    .map(|name|
                        nodes.get(name).unwrap().neighbour(&instructions[step % instructions.len()])
                    )
                    .collect::<Vec<_>>();

            step += 1;

            for (idx, name) in currents.iter().enumerate() {
                if name.ends_with('Z') {
                    end_steps.entry(idx).or_insert(step);
                }
            }

            if end_steps.len() == currents.len() {
                break;
            }
        }

        lcm(
            &end_steps.iter()
                .map(|(_, &num)| num as u64)
                .collect::<Vec<_>>()
        )
    }
}

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        return a;
    }
    gcd(b, a % b)
}

fn lcm(nums: &[u64]) -> u64 {
    nums.iter()
        .copied()
        .reduce(|acc, e| (e * acc) / gcd(e, acc))
        .unwrap()
}

#[derive(Debug)]
enum Instruction {
    Left,
    Right,
}

impl TryFrom<char> for Instruction {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'L' => Ok(Instruction::Left),
            'R' => Ok(Instruction::Right),
            _ => Err(())
        }
    }
}

#[derive(Debug)]
struct Map {
    instructions: Vec<Instruction>,
    nodes: HashMap<String, Node>,
}

impl Map {
    fn new(instructions: Vec<Instruction>, nodes: HashMap<String, Node>) -> Self {
        Self { instructions, nodes }
    }
}

impl From<&[String]> for Map {
    fn from(value: &[String]) -> Self {
        let instructions =
            value[0].chars()
                .map(|c| Instruction::try_from(c).unwrap())
                .collect::<Vec<_>>();


        let nodes =
            value[2..].iter()
                .filter(|l| !l.is_empty())
                .map(|l| Node::from_str(l).unwrap())
                .map(|n| (n.name.clone(), n))
                .collect::<HashMap<_, _>>();

        Map::new(instructions, nodes)
    }
}

#[derive(Debug)]
struct Node {
    name: String,
    left_neighbour: String,
    right_neighbour: String,
}

impl Node {
    fn new(name: String, left_neighbour: String, right_neighbour: String) -> Self {
        Self { name, left_neighbour, right_neighbour }
    }

    fn neighbour(&self, instruction: &Instruction) -> &str {
        match instruction {
            Instruction::Left => &self.left_neighbour,
            Instruction::Right => &self.right_neighbour,
        }
    }
}

impl FromStr for Node {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Node::new(
            s[0..3].to_string(),
            s[7..10].to_string(),
            s[12..15].to_string(),
        ))
    }
}
