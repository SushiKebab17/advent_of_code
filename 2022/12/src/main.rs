aoc::parts!(1, 2);

use std::collections::{HashSet, VecDeque};

use grid::{constants::*, Grid, Vector};

fn part_1(input: &[&str]) -> i32 {
    let (grid, start, end) = parse(input);
    find_start(|v| v == start, &grid, end)
}

fn part_2(input: &[&str]) -> i32 {
    let (grid, _, end) = parse(input);
    find_start(|v| &grid[v] == &b'a', &grid, end)
}

fn find_start(check: impl Fn(Vector) -> bool, grid: &Grid<u8>, end: Vector) -> i32 {
    let mut queue = VecDeque::new();
    let mut visited = HashSet::from([end]);
    queue.push_back((end, 0));
    loop {
        let (curr, s) = queue.pop_front().unwrap();
        if check(curr) {
            return s;
        }
        for n in ORTHOGONAL.map(|v| curr + v) {
            if grid.in_bounds(n) && !visited.contains(&n) && (grid[curr] <= 1 + grid[n]) {
                visited.insert(n);
                queue.push_back((n, s + 1));
            }
        }
    }
}

fn parse(input: &[&str]) -> (Grid<u8>, Vector, Vector) {
    let mut grid = Grid::new(input[0].len() as i64, input.len() as i64, 0);
    let mut pos = grid.positions();
    let mut end = Vector::new(0, 0);
    let mut start = Vector::new(0, 0);
    for line in input {
        for c in line.chars() {
            let next = pos.next().unwrap();
            grid[next] = if c == 'S' {
                start = next;
                b'a'
            } else if c == 'E' {
                end = next;
                b'z'
            } else {
                c as u8
            };
        }
    }
    (grid, start, end)
}
