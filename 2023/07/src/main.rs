use std::collections::HashMap;

use aoc::{Input, IterUnwrap, Parse};

aoc::parts!(1, 2);

fn part_1(input: Input) -> impl ToString {
    let mut all_hands = parse(input, 1);
    all_hands.sort();
    let mut total = 0;
    for (i, hand) in all_hands.iter().enumerate() {
        total += (all_hands.len() - i) as u32 * hand.bid;
    }
    total
}

fn part_2(input: Input) -> impl ToString {
    let mut all_hands = parse(input, 2);
    all_hands.sort();
    let mut total = 0;
    for (i, hand) in all_hands.iter().enumerate() {
        total += (all_hands.len() - i) as u32 * hand.bid;
    }
    total
}

fn parse(input: Input, part: u32) -> Vec<Hand> {
    input.lines().map(|l| Hand::new(l, part)).collect()
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Hand {
    hand_type: HandType,
    hand: [Card; 5],
    bid: u32,
}

impl Hand {
    fn new(line: &str, part: u32) -> Self {
        let mut parser = line.as_parser();
        let mut hand = [Card::Ace; 5];
        for (i, ch) in parser.before(" ").char_indices() {
            hand[i] = Card::new(ch, part).unwrap();
        }
        let hand_type = HandType::get_type(hand, part).unwrap();
        let bid = parser.rest().parse_uw();
        Self {
            hand_type,
            hand,
            bid,
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
enum HandType {
    FiveKind,  // 1 distinct
    FourKind,  // 2 distinct
    Full,      // 2 distinct
    ThreeKind, // 3 distinct
    TwoPair,   // 3 distinct
    OnePair,   // 4 distinct
    High,      // 5 distinct
}

impl HandType {
    fn get_type(hand: [Card; 5], part: u32) -> Option<Self> {
        let mut count = HashMap::new();
        for card in hand {
            count.entry(card).and_modify(|c| *c += 1).or_insert(1);
        }
        if part == 2 {
            if let Some(&joker_count) = count.get(&Card::Joker) {
                if joker_count < 5 {
                    count.remove(&Card::Joker);
                    *count.values_mut().max().unwrap() += joker_count;
                }
            }
        }
        match count.keys().len() {
            1 => Some(Self::FiveKind),
            2 => {
                let card = count.keys().next_uw();
                if count[card] == 1 || count[card] == 4 {
                    Some(Self::FourKind)
                } else {
                    Some(Self::Full)
                }
            }
            3 => {
                let mut highest_count = 0;
                for card in count.keys() {
                    highest_count = highest_count.max(count[card]);
                }
                if highest_count == 3 {
                    Some(Self::ThreeKind)
                } else {
                    Some(Self::TwoPair)
                }
            }
            4 => Some(Self::OnePair),
            5 => Some(Self::High),
            _ => None,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
enum Card {
    Ace,
    King,
    Queen,
    Jack,
    Ten,
    Nine,
    Eight,
    Seven,
    Six,
    Five,
    Four,
    Three,
    Two,
    One,
    Joker,
}

impl Card {
    fn new(c: char, part: u32) -> Option<Self> {
        match c {
            'A' => Some(Self::Ace),
            'K' => Some(Self::King),
            'Q' => Some(Self::Queen),
            'J' => {
                if part == 1 {
                    Some(Self::Jack)
                } else {
                    Some(Self::Joker)
                }
            }
            'T' => Some(Self::Ten),
            '9' => Some(Self::Nine),
            '8' => Some(Self::Eight),
            '7' => Some(Self::Seven),
            '6' => Some(Self::Six),
            '5' => Some(Self::Five),
            '4' => Some(Self::Four),
            '3' => Some(Self::Three),
            '2' => Some(Self::Two),
            '1' => Some(Self::One),
            _ => None,
        }
    }
}
