use std::collections::{HashMap, VecDeque};
use std::fmt::Debug;
use std::fs::write;

use crate::harness::{Day, Part};

pub fn day20() -> Day<u64, u64> {
    Day::new(20, Box::new(Part1 {}), Box::new(Part2 {}))
}

struct Part1;

impl Part<u64> for Part1 {
    fn expect_test(&self) -> u64 {
        32000000
    }

    fn solve(&self, input: &[String]) -> u64 {
        let mut modules = parse(input);

        let mut low_count = 0;
        let mut high_count = 0;

        let n = 1000;

        for i in 0..n {
            let mut open_list = VecDeque::new();

            open_list.push_back(("button".to_string(), "broadcaster".to_string(), Pulse::Low));

            while let Some((from, to, pulse)) = open_list.pop_front() {
                match pulse {
                    Pulse::High => high_count += 1,
                    Pulse::Low => low_count += 1,
                }

                if let Some(module) = modules.get_mut(&to) {
                    if let Some(next_pulse) = module.receiver.receive_pulse(&from, pulse) {
                        for connection in &module.connections {
                            open_list.push_back((to.clone(), connection.clone(), next_pulse));
                        }
                    }
                }
            }
        }

        low_count * high_count
    }
}

struct Part2;

impl Part<u64> for Part2 {
    fn expect_test(&self) -> u64 {
        0
    }

    fn solve(&self, input: &[String]) -> u64 {
        let mut modules = parse(input);

        if !modules.values().any(|e| e.connections.contains(&"tx".to_string())) {
            return 0;
        }

        let mut tgf = String::new();

        for x in modules.keys() {
            tgf.push_str(x);
            tgf.push(' ');
            tgf.push_str(x);
            tgf.push('\n');
        }

        tgf.push_str("#\n");

        for module in modules.values() {
            for connection in &module.connections {
                tgf.push_str(&module.name);
                tgf.push(' ');
                tgf.push_str(connection);
                tgf.push('\n');
            }
        }

        write("graph.tgf", tgf).unwrap();

        todo!();


        let mut low_count = 0;
        let mut high_count = 0;

        let mut n = 0;

        loop {
            n += 1;
            let mut open_list = VecDeque::new();

            open_list.push_back(("button".to_string(), "broadcaster".to_string(), Pulse::Low));

            while let Some((from, to, pulse)) = open_list.pop_front() {
                match pulse {
                    Pulse::High => high_count += 1,
                    Pulse::Low => low_count += 1,
                }

                if to == "rx" {
                    println!("{} {:?}", n, pulse);

                    if matches!(pulse, Pulse::Low) {
                        return n;
                    }
                }

                if let Some(module) = modules.get_mut(&to) {
                    if let Some(next_pulse) = module.receiver.receive_pulse(&from, pulse) {
                        for connection in &module.connections {
                            open_list.push_back((to.clone(), connection.clone(), next_pulse));
                        }
                    }
                }
            }
        }
    }
}

fn parse(input: &[String]) -> HashMap<String, Module> {
    let map =
        input.iter()
            .filter(|e| !e.is_empty())
            .map(|e| {
                let line = parse_line(e);
                (line.1, (line.0, line.2))
            })
            .collect::<HashMap<_, _>>();

    let map1 =
        map.keys()
            .map(|&key| (key, map.iter().filter(|(_, value)| value.1.contains(&key)).map(|e| *e.0).collect::<Vec<_>>()))
            .collect::<HashMap<_, _>>();


    map.into_iter()
        .map(|(key, (prefix, succesors))| {
            let option = map1.get(key).unwrap().iter().map(|e| e.to_string()).collect();
            let receiver: Box<dyn Receiver> = match prefix {
                None => Box::new(BroadcastReceiver::new()),
                Some('&') => Box::new(ConjunctionReceiver::new(option)),
                Some('%') => Box::new(FlipFlopReceiver::new()),
                _ => panic!(),
            };
            Module::new(key.to_string(), succesors.iter().map(|e| e.to_string()).collect(), receiver)
        })
        .map(|e| (e.name.clone(), e))
        .collect::<HashMap<_, _>>()
}

fn parse_line(line: &str) -> (Option<char>, &str, Vec<&str>) {
    let mut split = line.split("->").map(|e| e.trim());
    let name = split.next().unwrap();

    let x =
        if let Some(end) = name.strip_prefix('%') {
            (Some('%'), end)
        } else if let Some(end) = name.strip_prefix('&') {
            (Some('&'), end)
        } else {
            (None, name)
        };

    let successors = split.next().unwrap().split(',').map(|e| e.trim()).collect();

    (x.0, x.1, successors)
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Pulse {
    High,
    Low,
}

impl Pulse {
    fn flip(&self) -> Pulse {
        match self {
            Pulse::High => Pulse::Low,
            Pulse::Low => Pulse::High,
        }
    }
}

#[derive(Debug)]
struct Module {
    name: String,
    connections: Vec<String>,
    receiver: Box<dyn Receiver>,
}

impl Module {
    pub fn new(name: String, connections: Vec<String>, receiver: Box<dyn Receiver>) -> Self {
        Self { name, connections, receiver }
    }
}

trait Receiver: Debug {
    fn receive_pulse(&mut self, from: &str, pulse: Pulse) -> Option<Pulse>;

    fn reset(&mut self);
}

#[derive(Debug, Clone)]
struct BroadcastReceiver;

impl BroadcastReceiver {
    pub fn new() -> Self {
        Self
    }
}

impl Receiver for BroadcastReceiver {
    fn receive_pulse(&mut self, _: &str, pulse: Pulse) -> Option<Pulse> {
        Some(pulse)
    }

    fn reset(&mut self) {}
}

#[derive(Debug, Clone)]
struct FlipFlopReceiver {
    state: Pulse,
}

impl FlipFlopReceiver {
    pub fn new() -> Self {
        Self { state: Pulse::Low }
    }
}

impl Receiver for FlipFlopReceiver {
    fn receive_pulse(&mut self, _: &str, pulse: Pulse) -> Option<Pulse> {
        match pulse {
            Pulse::High => None,
            Pulse::Low => {
                self.state = self.state.flip();
                Some(self.state)
            }
        }
    }

    fn reset(&mut self) {
        self.state = Pulse::Low;
    }
}

#[derive(Debug, Clone)]
struct ConjunctionReceiver {
    states: HashMap<String, Pulse>,
}

impl ConjunctionReceiver {
    pub fn new(predecessors: Vec<String>) -> Self {
        Self { states: predecessors.into_iter().map(|e| (e, Pulse::Low)).collect() }
    }
}

impl Receiver for ConjunctionReceiver {
    fn receive_pulse(&mut self, from: &str, pulse: Pulse) -> Option<Pulse> {
        self.states.insert(from.to_string(), pulse);

        if self.states.values().all(|&e| e == Pulse::High) {
            Some(Pulse::Low)
        } else {
            Some(Pulse::High)
        }
    }

    fn reset(&mut self) {
        for value in self.states.values_mut() {
            *value = Pulse::Low;
        }
    }
}
