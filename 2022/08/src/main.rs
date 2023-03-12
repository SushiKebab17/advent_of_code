aoc::parts!(1, 2);

use grid::{v, Grid, Vector};

fn part_1(input: &[&str]) -> u32 {
    let forest = parse(input);
    let mut total = 0;
    let mut pos = forest.positions();
    while let Some(Vector { x, y }) = pos.next() {
        if coord_is_on_edge(&forest, (x, y)) || is_visible(&forest, (x, y)) {
            total += 1;
        }
    }
    total
}

fn part_2(input: &[&str]) -> i64 {
    let forest = parse(input);
    let mut max_score = 0;
    let mut pos = forest.positions();
    while let Some(Vector { x, y }) = pos.next() {
        if coord_is_on_edge(&forest, (x, y)) {
            continue;
        }
        let mut curr_score = 1;
        let tree_height = forest[Vector { x, y }];
        for n in 1..=x {
            if forest[v!(x - n, y)] >= tree_height || n == x {
                curr_score *= n;
                break;
            }
        }
        for n in 1..=y {
            if forest[v!(x, y - n)] >= tree_height || n == y {
                curr_score *= n;
                break;
            }
        }
        for n in 1..(forest.width() - x) {
            if forest[v!(x + n, y)] >= tree_height || n == forest.width() - x - 1 {
                curr_score *= n;
                break;
            }
        }
        for n in 1..(forest.height() - y) {
            if forest[v!(x, y + n)] >= tree_height || n == forest.height() - y - 1 {
                curr_score *= n;
                break;
            }
        }
        if curr_score > max_score {
            max_score = curr_score;
        }
    }
    max_score
}

fn parse(input: &[&str]) -> Grid<u8> {
    let parse = input
        .iter()
        .flat_map(|line| line.as_bytes())
        .map(|n| n - 48);
    Grid::from_iter(input[0].len() as i64, input.len() as i64, parse)
}

fn coord_is_on_edge(forest: &Grid<u8>, (x, y): (i64, i64)) -> bool {
    x == 0 || y == 0 || x == forest.width() - 1 || y == forest.height() - 1
}

fn is_visible(forest: &Grid<u8>, (x, y): (i64, i64)) -> bool {
    let (mut up, mut down, mut left, mut right) = (true, true, true, true);
    let tree_height = forest[v!(x, y)];
    for n in 0..x {
        if forest[v!(n, y)] >= tree_height {
            left = false;
            break;
        }
    }
    for n in 0..y {
        if forest[v!(x, n)] >= tree_height {
            up = false;
            break;
        }
    }
    for n in x + 1..forest.width() {
        if forest[v!(n, y)] >= tree_height {
            right = false;
            break;
        }
    }
    for n in y + 1..forest.height() {
        if forest[v!(x, n)] >= tree_height {
            down = false;
            break;
        }
    }
    up || down || left || right
}
