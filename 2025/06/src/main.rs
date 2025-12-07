use std::vec;

use aoc::{Input, Parse};

aoc::parts!(1, 2);

fn part_1(input: Input) -> impl ToString {
    let mut operations = Vec::new();
    let mut results = Vec::new();
    for op in input[input.len() - 1].split_whitespace() {
        match op {
            "+" => {
                operations.push(Operator::Add);
                results.push(0);
            }
            _ => {
                operations.push(Operator::Multiply);
                results.push(1);
            }
        }
    }
    for line in &input[..input.len() - 1] {
        let nums = line
            .split_whitespace()
            .map(|n| n.parse_uw::<u64>())
            .enumerate();
        for (i, num) in nums {
            match operations[i] {
                Operator::Add => results[i] += num,
                Operator::Multiply => results[i] *= num,
            }
        }
    }
    results.into_iter().sum::<u64>()
}

fn part_2(input: Input) -> impl ToString {
    let line_len = input[0].len();
    let mut numbers = vec![None; line_len];
    for line in &input[..input.len() - 1] {
        for (i, c) in line.chars().enumerate() {
            if let Some(digit) = c.to_digit(10) {
                if let Some(entry) = numbers.get_mut(i) {
                    match entry {
                        Some(n) => *entry = Some(*n * 10 + digit as u64),
                        None => *entry = Some(digit as u64),
                    }
                }
            }
        }
    }
    let mut total = 0;
    let mut iter = numbers.into_iter();
    for op in input[input.len() - 1].split_whitespace() {
        match op {
            "+" => {
                let mut sum = 0;
                while let Some(Some(n)) = iter.next() {
                    sum += n;
                }
                total += sum;
            }
            _ => {
                let mut prod = 1;
                while let Some(Some(n)) = iter.next() {
                    prod *= n;
                }
                total += prod;
            }
        }
    }
    total
}

enum Operator {
    Add,
    Multiply,
}
