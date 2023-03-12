aoc::parts!(1, 2);

fn part_1(input: &[&str]) -> u64 {
    generate(input)
        .into_iter()
        .filter(|d| *d <= 100_000)
        .sum::<u64>()
}

fn part_2(input: &[&str]) -> u64 {
    let mut sizes = generate(input);
    let max = sizes.pop().unwrap();
    sizes
        .into_iter()
        .filter(|d| *d >= max - 40_000_000)
        .min()
        .unwrap()
}

fn generate(input: &[&str]) -> Vec<u64> {
    let mut stack = Vec::new();
    let mut sizes = Vec::new();
    for line in input {
        match line.split_once(" ").unwrap() {
            ("$", c) => {
                if let Some((_, x)) = c.split_once(" ") {
                    if x == ".." {
                        sizes.push(update_sizes(&mut stack))
                    } else {
                        stack.push(0)
                    }
                }
            }
            (item, _) => {
                if let Ok(num) = item.parse::<u64>() {
                    *stack.last_mut().unwrap() += num;
                }
            }
        }
    }
    while stack.len() >= 2 {
        sizes.push(update_sizes(&mut stack));
    }
    sizes.push(stack.pop().unwrap());
    sizes
}

fn update_sizes(sizes: &mut Vec<u64>) -> u64 {
    let recent = sizes.pop().unwrap();
    *sizes.last_mut().unwrap() += recent;
    recent
}
