aoc::parts!(1, 2);

fn part_1(input: &[&str]) -> String {
    evaluate_message(input, rearr_one)
}

fn part_2(input: &[&str]) -> String {
    evaluate_message(input, rearr_two)
}

fn evaluate_message(
    input: &[&str],
    rearr: fn(&mut Vec<Vec<char>>, (usize, usize, usize)),
) -> String {
    let moves_start = input.iter().copied().position(str::is_empty).unwrap() + 1;
    let mut stacks: Vec<Vec<char>> = vec![Vec::new(); (input[0].len() + 1) / 4];
    for x in (0..moves_start - 1).rev() {
        for (i, c) in input[x].chars().enumerate() {
            if c.is_alphabetic() {
                stacks[(i - 1) / 4].push(c);
            }
        }
    }
    let mut lines = input[moves_start..].iter();
    while let Some(line) = lines.next() {
        let mut split = line.split(" ");
        rearr(
            &mut stacks,
            (
                split.nth(1).unwrap().parse().unwrap(),
                split.nth(1).unwrap().parse::<usize>().unwrap() - 1,
                split.nth(1).unwrap().parse::<usize>().unwrap() - 1,
            ),
        );
    }
    let mut message = String::new();
    for stack in stacks {
        message = format!("{}{}", message, stack.last().unwrap());
    }
    message
}

fn rearr_one(stacks: &mut Vec<Vec<char>>, (num_moves, from, to): (usize, usize, usize)) {
    for _ in 0..num_moves {
        let item = stacks[from].pop().unwrap();
        stacks[to].push(item);
    }
}

fn rearr_two(stacks: &mut Vec<Vec<char>>, (num_moves, from, to): (usize, usize, usize)) {
    let size = stacks[from].len();
    let mut items = stacks[from].drain(size - num_moves..).collect();
    stacks[to].append(&mut items);
}
