use aoc::{Input, Parse};

aoc::parts!(1, 2);

fn part_1(input: Input) -> impl ToString {
    let (seeds, maps) = parse(input);
    let mut min = u32::MAX;
    for seed in seeds {
        let mut curr_val = seed;
        for full_mapping in &maps {
            curr_val = Mapping::map(full_mapping, curr_val);
        }
        min = min.min(curr_val);
    }
    min
}

fn part_2(input: Input) -> impl ToString {
    let (seeds, maps) = parse(input);
    let mut seed_ranges = SeedRange::change_into_ranges(&seeds);
    let mut min = u32::MAX;
    // println!("{:?}", &seed_ranges);
    for full_mapping in &maps {
        Mapping::map_seed_ranges(full_mapping, &mut seed_ranges);
        // println!("{:?}\n\n", &seed_ranges);
    }
    for seed in &seed_ranges {
        min = min.min(seed.value);
    }
    // println!("{}", min);
    min
}

fn parse(input: Input) -> (Vec<u32>, [Vec<Mapping>; 7]) {
    let mut lines = input.lines();
    let seeds: Vec<u32> = lines
        .next()
        .unwrap()
        .as_parser()
        .after(": ")
        .split(" ")
        .map(|x| x.parse_uw())
        .collect();
    let mut maps: [Vec<Mapping>; 7] = [(); 7].map(|_| Vec::new());
    let mut map_num = 0;
    let mut is_title = false;
    for line in lines {
        if !is_title {
            if line.is_empty() {
                is_title = true;
                continue;
            }
            let nums = line.uints::<3, u32>();
            maps[map_num - 1].push(Mapping {
                dest: nums[0],
                start: nums[1],
                range: nums[2],
            });
        } else {
            map_num += 1;
            is_title = false;
        }
    }
    (seeds, maps)
}

#[derive(Debug)]
struct Mapping {
    dest: u32,
    start: u32,
    range: u32,
}

impl Mapping {
    fn map_single(&self, n: u32) -> Option<u32> {
        if n >= self.start && n < self.start + self.range {
            Some(self.dest + n - self.start)
        } else {
            None
        }
    }

    fn map(mappings: &Vec<Mapping>, n: u32) -> u32 {
        let mut val = n;
        for mapping in mappings {
            if let Some(x) = mapping.map_single(val) {
                val = x;
                break;
            }
        }
        val
    }

    fn map_seed_ranges(mappings: &Vec<Mapping>, seed_ranges: &mut Vec<SeedRange>) {
        let n = seed_ranges.len();
        for _ in 0..n {
            let mut seed_range = seed_ranges.remove(0);
            let mut all_mapped = false;
            for mapping in mappings {
                // println!(
                //     "SR: {}, {}",
                //     seed_range.value,
                //     seed_range.value + seed_range.range - 1
                // );
                // println!(
                //     "M: {}, {} to {}",
                //     mapping.start,
                //     mapping.start + mapping.range - 1,
                //     mapping.dest
                // );

                if (mapping.start..(mapping.start + mapping.range)).contains(&seed_range.value) {
                    let new_range =
                        (seed_range.range).min(mapping.start + mapping.range - seed_range.value);
                    seed_ranges.push(SeedRange {
                        value: mapping.map_single(seed_range.value).unwrap(),
                        range: new_range,
                    });
                    if seed_range.value + seed_range.range <= mapping.start + mapping.range {
                        all_mapped = true;
                        break;
                    }
                    seed_range = SeedRange {
                        value: mapping.start + mapping.range,
                        range: seed_range.value + seed_range.range - mapping.start - mapping.range,
                    };
                    // println!("{:?}", seed_range);
                } else if (mapping.start..(mapping.start + mapping.range))
                    .contains(&(seed_range.value + seed_range.range - 1))
                {
                    seed_ranges.push(SeedRange {
                        value: mapping.map_single(mapping.start).unwrap(),
                        range: seed_range.value + seed_range.range - mapping.start,
                    });
                    seed_range = SeedRange {
                        value: seed_range.value,
                        range: mapping.start - seed_range.value,
                    };
                }
            }
            if !all_mapped {
                seed_ranges.push(seed_range);
            }
        }
    }
}

#[derive(Debug)]
struct SeedRange {
    value: u32,
    range: u32,
}

impl SeedRange {
    fn change_into_ranges(seeds: &Vec<u32>) -> Vec<Self> {
        let mut seed_ranges = vec![];
        for i in 0..(seeds.len() / 2) {
            seed_ranges.push(Self {
                value: seeds[2 * i],
                range: seeds[2 * i + 1],
            });
        }
        seed_ranges
    }
}
