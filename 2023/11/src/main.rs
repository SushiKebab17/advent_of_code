use std::collections::HashSet;

use aoc::Input;

aoc::parts!(1, 2);

fn part_1(input: Input) -> impl ToString {
    let adjusted_galaxies = parse(input, 2);
    let mut total = 0;
    for i in 0..(adjusted_galaxies.len() - 1) {
        for j in (i + 1)..adjusted_galaxies.len() {
            total += (adjusted_galaxies[i].0 as i64 - adjusted_galaxies[j].0 as i64).abs()
                + (adjusted_galaxies[i].1 as i64 - adjusted_galaxies[j].1 as i64).abs();
        }
    }
    total
}

fn part_2(input: Input) -> impl ToString {
    let adjusted_galaxies = parse(input, 1_000_000);
    let mut total = 0;
    for i in 0..(adjusted_galaxies.len() - 1) {
        for j in (i + 1)..adjusted_galaxies.len() {
            total += (adjusted_galaxies[i].0 as i64 - adjusted_galaxies[j].0 as i64).abs()
                + (adjusted_galaxies[i].1 as i64 - adjusted_galaxies[j].1 as i64).abs();
        }
    }
    total
}

fn parse(input: Input, expansion: u64) -> Vec<(u64, u64)> {
    let mut galaxies = vec![Vec::with_capacity(input[0].len()); input.len()];
    let mut empty_rows: HashSet<_> = (0..input.len()).into_iter().collect();
    let mut empty_cols: HashSet<_> = (0..input[0].len()).into_iter().collect();
    for (x, line) in input.lines().enumerate() {
        for (y, ch) in line.char_indices() {
            if ch == '#' {
                empty_rows.remove(&x);
                empty_cols.remove(&y);
                galaxies[x].push(y);
            }
        }
    }
    let mut adjusted_galaxies = Vec::new();
    let mut exp_row = 0;
    for i in 0..galaxies.len() {
        if galaxies[i].len() == 0 {
            exp_row += expansion - 1;
            continue;
        }
        for j in 0..galaxies[i].len() {
            let exp_col =
                empty_cols.iter().filter(|&c| *c < galaxies[i][j]).count() as u64 * (expansion - 1);
            adjusted_galaxies.push((i as u64 + exp_row, galaxies[i][j] as u64 + exp_col));
        }
    }
    adjusted_galaxies
}
