use std::collections::HashMap;

use aoc::{Input, Parse};

aoc::parts!(1, 2);

fn part_1(input: Input) -> impl ToString {
    let mut total: u32 = 0;
    for line in input {
        total += solve_pt1(line);
    }
    total
}

fn part_2(input: Input) -> impl ToString {
    let mut total: u32 = 0;
    for line in input {
        total += solve_pt2(line);
    }
    total
}

fn solve_pt1(line: &str) -> u32 {
    let (id, max) = parse(line);
    if max["red"] <= 12 && max["green"] <= 13 && max["blue"] <= 14 {
        id
    } else {
        0
    }
}

fn solve_pt2(line: &str) -> u32 {
    let (_, max) = parse(line);
    max["red"] * max["green"] * max["blue"]
}

fn parse(line: &str) -> (u32, HashMap<&str, u32>) {
    let mut max = HashMap::from([("red", 0), ("green", 0), ("blue", 0)]);
    let mut parser = line.as_parser();
    let id: u32 = parser.between("Game ", ": ").parse().unwrap();
    let mut split = parser.rest().split("; ");
    while let Some(handful) = split.next() {
        let mut split_cubes = handful.split(", ");
        while let Some(cubes) = split_cubes.next() {
            let mut split_color = cubes.split(" ");
            let num: u32 = split_color.next().unwrap().parse().unwrap();
            let color = split_color.next().unwrap();
            *max.get_mut(color).unwrap() = max[color].max(num);
        }
    }
    (id, max)
}
