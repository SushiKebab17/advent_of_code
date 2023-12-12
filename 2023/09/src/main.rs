use aoc::{Input, Parse};

aoc::parts!(1, 2);

fn part_1(input: Input) -> impl ToString {
    let mut extrapolated_values_sum = 0;
    for line in input {
        let mut sequence: Vec<i32> = line.split_whitespace().map(|x| x.parse_uw()).collect();
        extrapolated_values_sum += get_forwards(&mut sequence);
    }
    extrapolated_values_sum
}

fn part_2(input: Input) -> impl ToString {
    let mut extrapolated_values_sum = 0;
    for line in input {
        let mut sequence: Vec<i32> = line.split_whitespace().map(|x| x.parse_uw()).collect();
        sequence.reverse();
        extrapolated_values_sum += get_forwards(&mut sequence);
    }
    extrapolated_values_sum
}

fn get_forwards(seq: &mut [i32]) -> i32 {
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
    seq.iter().sum()
}
