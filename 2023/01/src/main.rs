use std::collections::HashMap;

use aoc::Input;

aoc::parts!(1, 2);

fn part_1(input: Input) -> impl ToString {
    let mut total = 0;
    for line in input.lines() {
        let mut forward = line.chars();
        let mut first = 0;
        let mut backward = line.chars().rev();
        let mut last = 0;
        while let Some(c) = forward.next() {
            if let Some(digit) = c.to_digit(10) {
                first = digit;
                break;
            }
        }
        while let Some(c) = backward.next() {
            if let Some(digit) = c.to_digit(10) {
                last = digit;
                break;
            }
        }
        total += first * 10 + last;
    }

    total
}

fn part_2(input: Input) -> impl ToString {
    let mut total = 0;
    let digits_forwards = HashMap::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]);
    for line in input.lines() {
        let (mut first, mut last) = (0, 0);
        for i in 0..line.len() {
            if let Some(c) = line.chars().nth(i).unwrap().to_digit(10) {
                first = c;
                break;
            }
            if i + 2 < line.len() {
                if let Some(d) = digits_forwards.get(&line[i..i + 3]) {
                    first = *d;
                    break;
                }
            }
            if i + 3 < line.len() {
                if let Some(d) = digits_forwards.get(&line[i..i + 4]) {
                    first = *d;
                    break;
                }
            }
            if i + 4 < line.len() {
                if let Some(d) = digits_forwards.get(&line[i..i + 5]) {
                    first = *d;
                    break;
                }
            }
        }

        for i in (0..line.len()).rev() {
            if let Some(c) = line.chars().nth(i).unwrap().to_digit(10) {
                last = c;
                break;
            }
            if i > 2 {
                if let Some(d) = digits_forwards.get(&line[i - 2..=i]) {
                    last = *d;
                    break;
                }
            }
            if i > 3 {
                if let Some(d) = digits_forwards.get(&line[i - 3..=i]) {
                    last = *d;
                    break;
                }
            }
            if i > 4 {
                if let Some(d) = digits_forwards.get(&line[i - 4..=i]) {
                    last = *d;
                    break;
                }
            }
        }
        total += first * 10 + last;
    }
    total
}
