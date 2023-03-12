aoc::parts!(1, 2);

use std::collections::{HashMap, HashSet};

fn part_1(input: &[&str]) -> i32 {
    let (mut solids, mut columns) = parse(input);
    let mut num = 0;
    while sand_rest(&mut solids, &mut columns) {
        num += 1;
    }
    num
}

fn part_2(input: &[&str]) -> i32 {
    let (mut solids, mut columns) = parse(input);
    let mut max_y = 0;
    for &key in columns.keys() {
        max_y = max_y.max(columns[&key]);
    }
    for i in (497 - max_y)..(503 + max_y) {
        solids.insert((i, max_y + 2));
        insert_max_y_in_col(&mut columns, (i, max_y + 2));
    }
    let mut num = 0;
    while sand_rest(&mut solids, &mut columns) {
        num += 1;
    }
    num + 1
}

fn parse(input: &[&str]) -> (HashSet<(u32, u32)>, HashMap<u32, u32>) {
    let mut rocks = HashSet::new();
    let mut columns = HashMap::new();
    for line in input {
        let num_lines = line.split(" -> ").count();
        for i in 0..num_lines - 1 {
            let mut split = line.split(" -> ");
            let coord_1 = split.nth(i).unwrap().split_once(",").unwrap();
            let (x_1, y_1): (u32, u32) = (coord_1.0.parse().unwrap(), coord_1.1.parse().unwrap());
            let coord_2 = split.next().unwrap().split_once(",").unwrap();
            let (x_2, y_2): (u32, u32) = (coord_2.0.parse().unwrap(), coord_2.1.parse().unwrap());
            rocks.insert((x_1, y_1));
            rocks.insert((x_2, y_2));
            insert_max_y_in_col(&mut columns, (x_1, y_1));
            insert_max_y_in_col(&mut columns, (x_2, y_2));
            if x_1 == x_2 {
                for y in y_1.min(y_2) + 1..y_1.max(y_2) {
                    rocks.insert((x_1, y));
                }
            } else {
                for x in x_1.min(x_2) + 1..x_1.max(x_2) {
                    rocks.insert((x, y_1));
                    insert_max_y_in_col(&mut columns, (x, y_1));
                }
            }
        }
    }
    (rocks, columns)
}

fn insert_max_y_in_col(columns: &mut HashMap<u32, u32>, (x, y): (u32, u32)) {
    if let Some(num) = columns.get(&x) {
        if &y > num {
            columns.insert(x, y);
        }
    } else {
        columns.insert(x, y);
    }
}

fn sand_rest(solids: &mut HashSet<(u32, u32)>, columns: &mut HashMap<u32, u32>) -> bool {
    let (mut x, mut y) = (500, 0);
    loop {
        if !columns.contains_key(&x) || columns[&x] < y {
            return false;
        }
        if !solids.contains(&(x, y + 1)) {
            y += 1;
        } else if !solids.contains(&(x - 1, y + 1)) {
            x -= 1;
            y += 1;
        } else if !solids.contains(&(x + 1, y + 1)) {
            x += 1;
            y += 1;
        } else if (x, y) == (500, 0) {
            return false;
        } else {
            solids.insert((x, y));
            return true;
        }
    }
}
