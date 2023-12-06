use std::collections::HashMap;

use aoc::{Input, Parse};
use grid::prelude::*;

aoc::parts!(1);

fn part_1(input: Input) -> impl ToString {
    let number_positions = parse(input);
    let lines = input.len();
    let columns = input[0].len();
    let mut total = 0;
    for (i, line) in input.lines().enumerate() {
        for (j, ch) in line.chars().enumerate() {
            if ch.to_digit(10) == None && ch != '.' {
                let pos = v(i as i64, j as i64);
                let adj = ADJACENT.map(|v| v + pos);
            }
        }
    }
    0
}

// fn part_2(input: Input) -> impl ToString {
//     0
// }

fn parse(input: Input) -> HashMap<Vector, u32> {
    let mut number_positions = HashMap::new();
    for (i, line) in input.lines().enumerate() {
        let mut num = 0;
        let mut first_index_num = 0;
        for (j, ch) in line.chars().enumerate() {
            if let Some(digit) = ch.to_digit(10) {
                num = 10 * num + digit;
            } else {
                for k in first_index_num..j {
                    // println!("{:?}, {}", v(i as i64, k as i64), num);
                    number_positions.insert(v(i as i64, k as i64), num);
                }
                first_index_num = j + 1;
                num = 0;
            }
        }
        for k in first_index_num..line.chars().count() {
            number_positions.insert(v(i as i64, k as i64), num);
        }
    }
    number_positions
}
