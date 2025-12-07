use aoc::{Input, IterUnwrap, Parse};

aoc::parts!(1, 2);

fn part_1(input: Input) -> impl ToString {
    input
        .as_lines()
        .first()
        .unwrap()
        .split(",")
        .map(|range| {
            let mut split = range.split("-");
            compute_invalids_pt1(split.next_uw().parse_uw(), split.next_uw().parse_uw())
        })
        .sum::<u64>()
}

fn part_2(input: Input) -> impl ToString {
    input
        .as_lines()
        .first()
        .unwrap()
        .split(",")
        .map(|range| {
            let mut split = range.split("-");
            compute_invalids_pt2(split.next_uw().parse_uw(), split.next_uw().parse_uw())
        })
        .sum::<u64>()
}

fn compute_invalids_pt1(low: u64, high: u64) -> u64 {
    let mut sum = 0;
    for n in low..=high {
        let str_num = n.to_string();
        if str_num[0..str_num.len() / 2] == str_num[str_num.len() / 2..str_num.len()] {
            sum += n;
        }
    }
    sum
}

fn compute_invalids_pt2(low: u64, high: u64) -> u64 {
    let mut sum = 0;
    for n in low..=high {
        let str_num = n.to_string();
        let divisors = (1..str_num.len() as u64)
            .filter(|&d| str_num.len() as u64 % d == 0)
            .collect::<Vec<u64>>();
        for divisor in divisors {
            let mut all_equal = true;
            let first_part = &str_num[0..divisor as usize];
            for i in 1..str_num.len() as u64 / divisor {
                let start = (i * divisor) as usize;
                let end = ((i + 1) * divisor) as usize;
                if &str_num[start..end] != first_part {
                    all_equal = false;
                    break;
                }
            }
            if all_equal {
                sum += n;
                break;
            }
        }
    }
    sum
}
