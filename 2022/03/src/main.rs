aoc::parts!(1, 2);

use std::collections::HashSet;

fn part_1(input: &[&str]) -> u32 {
    let mut total = 0;
    for line in input {
        let first: HashSet<u8> = HashSet::from_iter(line.bytes().take(line.len() / 2));
        for letter in line.bytes().skip(line.len() / 2) {
            if first.contains(&letter) {
                total += letter as u32 - if letter >= 97 { 96 } else { 38 };
                break;
            }
        }
    }
    total
}

fn part_2(input: &[&str]) -> u32 {
    let mut total = 0;
    for group in input.chunks(3) {
        let first: HashSet<u8> = HashSet::from_iter(group[0].bytes());
        let second: HashSet<u8> = HashSet::from_iter(group[1].bytes());
        let third: HashSet<u8> = HashSet::from_iter(group[2].bytes());
        let common = *first
            .intersection(&(&second & &third))
            .into_iter()
            .next()
            .unwrap() as u32;
        total += common - if common >= 97 { 96 } else { 38 };
    }
    total
}
