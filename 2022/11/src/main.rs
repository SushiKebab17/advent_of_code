aoc::parts!(1, 2);

fn part_1(input: &[&str]) -> u64 {
    run(input, 20, |x| x / 3)
}

fn part_2(input: &[&str]) -> u64 {
    run(input, 10000, |x| x)
}

fn run(input: &[&str], rounds: usize, modifier: fn(u64) -> u64) -> u64 {
    let mut monkeys = parse(input);
    let lcm: u64 = monkeys.iter().map(|x| x.div).product();
    for _ in 0..rounds {
        for i in 0..monkeys.len() {
            let (t, f) = (monkeys[i].t, monkeys[i].f);
            for item in monkeys[i].items.clone() {
                let worry_level = modifier(match &monkeys[i].op {
                    Op::Plus(num) => item + num,
                    Op::Multiply(num) => item * num,
                    Op::Square => item * item,
                });
                if worry_level % monkeys[i].div == 0 {
                    monkeys[t].items.push(worry_level % lcm);
                } else {
                    monkeys[f].items.push(worry_level % lcm);
                }
                monkeys[i].inspected += 1;
            }
            monkeys[i].clear_items();
        }
    }
    calculate_monkey_business(&monkeys)
}

fn parse(input: &[&str]) -> Vec<Monkey> {
    let monkey_num = (input.len() - 6) / 7 + 1;
    let mut monkeys = Vec::new();
    for i in 0..monkey_num {
        monkeys.push(Monkey::from(&input[7 * i..7 * i + 6]));
    }
    monkeys
}

fn calculate_monkey_business(monkeys: &Vec<Monkey>) -> u64 {
    let (mut max1, mut max2) = (0, 0);
    for m in monkeys {
        (max1, max2) = (m.inspected.max(max1), m.inspected.min(max1).max(max2));
    }
    max1 * max2
}

#[derive(Debug, Clone)]
struct Monkey {
    items: Vec<u64>,
    op: Op,
    div: u64,
    t: usize,
    f: usize,
    inspected: u64,
}

impl Monkey {
    fn from(input: &[&str]) -> Self {
        Self {
            items: input[1][18..]
                .split(", ")
                .map(|x| x.parse().unwrap())
                .collect(),
            op: match &input[2][23..24] {
                "+" => Op::Plus(input[2][25..].parse().unwrap()),
                _ => match &input[2][25..].parse::<u64>() {
                    Ok(num) => Op::Multiply(*num),
                    Err(_) => Op::Square,
                },
            },
            div: input[3][21..].parse().unwrap(),
            t: input[4][29..].parse().unwrap(),
            f: input[5][30..].parse().unwrap(),
            inspected: 0,
        }
    }

    fn clear_items(&mut self) {
        self.items = Vec::new();
    }
}

#[derive(Debug, Clone)]
enum Op {
    Plus(u64),
    Multiply(u64),
    Square,
}
