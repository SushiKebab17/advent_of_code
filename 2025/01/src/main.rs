use aoc::{Input, Parse};

aoc::parts!(1, 2);

fn part_1(input: Input) -> impl ToString {
    let (mut count, mut dial_pos) = (0, 50);
    for line in input {
        let mut rotation = line.ints::<1, i32>()[0];
        if line.starts_with("L") {
            rotation *= -1;
        }
        dial_pos = (dial_pos + rotation) % 100;
        if dial_pos == 0 {
            count += 1;
        }
    }
    count
}

fn part_2(input: Input) -> impl ToString {
    let (mut count, mut dial_pos) = (0, 50);
    for line in input {
        let rotation = line.ints::<1, i32>()[0];
        let full_rotations = rotation / 100;
        count += full_rotations;
        let mut remainder = rotation % 100;
        if line.starts_with("L") {
            remainder *= -1;
        }
        let old_dial_pos = dial_pos;
        dial_pos += remainder;
        if (old_dial_pos != 0 && dial_pos <= 0) || dial_pos >= 100 {
            count += 1;
        }
        dial_pos = ((dial_pos % 100) + 100) % 100;
    }
    count
}
