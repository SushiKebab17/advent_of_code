use std::collections::HashMap;

use aoc::{Input, IterUnwrap, Parse};

aoc::parts!(1, 2);

fn part_1(input: Input) -> impl ToString {
    let mut total = 0;
    for line in input {
        let (id, max) = parse(line);
        total += if max["red"] <= 12 && max["green"] <= 13 && max["blue"] <= 14 {
            id
        } else {
            0
        };
    }
    total
}

fn part_2(input: Input) -> impl ToString {
    let mut total = 0;
    for line in input {
        let (_, max) = parse(line);
        total += max["red"] * max["green"] * max["blue"];
    }
    total
}

fn parse(line: &str) -> (u32, HashMap<&str, u32>) {
    let mut max = HashMap::from([("red", 0), ("green", 0), ("blue", 0)]);
    let mut parser = line.as_parser();
    let id: u32 = parser.between("Game ", ": ").parse_uw();
    for handful in parser.rest().split("; ") {
        let mut split_cubes = handful.split(", ");
        while let Some(cubes) = split_cubes.next() {
            let mut split_colour = cubes.split(" ");
            let num: u32 = split_colour.next_uw().parse_uw();
            let colour = split_colour.next_uw();
            max.insert(colour, max[colour].max(num));
        }
    }
    (id, max)
}
