use std::collections::HashMap;

use aoc::{Input, IterUnwrap, Parse};

aoc::parts!(1, 2);

fn part_1(input: Input) -> impl ToString {
    let (instructions, tree, _) = parse(input);
    let num = instructions.chars().count();
    let mut steps = 0;
    let mut curr_pos = ['A', 'A', 'A'];
    while curr_pos != ['Z', 'Z', 'Z'] {
        for step in instructions.chars() {
            match step {
                'L' => curr_pos = tree.get(&curr_pos).unwrap().0,
                'R' => curr_pos = tree.get(&curr_pos).unwrap().1,
                _ => (),
            }
        }
        steps += 1;
    }
    steps * num
}

fn part_2(input: Input) -> impl ToString {
    let (instructions, tree, starting_nodes) = parse(input);
    let num = instructions.chars().count();
    let mut list_of_steps = Vec::new();
    for starting in starting_nodes {
        let mut steps = 0;
        let mut curr_pos = starting;
        while curr_pos[2] != 'Z' {
            for step in instructions.chars() {
                match step {
                    'L' => curr_pos = tree.get(&curr_pos).unwrap().0,
                    'R' => curr_pos = tree.get(&curr_pos).unwrap().1,
                    _ => (),
                }
            }
            steps += 1;
        }
        list_of_steps.push(steps);
    }
    lcm(&list_of_steps) * num
}

fn parse(
    input: Input,
) -> (
    &str,
    HashMap<[char; 3], ([char; 3], [char; 3])>,
    Vec<[char; 3]>,
) {
    let instructions = input[0];
    let mut tree = HashMap::new();
    let mut starting_nodes = Vec::new();
    for line in &input[2..] {
        let mut parser = line.as_parser();
        let node = parser.before(" = (").chars().collect_n();
        if node[2] == 'A' {
            starting_nodes.push(node);
        }
        tree.insert(
            node,
            (
                parser.before(", ").chars().collect_n(),
                parser.before(")").chars().collect_n(),
            ),
        );
    }
    (instructions, tree, starting_nodes)
}

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn lcm(list: &[usize]) -> usize {
    if list.len() == 1 {
        list[0]
    } else {
        let a = list[0];
        let b = lcm(&list[1..]);
        a * b / gcd(a, b)
    }
}
