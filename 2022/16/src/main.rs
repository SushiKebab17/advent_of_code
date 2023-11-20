use search::bft;
use std::collections::HashMap;
// use std::time::Instant;
// let now = Instant::now();
// let elapsed = now.elapsed().as_micros();
// println!("{}ms", elapsed as f64 / 1000.);

aoc::parts!(1, 2);

fn part_1(input: &[&str]) -> u32 {
    let (complete_graph, valves) = parse(input);
    let mut max = 0;
    let mut all_valves = vec![true; valves.len()];
    all_paths(&complete_graph, 0, 30, &mut max, 0, &mut all_valves);
    max
}

fn part_2(input: &[&str]) -> u32 {
    let (complete_graph, valves) = parse(input);
    // for val in complete_graph.keys() {
    //     println!("{:?}: {:?}\n", val, complete_graph[val]);
    // }
    // std::io::stdin().read_line(&mut String::new()).unwrap();
    let mut set = vec![false; valves.len()];
    let mut max = 0;

    while !set[set.len() - 1] {
        let mut max_a = 0;
        let mut max_b = 0;
        all_paths(&complete_graph, 0, 26, &mut max_a, 0, &mut set);
        all_paths(&complete_graph, 0, 26, &mut max_b, 0, &mut complement(&set));
        max = max.max(max_a + max_b);
        increment_set(&mut set);
    }
    max
}

fn complement(set: &[bool]) -> Vec<bool> {
    let mut complement = Vec::with_capacity(set.len());
    for valve in set {
        complement.push(!valve);
    }
    complement
}

fn increment_set(set: &mut [bool]) {
    for i in 0..set.len() {
        set[i] = !set[i];
        if set[i] {
            return;
        }
    }
}

fn all_paths(
    complete_graph: &HashMap<u32, ValveInfo>,
    valve: u32,
    mut time: u32,
    max: &mut u32,
    mut curr_release: u32,
    partition: &mut Vec<bool>,
) {
    curr_release += time * complete_graph[&valve].flow_rate;
    for adj_valve_i in 1..partition.len() {
        if valve != adj_valve_i as u32 {
            let dist_to_adj = complete_graph[&valve].other_valves[&(adj_valve_i as u32)];
            if partition[adj_valve_i - 1] && time > dist_to_adj as u32 {
                partition[adj_valve_i - 1] = false;
                time -= dist_to_adj as u32;
                all_paths(
                    complete_graph,
                    adj_valve_i as u32,
                    time,
                    max,
                    curr_release,
                    partition,
                );
                time += dist_to_adj as u32;
                partition[adj_valve_i - 1] = true;
            }
        }
    }
    // for adj_valve in partition.clone().iter() {
    //     let dist_to_adj = complete_graph[&valve].other_valves[adj_valve];
    //     if partition.contains(adj_valve) && time > dist_to_adj as u32 {
    //         partition.remove(&adj_valve);
    //         time -= dist_to_adj as u32;
    //         all_paths(
    //             complete_graph,
    //             *adj_valve,
    //             time,
    //             max,
    //             curr_release,
    //             partition,
    //         );
    //         time += dist_to_adj as u32;
    //         partition.insert(*adj_valve);
    //     }
    // }

    *max = (*max).max(curr_release);
}

fn parse(input: &[&str]) -> (HashMap<u32, ValveInfo>, Vec<Valve>) {
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
    let mut complete_graph: HashMap<u32, ValveInfo> = HashMap::new();
    let mut useful_valves: Vec<Valve> = vec![Valve::new("AA")];
    let mut valve_index_map: HashMap<Valve, u32> = HashMap::new();
    valve_index_map.insert(Valve::new("AA"), 0);
    for valve in &valves {
        if initial_graph[valve].0 != 0 || valve.0 == ['A', 'A'] {
            for other_valve in &valves {
                if valve != other_valve
                    && (initial_graph[other_valve].0 != 0 || other_valve.0 == ['A', 'A'])
                {
                    let (valve_i, other_valve_i): (u32, u32);
                    if !valve_index_map.contains_key(valve) {
                        valve_i = useful_valves.len() as u32;
                        useful_valves.push(*valve);
                        valve_index_map.insert(*valve, valve_i);
                    } else {
                        valve_i = valve_index_map[valve];
                    }

                    if !valve_index_map.contains_key(other_valve) {
                        other_valve_i = useful_valves.len() as u32;
                        useful_valves.push(*other_valve);
                        valve_index_map.insert(*other_valve, other_valve_i);
                    } else {
                        other_valve_i = valve_index_map[other_valve];
                    }

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
                        .entry(valve_i)
                        .and_modify(|x| {
                            x.other_valves.insert(other_valve_i, goal.cost + 1);
                        })
                        .or_insert(ValveInfo {
                            flow_rate: (initial_graph[valve].0),
                            other_valves: (HashMap::from([(other_valve_i, goal.cost + 1)])),
                        });
                }
            }
        }
    }
    (complete_graph, useful_valves)
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
    other_valves: HashMap<u32, u8>,
}

#[derive(Eq, Hash, PartialEq, Debug, Clone, Copy)]
struct Valve([char; 2]);

impl Valve {
    fn new(name: &str) -> Valve {
        let mut chars = name.chars();
        Valve([chars.next().unwrap(), chars.next().unwrap()])
    }
}
