use aoc::{Input, IterUnwrap, Parse};

aoc::parts!(1, 2);

fn part_1(input: Input) -> impl ToString {
    let (times, records) = parse_pt1(input);
    let mut broken_product = 1;
    for i in 0..times.len() {
        let t = ((times[i] as f64 - ((times[i].pow(2) - 4 * records[i]) as f64).sqrt()) / 2.).ceil()
            as u32;
        broken_product *= 2 * (times[i] / 2 - t + 1) - (times[i] % 2 == 0) as u32;
    }
    broken_product
}

fn part_2(input: Input) -> impl ToString {
    let (time, record) = parse_pt2(input);
    let t = ((time as f64 - ((time.pow(2) - 4 * record) as f64).sqrt()) / 2.).ceil() as u64;
    2 * (time / 2 - t + 1) - (time % 2 == 0) as u64
}

fn parse_pt1(input: Input) -> (Vec<u32>, Vec<u32>) {
    (
        input[0].uints_iter().collect(),
        input[1].uints_iter().collect(),
    )
}

fn parse_pt2(input: Input) -> (u64, u64) {
    (
        input[0]
            .split_whitespace()
            .collect::<String>()
            .uints_iter::<u64>()
            .next_uw(),
        input[1]
            .split_whitespace()
            .collect::<String>()
            .uints_iter::<u64>()
            .next_uw(),
    )
}
