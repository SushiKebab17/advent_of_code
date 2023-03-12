aoc::parts!(1, 2);

use std::collections::HashSet;

fn part_1(input: &[&str]) -> i32 {
    let mut total = 0;
    let mut curr_cycle = 0;
    let mut x = 1;
    let cycle_nums = HashSet::from([20, 60, 100, 140, 180, 220]);
    for line in input {
        let mut op_length = 1;
        let mut add = 0;
        if &line[..1] == "a" {
            op_length += 1;
            add = line[5..].parse().unwrap();
        }
        for _ in 0..op_length {
            curr_cycle += 1;
            if cycle_nums.contains(&curr_cycle) {
                total += x * curr_cycle;
            }
        }
        x += add;
    }
    total
}

fn part_2(input: &[&str]) -> String {
    let mut curr_cycle = 0;
    let mut x: isize = 1;
    let mut crt = ["."; 240];
    for line in input {
        let mut op_length = 1;
        let mut add: isize = 0;
        if &line[..1] == "a" {
            op_length += 1;
            add = line[5..].parse().unwrap();
        }
        for _ in 0..op_length {
            curr_cycle += 1;
            let arr = [(x - 1) as usize, x as usize, (x + 1) as usize];
            if arr.contains(&((curr_cycle - 1) % 40)) {
                crt[curr_cycle - 1] = "#";
            }
        }
        x += add;
    }
    format!(
        "{:?}\n{:?}\n{:?}\n{:?}\n{:?}\n{:?}",
        &crt[..40],
        &crt[40..80],
        &crt[80..120],
        &crt[120..160],
        &crt[160..200],
        &crt[200..240]
    )
}
