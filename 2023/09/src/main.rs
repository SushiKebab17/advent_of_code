use aoc::{Input, Parse};

aoc::parts!(1, 2);

fn part_1(input: Input) -> impl ToString {
    solve(input, false)
}

fn part_2(input: Input) -> impl ToString {
    solve(input, true)
}

fn solve(input: Input, is_part_two: bool) -> i32 {
    let mut extrapolated_values_sum = 0;
    for line in input {
        let mut seq: Vec<i32> = line.ints_iter().collect();
        if is_part_two {
            seq.reverse();
        }
        let mut max_index = seq.len() - 1;
        let mut zeroes = false;
        while !zeroes {
            zeroes = true;
            for i in 0..max_index {
                seq[i] = seq[i + 1] - seq[i];
                if seq[i] != 0 {
                    zeroes = false;
                }
            }
            max_index -= 1;
        }
        extrapolated_values_sum += seq.iter().sum::<i32>();
    }
    extrapolated_values_sum
}
