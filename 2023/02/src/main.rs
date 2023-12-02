use std::collections::HashMap;

use aoc::{Input, Parse};

aoc::parts!(1, 2);

fn part_1(input: Input) -> impl ToString {
    // more idiomatic:
    // input.lines().map(parse).map(|(id, max)| (max["red"] <= 12 && max["green"] <= 13 && max["blue"] <= 14) as u32 * id).sum()
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
    // more idiomatic:
    // input.lines().map(parse).map(|_, max| max["red"] * max["green"] * max["blue"]).sum()
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
    let id = parser.between("Game ", ": ").parse_uw();
    for handful in parser.rest().split("; ") {
        for cubes in handful.split(", ") {
            let mut parser = cubes.as_parser();
            let (num, colour) = (parser.before(" ").parse_uw(), parser.rest());
            max.insert(colour, max[colour].max(num));
        }
    }
    (id, max)
}
