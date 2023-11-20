use search::bft;
use std::collections::{HashMap, HashSet};
// use std::time::Instant;
// let now = Instant::now();
// let elapsed = now.elapsed().as_micros();
// println!("{}ms", elapsed as f64 / 1000.);

aoc::parts!(1, 2);

fn part_1(input: &[&str]) -> u32 {
    let complete_graph = parse(input).0;
    let mut max = 0;
    let mut visited: HashSet<Valve> = HashSet::new();
    visited.insert(Valve::new("AA"));
    all_paths_pt1(
        &complete_graph,
        Valve::new("AA"),
        30,
        &mut max,
        0,
        &mut visited,
    );
    max
}

fn part_2(input: &[&str]) -> u32 {
    let (complete_graph, valves) = parse(input);
    // for val in complete_graph.keys() {
    //     println!("{:?}: {:?}\n", val, complete_graph[val]);
    // }
    // std::io::stdin().read_line(&mut String::new()).unwrap();
    let powerset = generate_subsets(&valves);
    let mut all_valves = HashSet::new();
    for valve in valves {
        all_valves.insert(valve);
    }
    all_valves.remove(&Valve::new("AA"));
    let mut max = 0;
    for set in &powerset {
        let mut max_a = 0;
        let mut max_b = 0;
        let mut complement: HashSet<Valve> = &all_valves - set;
        all_paths_pt2(
            &complete_graph,
            Valve::new("AA"),
            26,
            &mut max_a,
            0,
            &mut set.clone(),
        );
        all_paths_pt2(
            &complete_graph,
            Valve::new("AA"),
            26,
            &mut max_b,
            0,
            &mut complement,
        );
        max = max.max(max_a + max_b);
    }
    max
}

fn generate_subsets(valves: &[Valve]) -> Vec<HashSet<Valve>> {
    if valves.is_empty() {
        return vec![HashSet::new()];
    }
    let rest = generate_subsets(&valves[1..]);
    let mut all = Vec::new();
    for subset in &rest {
        let mut new = subset.clone();
        new.remove(&Valve::new("AA"));
        all.push(new.clone());
        new.insert(valves[0]);
        all.push(new);
    }
    all
}

fn all_paths_pt1(
    complete_graph: &HashMap<Valve, ValveInfo>,
    valve: Valve,
    mut time: u32,
    max: &mut u32,
    mut curr_release: u32,
    visited: &mut HashSet<Valve>,
) {
    curr_release += time * complete_graph[&valve].flow_rate;
    for (adj_valve, &dist_to_adj) in &complete_graph[&valve].other_valves {
        if !visited.contains(adj_valve) && time > dist_to_adj as u32 {
            visited.insert(*adj_valve);
            time -= dist_to_adj as u32;
            all_paths_pt1(complete_graph, *adj_valve, time, max, curr_release, visited);
            time += dist_to_adj as u32;
            visited.remove(adj_valve);
        }
    }
    *max = (*max).max(curr_release);
}

fn all_paths_pt2(
    complete_graph: &HashMap<Valve, ValveInfo>,
    valve: Valve,
    mut time: u32,
    max: &mut u32,
    mut curr_release: u32,
    partition: &mut HashSet<Valve>,
) {
    curr_release += time * complete_graph[&valve].flow_rate;
    for adj_valve in partition.clone().iter() {
        let dist_to_adj = complete_graph[&valve].other_valves[adj_valve];
        if partition.contains(adj_valve) && time > dist_to_adj as u32 {
            partition.remove(&adj_valve);
            time -= dist_to_adj as u32;
            all_paths_pt2(
                complete_graph,
                *adj_valve,
                time,
                max,
                curr_release,
                partition,
            );
            time += dist_to_adj as u32;
            partition.insert(*adj_valve);
        }
    }

    *max = (*max).max(curr_release);
}

fn parse(input: &[&str]) -> (HashMap<Valve, ValveInfo>, Vec<Valve>) {
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
                        .and_modify(|x| {
                            x.other_valves.insert(*other_valve, goal.cost + 1);
                        })
                        .or_insert(ValveInfo {
                            flow_rate: (initial_graph[valve].0),
                            other_valves: (HashMap::from([(*other_valve, goal.cost + 1)])),
                        });
                }
            }
        }
    }
    let new_valves: Vec<Valve> = complete_graph[&Valve::new("AA")]
        .other_valves
        .keys()
        .cloned()
        .collect();
    (complete_graph, new_valves)
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
    other_valves: HashMap<Valve, u8>,
}

#[derive(Eq, Hash, PartialEq, Debug, Clone, Copy)]
struct Valve([char; 2]);

impl Valve {
    fn new(name: &str) -> Valve {
        let mut chars = name.chars();
        Valve([chars.next().unwrap(), chars.next().unwrap()])
    }
}
