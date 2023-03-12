aoc::parts!(1, 2);

fn part_1(input: &[&str]) -> u32 {
    let mut score = 0;
    for line in input {
        score += match &line[..] {
            "A X" => 4,
            "A Y" => 8,
            "A Z" => 3,
            "B X" => 1,
            "B Y" => 5,
            "B Z" => 9,
            "C X" => 7,
            "C Y" => 2,
            "C Z" => 6,
            _ => 0,
        };
    }
    score
}

fn part_2(input: &[&str]) -> u32 {
    let mut score = 0;
    for line in input {
        score += match &line[..] {
            "A X" => 3,
            "A Y" => 4,
            "A Z" => 8,
            "B X" => 1,
            "B Y" => 5,
            "B Z" => 9,
            "C X" => 2,
            "C Y" => 6,
            "C Z" => 7,
            _ => 0,
        };
    }
    score
}
