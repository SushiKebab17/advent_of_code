use aoc::Input;

aoc::parts!(1, 2);

fn part_1(input: Input) -> impl ToString {
    let mut total = 0;
    for line in input.lines() {
        let mut forward = line.chars();
        let mut first = 0;
        let mut backward = line.chars().rev();
        let mut last = 0;
        while let Some(c) = forward.next() {
            if let Some(digit) = c.to_digit(10) {
                first = digit;
                break;
            }
        }
        while let Some(c) = backward.next() {
            if let Some(digit) = c.to_digit(10) {
                last = digit;
                break;
            }
        }
        total += first * 10 + last;
    }

    total
}

fn part_2(input: Input) -> impl ToString {
    let mut total = 0;
    let char_digits = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    total
}
