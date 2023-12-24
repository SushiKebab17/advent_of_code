use aoc::Input;

aoc::parts!(1, 2);

fn part_1(input: Input) -> impl ToString {
    let mut total = 0;
    let mut pattern: Vec<Vec<_>> = Vec::new();
    for line in input {
        if !line.is_empty() {
            pattern.push(line.chars().collect());
        } else {
            total += find_vertical(&pattern);
            total += 100 * find_horizontal(&pattern);
            pattern.clear();
        }
    }
    total += find_vertical(&pattern);
    total += 100 * find_horizontal(&pattern);
    pattern.clear();
    total
}

fn part_2(input: Input) -> impl ToString {
    let mut total = 0;
    let mut pattern: Vec<Vec<_>> = Vec::new();
    for line in input {
        if !line.is_empty() {
            pattern.push(line.chars().collect());
        } else {
            total += find_smudged_vertical(&pattern);
            total += 100 * find_smudged_horizontal(&pattern);
            pattern.clear();
        }
    }
    total += find_smudged_vertical(&pattern);
    total += 100 * find_smudged_horizontal(&pattern);
    pattern.clear();
    total
}

fn find_vertical(pattern: &Vec<Vec<char>>) -> u32 {
    let mut reflection_col = 0;
    'outer: for col in 1..pattern[0].len() {
        let mut i: i32 = col as i32 - 1;
        let mut j = col;
        while i >= 0 && j < pattern[0].len() {
            for row in pattern {
                if row[i as usize] != row[j] {
                    continue 'outer;
                }
            }
            i -= 1;
            j += 1;
        }
        reflection_col = col;
    }
    reflection_col as u32
}

fn find_horizontal(pattern: &Vec<Vec<char>>) -> u32 {
    let mut reflection_row = 0;
    'outer: for row in 1..pattern.len() {
        let mut i: i32 = row as i32 - 1;
        let mut j = row;
        while i >= 0 && j < pattern.len() {
            for col in 0..pattern[0].len() {
                if pattern[i as usize][col] != pattern[j][col] {
                    continue 'outer;
                }
            }
            i -= 1;
            j += 1;
        }
        reflection_row = row;
    }
    reflection_row as u32
}

fn find_smudged_horizontal(pattern: &Vec<Vec<char>>) -> u32 {
    let mut reflection_row = 0;
    let mut errors = 0;
    for row in 1..pattern.len() {
        let mut i: i32 = row as i32 - 1;
        let mut j = row;
        while i >= 0 && j < pattern.len() {
            for col in 0..pattern[0].len() {
                if pattern[i as usize][col] != pattern[j][col] {
                    errors += 1;
                }
            }
            i -= 1;
            j += 1;
        }
        if errors == 1 {
            reflection_row = row;
        } else {
            errors = 0;
        }
    }
    reflection_row as u32
}

fn find_smudged_vertical(pattern: &Vec<Vec<char>>) -> u32 {
    let mut reflection_col = 0;
    let mut errors = 0;
    for col in 1..pattern[0].len() {
        let mut i: i32 = col as i32 - 1;
        let mut j = col;
        while i >= 0 && j < pattern[0].len() {
            for row in pattern {
                if row[i as usize] != row[j] {
                    errors += 1;
                }
            }
            i -= 1;
            j += 1;
        }
        if errors == 1 {
            reflection_col = col;
        } else {
            errors = 0;
        }
    }
    reflection_col as u32
}
