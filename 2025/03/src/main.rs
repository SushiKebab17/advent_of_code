use aoc::{Input, Parse};

aoc::parts!(1, 2);

fn part_1(input: Input) -> impl ToString {
    solve(2, input)
}

fn part_2(input: Input) -> impl ToString {
    solve(12, input)
}

fn solve(number_of_digits: usize, input: Input) -> u64 {
    let mut total: u64 = 0;
    for line in input.lines() {
        let mut num: u64 = 0;
        let mut prev_digit_i: isize = -1;
        for digit_i in 1..=number_of_digits {
            let line_digits = line.char_indices().skip((prev_digit_i + 1) as usize);
            let mut digit = 0;
            for (i, c) in line_digits {
                if i >= line.len() - (number_of_digits - digit_i) {
                    break;
                }
                let n = c.to_digit(10).unwrap();
                if n > digit {
                    digit = n;
                    prev_digit_i = i as isize;
                }
            }
            num = 10 * num + digit as u64;
        }
        total += num;
    }
    total
}
