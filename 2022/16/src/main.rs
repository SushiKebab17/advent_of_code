use search::bft;
use std::collections::{HashMap, HashSet};

aoc::parts!(1);

fn part_1(input: &[&str]) -> bool {
    let (initial_graph, valves) = parse(input);
    //println!("{:?}", initial_graph);
    let mut complete_graph: HashMap<String, ValveInfo> = HashMap::new();
    for valve in initial_graph.keys() {
        if initial_graph[valve].0 != 0 || valve == "AA" {
            for other_valve in &valves {
                if valve != other_valve
                    && (initial_graph[other_valve].0 != 0 || other_valve == "AA")
                {
                    let mut traverse = bft(
                        State {
                            node: valve.clone(),
                            cost: 0,
                        },
                        |s| s.adjacent(&initial_graph),
                        |s| s.node.clone(),
                    );
                    let goal = traverse.find(|s| s.node == *other_valve).unwrap();
                    complete_graph
                        .entry(valve.clone())
                        .and_modify(|x| x.other_valves.push((other_valve.clone(), goal.cost + 1)))
                        .or_insert(ValveInfo {
                            flow_rate: (initial_graph[valve].0),
                            other_valves: (vec![(other_valve.clone(), goal.cost + 1)]),
                        });
                }
            }
        }
    }
    // println!();
    // for k in complete_graph.keys() {
    //     println!("{k}, {:?}", complete_graph[k]);
    // }
    let mut time = 0;
    let mut max = 0;
    let mut curr_release = 0;
    let mut visited: HashSet<String> = HashSet::new();
    visited.insert("AA".to_string());
    all_paths(
        &complete_graph,
        &("AA".to_string(), 0),
        &mut time,
        &mut max,
        &mut curr_release,
        &mut visited,
    );
    println!("{}", max);
    unimplemented!()
}

// fn part_2(input: &[&str]) -> bool {
//     unimplemented!()
// }

fn all_paths(
    complete_graph: &HashMap<String, ValveInfo>,
    (valve, dist): &(String, u8),
    time: &mut u32,
    max: &mut u32,
    curr_release: &mut u32,
    visited: &mut HashSet<String>,
) {
    if *time + *dist as u32 > 30 {
        return;
    }
    *time += *dist as u32;
    *curr_release += (30 - *time) * complete_graph[valve].flow_rate;
    println!(
        "{}, {}, {} \n {:?}",
        valve, dist, curr_release, &complete_graph[valve].other_valves
    );
    for adj_valveinfo in &complete_graph[valve].other_valves {
        let adj_valve = (*adj_valveinfo.0).to_string();
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
}

fn parse(input: &[&str]) -> (HashMap<String, (u32, Vec<String>)>, Vec<String>) {
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
        valves.push(valve.clone());
        let split_flow_valves = split.next().unwrap();
        let flow_rate: u32 = split_flow_valves
            .split("; tunnel")
            .next()
            .unwrap()
            .parse()
            .unwrap();
        if let Some(valves) = split_flow_valves.split(" lead to valves ").nth(1) {
            let tunnels: Vec<String> = valves.split(", ").map(|x| x.to_string()).collect();
            initial_graph.insert(valve, (flow_rate, tunnels));
        } else {
            let tunnels: Vec<String> = split_flow_valves
                .split(" leads to valve ")
                .nth(1)
                .unwrap()
                .split(", ")
                .map(|x| x.to_string())
                .collect();
            initial_graph.insert(valve, (flow_rate, tunnels));
        }
    }
    (initial_graph, valves)
}

struct State {
    node: String,
    cost: u8,
}

impl State {
    fn adjacent(&self, graph: &HashMap<String, (u32, Vec<String>)>) -> Vec<State> {
        let mut states = Vec::new();
        for node in &graph[&self.node].1 {
            states.push(State {
                node: node.to_string(),
                cost: self.cost + 1,
            });
        }
        states
    }
}

#[derive(Debug)]
struct ValveInfo {
    flow_rate: u32,
    other_valves: Vec<(String, u8)>,
}
