use aoc::{Input, IterUnwrap, Lines, Parse};

aoc::parts!(1, 2);

// For improvement:
// sort ranges, sort inputs, use indices to track position in both and compare
fn part_1(input: Input) -> impl ToString {
    let (mut ranges, lines) = parse(input);
    ranges.sort_by(|r1, r2| r1.0.cmp(&r2.0));
    let mut merged_ranges: Vec<(u64, u64)> = Vec::new();
    for (l, h) in ranges {
        if let Some((_, mh)) = merged_ranges.last_mut() {
            if *mh + 1 >= l {
                *mh = (*mh).max(h);
                continue;
            }
        }
        merged_ranges.push((l, h));
    }
    lines
        .filter(|line| {
            merged_ranges
                .iter()
                .any(|(l, h)| (*l..=*h).contains(&line.parse_uw()))
        })
        .count()
}

fn part_2(input: Input) -> impl ToString {
    let mut ranges = parse(input).0;
    ranges.sort_by(|r1, r2| r1.0.cmp(&r2.0));
    let (mut min, mut max) = (0, 0);
    let mut total = 0;
    for (l, h) in ranges {
        if max >= l {
            max = max.max(h);
            continue;
        }
        total += max - min + 1;
        min = l;
        max = h;
    }
    total += max - min;
    total
}

fn parse(input: Input) -> (Vec<(u64, u64)>, Lines) {
    let mut ranges: Vec<(u64, u64)> = Vec::new();
    let mut lines = input.lines();
    for line in &mut lines {
        if line.is_empty() {
            break;
        }
        let mut split = line.split("-");
        ranges.push((split.next_uw().parse_uw(), split.next_uw().parse_uw()));
    }
    (ranges, lines)
}
