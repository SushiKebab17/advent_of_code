use std::collections::HashSet;

use aoc::Input;
use grid::prelude::*;

aoc::parts!(1, 2);

fn part_1(input: Input) -> impl ToString {
    let (mut land, start_loc, start_typ) = parse(input);
    let (mut curr_loc, mut curr_typ) = (start_loc, start_typ);
    let mut curr_dir = match curr_typ {
        '-' | 'L' | 'F' => v(1, 0),
        '7' => v(-1, 0),
        _ => v(0, -1),
    };
    next(&mut curr_loc, &mut curr_typ, &mut curr_dir, &mut land);
    let mut perimeter = 1;
    while curr_loc != start_loc {
        next(&mut curr_loc, &mut curr_typ, &mut curr_dir, &mut land);
        perimeter += 1;
    }
    perimeter / 2
}

fn part_2(input: Input) -> impl ToString {
    let (mut land, start_loc, start_typ) = parse(input);
    let (mut curr_loc, mut curr_typ) = (start_loc, start_typ);
    let mut curr_dir = match curr_typ {
        '-' | 'L' | 'F' => v(1, 0),
        '7' => v(-1, 0),
        _ => v(0, -1),
    };
    let mut loop_vectors = HashSet::new();
    let (mut min_x, mut min_y) = (start_loc.x, start_loc.y);
    let (mut max_x, mut max_y) = (start_loc.x, start_loc.y);
    loop_vectors.insert(start_loc);
    next(&mut curr_loc, &mut curr_typ, &mut curr_dir, &mut land);
    while curr_loc != start_loc {
        (min_x, min_y) = (min_x.min(curr_loc.x), min_y.min(curr_loc.y));
        (max_x, max_y) = (max_x.max(curr_loc.x), max_y.max(curr_loc.y));
        loop_vectors.insert(curr_loc);
        next(&mut curr_loc, &mut curr_typ, &mut curr_dir, &mut land);
    }
    land[start_loc] = start_typ;
    let mut last_corner = None;
    let mut inside_points_num = 0;
    for j in min_y..=max_y {
        let mut edge_count = 0;
        for i in min_x..=max_x {
            if !loop_vectors.contains(&v(i as i64, j as i64)) {
                if edge_count % 2 != 0 {
                    inside_points_num += 1;
                }
            } else {
                match land[v(i as i64, j as i64)] {
                    '|' => edge_count += 1,
                    c @ ('L' | 'F') => {
                        if last_corner.is_none() {
                            last_corner = Some(c)
                        }
                    }
                    'J' => {
                        if last_corner == Some('F') {
                            edge_count += 1;
                        }
                        last_corner = None;
                    }
                    '7' => {
                        if last_corner == Some('L') {
                            edge_count += 1;
                        }
                        last_corner = None;
                    }
                    _ => (),
                }
            }
        }
    }
    inside_points_num
}

fn parse(input: Input) -> (Grid<char>, Vector, char) {
    let mut land = Grid::new(input[0].len() as i64, input.len() as i64, '.');
    let (mut loc, mut typ) = (v(0, 0), '.');
    for (x, line) in input.lines().enumerate() {
        for (y, ch) in line.char_indices() {
            land[v(y as i64, x as i64)] = ch;
            if ch == 'S' {
                loc = v(y as i64, x as i64);
                let mut pipe_dir = [false; 4]; // east, north, west, south
                for (i, &diff) in ORTHOGONAL.iter().enumerate() {
                    let adj = diff + v(y as i64, x as i64);
                    let (j, k) = (adj.y as usize, adj.x as usize);
                    if land.in_bounds(adj) {
                        let adj_ch = input[j].chars().nth(k).unwrap();
                        match i {
                            0 => {
                                if adj_ch == '-' || adj_ch == 'J' || adj_ch == '7' {
                                    pipe_dir[0] = true;
                                }
                            }
                            1 => {
                                if adj_ch == '|' || adj_ch == '7' || adj_ch == 'F' {
                                    pipe_dir[1] = true;
                                }
                            }
                            2 => {
                                if adj_ch == '-' || adj_ch == 'L' || adj_ch == 'F' {
                                    pipe_dir[2] = true;
                                }
                            }
                            3 => {
                                if adj_ch == '|' || adj_ch == 'L' || adj_ch == 'J' {
                                    pipe_dir[3] = true;
                                }
                            }
                            _ => (),
                        }
                    }
                }
                typ = if pipe_dir[1] == true && pipe_dir[3] == true {
                    '|'
                } else if pipe_dir[0] == true && pipe_dir[2] == true {
                    '-'
                } else if pipe_dir[0] == true && pipe_dir[1] == true {
                    'L'
                } else if pipe_dir[1] == true && pipe_dir[2] == true {
                    'J'
                } else if pipe_dir[2] == true && pipe_dir[3] == true {
                    '7'
                } else if pipe_dir[3] == true && pipe_dir[0] == true {
                    'F'
                } else {
                    '.'
                }
            }
        }
    }
    (land, loc, typ)
}

fn next(loc: &mut Vector, typ: &mut char, dir: &mut Vector, land: &mut Grid<char>) {
    *loc += *dir;
    *typ = land[*loc];
    *dir = match typ {
        '|' => {
            if *dir == v(0, -1) {
                v(0, -1)
            } else {
                v(0, 1)
            }
        }
        '-' => {
            if *dir == v(1, 0) {
                v(1, 0)
            } else {
                v(-1, 0)
            }
        }
        'L' => {
            if *dir == v(-1, 0) {
                v(0, -1)
            } else {
                v(1, 0)
            }
        }
        'J' => {
            if *dir == v(0, 1) {
                v(-1, 0)
            } else {
                v(0, -1)
            }
        }
        '7' => {
            if *dir == v(1, 0) {
                v(0, 1)
            } else {
                v(-1, 0)
            }
        }
        'F' => {
            if *dir == v(-1, 0) {
                v(0, 1)
            } else {
                v(1, 0)
            }
        }
        _ => *dir,
    }
}
