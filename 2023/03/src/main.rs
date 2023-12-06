use std::collections::HashMap;

use aoc::Input;
use grid::prelude::*;

aoc::parts!(1, 2);

fn part_1(input: Input) -> impl ToString {
    let mut number_positions = parse(input);
    let mut total = 0;
    for (i, line) in input.lines().enumerate() {
        for (j, ch) in line.chars().enumerate() {
            if ch.to_digit(10) == None && ch != '.' {
                let adj = ADJACENT.iter().map(|&vec| vec + v(i as i64, j as i64));
                for pos in adj {
                    if number_positions.contains_key(&pos) {
                        let num_pos = number_positions[&pos];
                        total += num_pos.num;
                        for k in (num_pos.low)..(num_pos.high) {
                            number_positions.remove(&v(num_pos.row, k as i64));
                        }
                    }
                }
            }
        }
    }
    total
}

fn part_2(input: Input) -> impl ToString {
    let mut number_positions = parse(input);
    let mut total = 0;
    for (i, line) in input.lines().enumerate() {
        'inner: for (j, ch) in line.chars().enumerate() {
            if ch == '*' {
                let adj = ADJACENT.iter().map(|&vec| vec + v(i as i64, j as i64));
                let mut curr_product = 1;
                let mut num_adj = 0;
                for pos in adj {
                    if number_positions.contains_key(&pos) {
                        let num_pos = number_positions[&pos];
                        println!("{:?}, {:?}", pos, num_pos);
                        curr_product *= num_pos.num;
                        num_adj += 1;
                        for k in (num_pos.low)..(num_pos.high) {
                            number_positions.remove(&v(num_pos.row, k as i64));
                        }
                        if num_adj >= 3 {
                            continue 'inner;
                        }
                    }
                }
                if num_adj != 2 {
                    continue 'inner;
                }
                println!();
                total += curr_product;
            }
        }
    }
    total
}

fn parse(input: Input) -> HashMap<Vector, NumberPos> {
    let mut number_positions = HashMap::new();
    for (i, line) in input.lines().enumerate() {
        let mut num = 0;
        let mut first_index_num = 0;
        for (j, ch) in line.chars().enumerate() {
            if let Some(digit) = ch.to_digit(10) {
                num = 10 * num + digit;
            } else {
                for k in first_index_num..j {
                    number_positions.insert(
                        v(i as i64, k as i64),
                        NumberPos::new(num, i as i64, first_index_num as i64, j as i64),
                    );
                }
                first_index_num = j + 1;
                num = 0;
            }
        }
        for k in first_index_num..line.chars().count() {
            number_positions.insert(
                v(i as i64, k as i64),
                NumberPos::new(
                    num,
                    i as i64,
                    first_index_num as i64,
                    line.chars().count() as i64,
                ),
            );
        }
    }
    number_positions
}

#[derive(Debug, Clone, Copy)]
struct NumberPos {
    num: u32,
    row: i64,
    low: i64,
    high: i64,
}

impl NumberPos {
    fn new(num: u32, row: i64, low: i64, high: i64) -> Self {
        NumberPos {
            num,
            row,
            low,
            high,
        }
    }
}
