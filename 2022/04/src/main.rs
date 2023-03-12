aoc::parts!(1, 2);

fn part_1(input: &[&str]) -> u32 {
    let mut total = 0;
    for line in input {
        let (min1, max1, min2, max2) = parse_line(line);
        if (min1 <= min2 && max1 >= max2) || (min1 >= min2 && max1 <= max2) {
            total += 1;
        }
    }
    total
}

fn part_2(input: &[&str]) -> u32 {
    let mut total = 0;
    for line in input {
        let (min1, max1, min2, max2) = parse_line(line);
        if min1 <= max2 && max1 >= min2 {
            total += 1;
        }
    }
    total
}

fn parse_line(line: &str) -> (u32, u32, u32, u32) {
    let mut split = line.split(",");
    let mut range1 = split.next().unwrap().split("-");
    let mut range2 = split.next().unwrap().split("-");
    let min1 = range1.next().unwrap().parse().unwrap();
    let max1 = range1.next().unwrap().parse().unwrap();
    let min2 = range2.next().unwrap().parse().unwrap();
    let max2 = range2.next().unwrap().parse().unwrap();
    (min1, max1, min2, max2)
}
