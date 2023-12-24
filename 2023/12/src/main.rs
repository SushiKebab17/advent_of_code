use std::{collections::HashMap, usize};

use aoc::{Input, Parse};

aoc::parts!(1, 2);

fn part_1(input: Input) -> impl ToString {
    let mut total = 0;
    for line in input {
        let mut parser = line.as_parser();
        let springs = parser.before(" ");
        let contig_groups: Vec<u64> = parser.rest().split(",").map(|s| s.parse_uw()).collect();
        total += calculate_arrs(springs, &contig_groups);
    }
    total
}

fn part_2(input: Input) -> impl ToString {
    let mut total = 0;
    for line in input {
        let mut parser = line.as_parser();
        let springs = parser.before(" ");
        let contig_groups: Vec<u64> = parser.rest().split(",").map(|s| s.parse_uw()).collect();
        let mut dup_springs = springs.to_string();
        let mut dup_contig_groups = contig_groups.clone();
        for _ in 0..4 {
            dup_springs += &("?".to_owned() + springs);
            dup_contig_groups.append(&mut contig_groups.clone());
        }
        total += calculate_arrs(&dup_springs, &dup_contig_groups);
    }
    total
}

fn calculate_arrs(springs: &str, contig_groups: &[u64]) -> u64 {
    let mut pos_count = HashMap::from([(0, 1)]);
    for (cg_i, &group) in contig_groups.iter().enumerate() {
        let mut new_pos_count = HashMap::new();
        for (&pos, &count) in &pos_count {
            for i in pos..(springs.len() - contig_groups[cg_i + 1..].iter().sum::<u64>() as usize
                + contig_groups[cg_i + 1..].len())
            {
                if (i + (group as usize) - 1 < springs.len()
                    && !springs[i..i + group as usize].contains("."))
                    && ((cg_i < contig_groups.len() - 1
                        && i + (group as usize) < springs.len()
                        && springs.chars().nth(i + group as usize).unwrap() != '#')
                        || (cg_i == contig_groups.len() - 1
                            && !springs[i + group as usize..].contains("#")))
                {
                    new_pos_count.insert(
                        i + group as usize + 1,
                        if let Some(val) = new_pos_count.get(&(i + group as usize + 1)) {
                            val + count
                        } else {
                            count
                        },
                    );
                }
                if springs.chars().nth(i).unwrap() == '#' {
                    break;
                }
            }
        }
        pos_count = new_pos_count.clone();
    }
    pos_count.values().sum()
}
