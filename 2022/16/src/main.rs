use search::bft;
use std::collections::HashMap;

aoc::parts!(1);

fn part_1(input: &[&str]) -> bool {
    let (initial_graph, valves) = parse(input);
    println!("{:?}", initial_graph);
    let mut complete_graph: HashMap<String, (u32, Vec<(String, u8)>)> = HashMap::new();
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
                        .and_modify(|x| x.1.push((other_valve.clone(), goal.cost + 1)))
                        .or_insert((
                            initial_graph[valve].0,
                            vec![(other_valve.clone(), goal.cost + 1)],
                        ));
                }
            }
        }
    }
    println!();
    for k in complete_graph.keys() {
        println!("{k}, {:?}", complete_graph[k]);
    }
    unimplemented!()
}

// fn part_2(input: &[&str]) -> bool {
//     unimplemented!()
// }

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
