aoc::parts!(1, 2);

use std::collections::HashSet;

fn part_1(input: &[&str]) -> usize {
    find_first_n_unique(input[0], 4)
}

fn part_2(input: &[&str]) -> usize {
    find_first_n_unique(input[0], 14)
}

fn find_first_n_unique(line: &str, n: usize) -> usize {
    for i in n..line.len() {
        if HashSet::<&u8>::from_iter(line[i - n..i].as_bytes()).len() == n {
            return i;
        }
    }
    0
}
