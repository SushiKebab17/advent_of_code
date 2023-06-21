use search::bft;
use std::collections::{HashMap, HashSet};
use std::time::Instant;
// let now = Instant::now();
// let elapsed = now.elapsed().as_micros();
// println!("{}ms", elapsed as f64 / 1000.);

aoc::parts!(1);

fn part_1(input: &[&str]) -> u32 {
    let (initial_graph, valves) = parse(input);
    let mut complete_graph: HashMap<Valve, ValveInfo> = HashMap::new();
    for valve in &valves {
        if initial_graph[valve].0 != 0 || valve.0 == ['A', 'A'] {
            for other_valve in &valves {
                if valve != other_valve
                    && (initial_graph[other_valve].0 != 0 || other_valve.0 == ['A', 'A'])
                {
                    let mut traverse = bft(
                        State {
                            node: *valve,
                            cost: 0,
                        },
                        |s| s.adjacent(&initial_graph),
                        |s| s.node,
                    );
                    let goal = traverse.find(|s| s.node == *other_valve).unwrap();
                    complete_graph
                        .entry(*valve)
                        .and_modify(|x| x.other_valves.push((*other_valve, goal.cost + 1)))
                        .or_insert(ValveInfo {
                            flow_rate: (initial_graph[valve].0),
                            other_valves: (vec![(*other_valve, goal.cost + 1)]),
                        });
                }
            }
        }
    }
    let now = Instant::now();
    let mut time = 0;
    let mut max = 0;
    let mut curr_release = 0;
    let mut visited: HashSet<Valve> = HashSet::new();
    visited.insert(Valve::new("AA"));
    all_paths(
        &complete_graph,
        &(Valve::new("AA"), 0),
        &mut time,
        &mut max,
        &mut curr_release,
        &mut visited,
    );
    let elapsed = now.elapsed().as_micros();
    println!("{}ms", elapsed as f64 / 1000.);
    max
}

// fn part_2(input: &[&str]) -> bool {
//     unimplemented!()
// }

fn all_paths(
    complete_graph: &HashMap<Valve, ValveInfo>,
    (valve, dist): &(Valve, u8),
    time: &mut u32,
    max: &mut u32,
    curr_release: &mut u32,
    visited: &mut HashSet<Valve>,
) {
    if *time + *dist as u32 > 30 {
        return;
    }
    *time += *dist as u32;
    *curr_release += (30 - *time) * complete_graph[valve].flow_rate;
    for adj_valveinfo in &complete_graph[valve].other_valves {
        let adj_valve = Valve((adj_valveinfo.0).0);
        if !visited.contains(&adj_valve) {
            visited.insert(adj_valve.clone());
            all_paths(
                complete_graph,
                adj_valveinfo,
                time,
                max,
                curr_release,
                visited,
            );
            visited.remove(&adj_valve);
        }
    }
    *max = *max.max(curr_release);
    *curr_release -= (30 - *time) * complete_graph[valve].flow_rate;
    *time -= *dist as u32;
}

fn parse(input: &[&str]) -> (HashMap<Valve, (u32, Vec<Valve>)>, Vec<Valve>) {
    let mut initial_graph = HashMap::new();
    let mut valves = Vec::new();
    for line in input {
        let mut split = line.split(" has flow rate=");
        let valve = split
            .next()
            .unwrap()
            .split("Valve ")
            .nth(1)
            .unwrap()
            .to_string();
        valves.push(Valve::new(&valve));
        let split_flow_valves = split.next().unwrap();
        let flow_rate: u32 = split_flow_valves
            .split("; tunnel")
            .next()
            .unwrap()
            .parse()
            .unwrap();
        if let Some(valves) = split_flow_valves.split(" lead to valves ").nth(1) {
            let tunnels: Vec<Valve> = valves.split(", ").map(|x| Valve::new(x)).collect();
            initial_graph.insert(Valve::new(&valve), (flow_rate, tunnels));
        } else {
            let tunnels: Vec<Valve> = split_flow_valves
                .split(" leads to valve ")
                .nth(1)
                .unwrap()
                .split(", ")
                .map(|x| Valve::new(x))
                .collect();
            initial_graph.insert(Valve::new(&valve), (flow_rate, tunnels));
        }
    }
    (initial_graph, valves)
}

struct State {
    node: Valve,
    cost: u8,
}

impl State {
    fn adjacent(&self, graph: &HashMap<Valve, (u32, Vec<Valve>)>) -> Vec<State> {
        let mut states = Vec::new();
        for node in &graph[&self.node].1 {
            states.push(State {
                node: *node,
                cost: self.cost + 1,
            });
        }
        states
    }
}

#[derive(Debug)]
struct ValveInfo {
    flow_rate: u32,
    other_valves: Vec<(Valve, u8)>,
}

#[derive(Eq, Hash, PartialEq, Debug, Clone, Copy)]
struct Valve([char; 2]);

impl Valve {
    fn new(name: &str) -> Valve {
        let mut chars = name.chars();
        Valve([chars.next().unwrap(), chars.next().unwrap()])
    }
}
