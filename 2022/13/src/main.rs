aoc::parts!(1, 2);

use std::cmp::Ordering;
use std::fmt;

fn part_1(input: &[&str]) -> usize {
    let mut total = 0;
    for i in 0..(input.len() + 1) / 3 {
        let (packet_l, packet_r) = (
            Packet::parse_packet(&input[3 * i]),
            Packet::parse_packet(&input[3 * i + 1]),
        );
        if let Ordering::Less = packets_ordering(&packet_l, &packet_r) {
            total += i + 1;
        }
    }
    total
}

fn part_2(input: &[&str]) -> usize {
    let mut list = Vec::new();
    for i in 0..(input.len() + 1) / 3 {
        let (packet_l, packet_r) = (
            Packet::parse_packet(&input[3 * i]),
            Packet::parse_packet(&input[3 * i + 1]),
        );
        list.insert(find_insert_index(&list, &packet_l), packet_l);
        list.insert(find_insert_index(&list, &packet_r), packet_r);
    }
    (find_insert_index(&list, &Packet::parse_packet("[[2]]")) + 1)
        * (find_insert_index(&list, &Packet::parse_packet("[[6]]")) + 2)
}

fn find_insert_index(list: &Vec<Packet>, packet: &Packet) -> usize {
    let mut i = 0;
    for item in list {
        if let Ordering::Greater = packets_ordering(item, packet) {
            break;
        } else {
            i += 1;
        }
    }
    i
}

fn packets_ordering(left: &Packet, right: &Packet) -> Ordering {
    match (left, right) {
        (Packet::List(list_l), Packet::List(list_r)) => {
            for i in 0..list_l.len().min(list_r.len()) {
                let check = packets_ordering(&list_l[i], &list_r[i]);
                if check != Ordering::Equal {
                    return check;
                }
            }
            return list_l.len().cmp(&list_r.len());
        }
        (Packet::List(list_l), Packet::Integer(int_r)) => packets_ordering(
            &Packet::List(list_l.clone()),
            &Packet::List(vec![Packet::Integer(*int_r)]),
        ),
        (Packet::Integer(int_l), Packet::List(list_r)) => packets_ordering(
            &Packet::List(vec![Packet::Integer(*int_l)]),
            &Packet::List(list_r.clone()),
        ),
        (Packet::Integer(int_l), Packet::Integer(int_r)) => return int_l.cmp(int_r),
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Packet {
    List(Vec<Packet>),
    Integer(u8),
}

impl Packet {
    fn new() -> Self {
        Packet::List(Vec::new())
    }

    fn add(&mut self, item: Packet) {
        match self {
            Packet::List(v) => v.push(item),
            _ => (),
        }
    }

    fn parse_packet(input: &str) -> Self {
        let input = &input[1..input.len() - 1];
        let mut packet = Packet::new();
        let mut i = 0;
        while i < input.len() {
            match &input[i..=i] {
                "[" => {
                    let end_b = find_end_bracket(input, i);
                    packet.add(Packet::parse_packet(&input[i..=end_b]));
                    i = end_b;
                }
                c => {
                    if c.is_empty() {
                        return packet;
                    } else if c != "," {
                        let end = end_index_of_number(input, i);
                        packet.add(Packet::Integer(input[i..=end].parse().unwrap()));
                        i = end;
                    }
                }
            }
            i += 1;
        }
        packet
    }
}

impl fmt::Display for Packet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Packet::List(v) => {
                write!(f, "[")?;
                for x in 0..v.len() {
                    v[x].fmt(f)?;
                    if x != v.len() - 1 {
                        write!(f, ",")?;
                    }
                }
                write!(f, "]")?;
                Ok(())
            }
            Packet::Integer(i) => write!(f, "{}", i),
        }
    }
}

fn find_end_bracket(expression: &str, i_first: usize) -> usize {
    let mut index = i_first;
    let mut counter = 1;

    while counter >= 1 {
        index += 1;
        match expression.chars().nth(index).unwrap() {
            '[' => counter += 1,
            ']' => counter -= 1,
            _ => (),
        }
    }

    index
}

fn end_index_of_number(expr: &str, i_first: usize) -> usize {
    let mut x = i_first;
    'outer: while x + 1 < expr.len() {
        while let Ok(_) = &expr[x + 1..=x + 1].parse::<usize>() {
            x += 1;
            continue 'outer;
        }
        break;
    }
    x
}
