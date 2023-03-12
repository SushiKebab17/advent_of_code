aoc::parts!(1, 2);

use std::collections::BTreeMap;
use std::ops::Range;

fn part_1(input: &[&str]) -> i32 {
    let mut non_present_ranges = Vec::new();
    for line in input {
        let (sens_x, sens_y, beac_x, beac_y, distance) = parse_line(line);
        let dy = 2_000_000 - sens_y;
        if dy.abs() <= distance {
            if sens_y + dy == beac_y
                && ((sens_x - distance + dy.abs())..=(sens_x + distance - dy.abs()))
                    .contains(&beac_x)
            {
                non_present_ranges.append(&mut vec![
                    (sens_x - distance + dy.abs())..beac_x,
                    (beac_x + 1)..(sens_x + distance - dy.abs() + 1),
                ]);
            } else {
                non_present_ranges
                    .push((sens_x - distance + dy.abs())..(sens_x + distance - dy.abs() + 1));
            }
        }
    }
    calculate_total_in_range(non_present_ranges)
}

fn part_2(input: &[&str]) -> i64 {
    let mut sensors = Vec::new();
    let mut coord = (0, 0);
    for line in input {
        let (sens_x, sens_y, _, _, distance) = parse_line(line);
        sensors.push(Sensor::new((sens_x, sens_y), distance));
    }
    'outer: for s in &sensors {
        'testing: for dx in -(s.max_distance + 1)..(s.max_distance + 1) {
            let tests = [
                (s.coord.0 + dx, s.coord.1 + (s.max_distance + 1 - dx.abs())),
                (s.coord.0 + dx, s.coord.1 - (s.max_distance + 1 - dx.abs())),
            ];
            for c in tests {
                if is_in_region(c) {
                    for other in &sensors {
                        if other.is_in_boundary(c) {
                            continue 'testing;
                        }
                    }
                    coord = c;
                    break 'outer;
                }
            }
        }
    }
    (coord.0 as i64) * 4_000_000 + coord.1 as i64
}

fn calculate_total_in_range(ranges: Vec<Range<i32>>) -> i32 {
    let mut map: BTreeMap<i32, Range<i32>> = BTreeMap::new();
    for range in ranges {
        if !map.contains_key(&range.start)
            || (map.contains_key(&range.start) && map[&range.start].end < range.end)
        {
            map.insert(range.start, range);
        }
    }
    let mut result = Vec::new();
    let mut current_range = map.iter().next().unwrap().1.clone();
    for (_, range) in map.iter() {
        if current_range.end >= range.start {
            current_range = current_range.start..(current_range.end).max(range.end);
        } else {
            result.push(current_range);
            current_range = range.clone();
        }
    }
    result.push(current_range);
    let mut total = 0;
    for range in &result {
        total += range.end - range.start;
    }
    total
}

struct Sensor {
    coord: (i32, i32),
    max_distance: i32,
}

impl Sensor {
    fn new(coord: (i32, i32), max_distance: i32) -> Self {
        Sensor {
            coord,
            max_distance,
        }
    }

    fn is_in_boundary(&self, coord: (i32, i32)) -> bool {
        let dis = (coord.0 - self.coord.0, coord.1 - self.coord.1);
        dis.0.abs() + dis.1.abs() <= self.max_distance
    }
}

fn is_in_region(coord: (i32, i32)) -> bool {
    (0..=4_000_000).contains(&coord.0) && (0..=4_000_000).contains(&coord.1)
}

fn parse_line(line: &str) -> (i32, i32, i32, i32, i32) {
    let mut split = line.split(": closest beacon is at x=");
    let mut split_sensor = split.next().unwrap().split(", y=");
    let (sens_x, sens_y): (i32, i32) = (
        split_sensor.next().unwrap()[12..].parse().unwrap(),
        split_sensor.next().unwrap().parse().unwrap(),
    );
    let mut split_beacon = split.next().unwrap().split(", y=");
    let (beac_x, beac_y): (i32, i32) = (
        split_beacon.next().unwrap().parse().unwrap(),
        split_beacon.next().unwrap().parse().unwrap(),
    );
    let distance = (sens_x - beac_x).abs() + (sens_y - beac_y).abs();
    (sens_x, sens_y, beac_x, beac_y, distance)
}
