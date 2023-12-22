use std::collections::HashMap;

use aoc::{Input, Parse};

aoc::parts!(1);

fn part_1(input: Input) -> impl ToString {
    let mut total = 0;
    for line in input {
        let mut parser = line.as_parser();
        let springs = parser.before(" ");
        let contig_groups: Vec<u32> = parser.rest().split(",").map(|s| s.parse_uw()).collect();
        // println!(
        //     "{:?}, {:?}, {}\n",
        //     springs,
        //     &contig_groups,
        //     calculate_arrs(springs, &contig_groups, 0, 0, 0)
        // );
        // total += calculate_arrs(springs, &contig_groups, 0, 0, 0);
    }

    // let mut parser = input[20].as_parser();
    // let springs = parser.before(" ");
    // let contig_groups: Vec<u32> = parser.rest().split(",").map(|s| s.parse_uw()).collect();
    // println!(
    //     "{:?}, {:?}, {}\n",
    //     springs,
    //     &contig_groups,
    //     calculate_arrs(springs, &contig_groups, 0, 0, 0)
    // );
    // total += calculate_arrs(springs, &contig_groups, 0, 0, 0);
    total
}

// fn part_2(input: Input) -> impl ToString {
//     0
// }

// fn calculate_arrs(
//     springs: &str,
//     contig_groups: &Vec<u32>,
//     ch_i: usize,
//     contig_i: usize,
//     depth: usize,
// ) -> u32 {
//     if contig_i >= contig_groups.len() {
//         return 1;
//     } else if ch_i >= springs.len() {
//         return 0;
//     }
//     let mut total = 0;
//     let group_len = contig_groups[contig_i];
//     let mut curr = 0;
//     let mut i = ch_i;
//     while i < springs.len() {
//         println!(
//             "{}{}, {}, {}, {}",
//             " ".repeat(depth),
//             springs.chars().nth(i).unwrap(),
//             curr,
//             i,
//             contig_i
//         );
//         match springs.chars().nth(i).unwrap() {
//             '?' | '#' => {
//                 curr += 1;
//                 if curr == group_len {
//                     // println!("recurse");
//                     if let Some('#') = springs.chars().nth(i + 1) {
//                     } else {
//                         if let Some('#') = springs.chars().nth(i - group_len as usize) {
//                         } else {
//                             total += calculate_arrs(
//                                 springs,
//                                 contig_groups,
//                                 i + 2,
//                                 contig_i + 1,
//                                 depth + 1,
//                             );
//                         }
//                     }
//                     i -= group_len as usize - 1;
//                     curr = 0;
//                     println!("{}{}", " ".repeat(depth), total);
//                 }
//             }
//             _ => {
//                 curr = 0;
//             }
//         }
//         i += 1;
//     }
//     total
// }
