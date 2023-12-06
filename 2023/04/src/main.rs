use std::collections::HashSet;

use aoc::{Input, Parse};

aoc::parts!(1, 2);

fn part_1(input: Input) -> impl ToString {
    let mut total = 0;
    for line in input {
        let (winning, test) = parse(line);
        let numbers_won = winning.intersection(&test).collect::<HashSet<_>>().len();
        if numbers_won != 0 {
            total += 2u32.pow((numbers_won - 1) as u32);
        }
    }
    total
}

fn part_2(input: Input) -> impl ToString {
    let mut total = 0;
    let mut total_scratchcards = vec![1; input.len()];
    for (i, line) in input.lines().enumerate() {
        let (winning, test) = parse(line);
        let numbers_won = winning.intersection(&test).collect::<HashSet<_>>().len();
        for j in (i + 1)..(i + 1 + numbers_won) {
            total_scratchcards[j] += total_scratchcards[i];
        }
        total += total_scratchcards[i];
    }
    total
}

fn parse(line: &str) -> (HashSet<&str>, HashSet<&str>) {
    let rest = line.as_parser().after(": ");
    let mut rest_parser = rest.as_parser();
    let mut winning: HashSet<_> = rest_parser.before(" | ").split(" ").collect();
    let mut test: HashSet<_> = rest_parser.rest().split(" ").collect();
    winning.remove("");
    test.remove("");
    (winning, test)
}
