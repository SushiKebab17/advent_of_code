aoc::parts!(1, 2);

use std::collections::HashSet;

fn part_1(input: &[&str]) -> usize {
    simulate_rope(input, 2)
}

fn part_2(input: &[&str]) -> usize {
    simulate_rope(input, 10)
}

fn simulate_rope(input: &[&str], knots: usize) -> usize {
    let mut rope = vec![(0, 0); knots];
    let mut visited = HashSet::new();
    for line in input {
        let mut split = line.split(" ");
        let dir = split.next().unwrap();
        let num: i32 = split.next().unwrap().parse().unwrap();
        for _ in 0..num {
            match dir {
                "L" => rope[0].0 -= 1,
                "R" => rope[0].0 += 1,
                "U" => rope[0].1 += 1,
                "D" => rope[0].1 -= 1,
                _ => (),
            }
            for i in 1..knots {
                let dx: i32 = rope[i - 1].0 - rope[i].0;
                let dy: i32 = rope[i - 1].1 - rope[i].1;
                if dx.abs() == 2 || dy.abs() == 2 {
                    rope[i].0 += if dx == 0 { 0 } else { dx / dx.abs() };
                    rope[i].1 += if dy == 0 { 0 } else { dy / dy.abs() };
                }
            }
            visited.insert(rope[knots - 1]);
        }
    }
    visited.len()
}
