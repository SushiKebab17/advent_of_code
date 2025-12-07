use aoc::{Input, Parse};
use nd::{Matrix, Vec2};

aoc::parts!(1, 2);

fn part_1(input: Input) -> impl ToString {
    let mat = parse(input);
    let mut total = 0;
    'main: for pos in mat.positions::<i32>() {
        if mat[pos] != Wall::Paper {
            continue;
        }
        let mut count = 0;
        for step in Vec2::adj() {
            let neighbor = pos + step;
            if let Some(wall) = mat.get(neighbor) {
                if let Wall::Paper = wall {
                    count += 1;
                    if count == 4 {
                        continue 'main;
                    }
                }
            }
        }
        total += 1;
    }
    total
}

fn part_2(input: Input) -> impl ToString {
    let mut mat = parse(input);
    let mut changes = true;
    let mut total = 0;
    while changes {
        changes = false;
        let mut next_mat = mat.clone();
        'main: for pos in mat.positions::<i32>() {
            if mat[pos] != Wall::Paper {
                continue;
            }
            let mut count = 0;
            for step in Vec2::adj() {
                let neighbor = pos + step;
                if let Some(wall) = mat.get(neighbor) {
                    if let Wall::Paper = wall {
                        count += 1;
                        if count == 4 {
                            continue 'main;
                        }
                    }
                }
            }
            total += 1;
            changes = true;
            next_mat[pos] = Wall::Empty;
        }
        mat = next_mat;
    }
    total
}

fn parse(input: Input) -> Matrix<Wall> {
    input
        .lines()
        .map(|line| {
            line.chars().map(|c| match c {
                '@' => Wall::Paper,
                _ => Wall::Empty,
            })
        })
        .collect()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Wall {
    Paper,
    Empty,
}
