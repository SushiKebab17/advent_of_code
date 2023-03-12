aoc::parts!(1, 2);

fn part_1(input: &[&str]) -> u32 {
    let (mut curr, mut max) = (0, 0);
    for line in input {
        if let Ok(num) = line.parse::<u32>() {
            curr += num;
        } else {
            if curr > max {
                max = curr;
            }
            curr = 0;
        }
    }
    curr.max(max)
}

fn part_2(input: &[&str]) -> u32 {
    let mut curr = 0;
    let (mut max1, mut max2, mut max3) = (0, 0, 0);
    for line in input {
        if let Ok(num) = line.parse::<u32>() {
            curr += num;
        } else {
            if curr > max1 {
                max3 = max2;
                max2 = max1;
                max1 = curr;
            } else if curr > max2 {
                max3 = max2;
                max2 = curr;
            } else if curr > max3 {
                max3 = curr;
            }
            curr = 0;
        }
    }
    if curr > max1 {
        max3 = max2;
        max2 = max1;
        max1 = curr;
    } else if curr > max2 {
        max3 = max2;
        max2 = curr;
    } else if curr > max3 {
        max3 = curr;
    }
    max1 + max2 + max3
}
