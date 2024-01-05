use std::collections::HashMap;
use std::fs;
use std::cmp::Ordering;

#[derive(PartialEq, PartialOrd, Eq, Ord, Clone)]
enum HandType {
    High,
    OnePair,
    TwoPairs,
    Three,
    Full,
    Four,
    Five,
}
impl HandType {
    fn description(&self) -> &str {
        match self {
            Five => "Five of a kind",
            Four => "Four of a kind",
            Full => "Full house",
            Three => "Three of a kind",
            TwoPairs => "Two pairs",
            OnePair => "One pair",
            High => "High card",
        }
    }
}

use HandType::*;



#[derive(Eq)]
#[derive(Ord)]
struct Hand {
    cards: [u32; 5],
    kind: HandType,
    bet: u32,
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool{
        (self.kind == other.kind) && (self.cards == other.cards)
    }
}
impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.kind < other.kind {
            return Some(Ordering::Less)
        } else if self.kind > other.kind {
            return Some(Ordering::Greater)
        }

        for (c1, c2) in self.cards.iter().zip(other.cards.iter()) {
            if c1 != c2 {return Some(c1.cmp(&c2))}
        }

        Some(Ordering::Equal)
    }
}




fn main() {
    // let hand1 = [3, 2, 10, 3, 13];
    let input = "input.txt";

    let mut hands = parse_input(&input);
    hands.sort();

    for hand in hands.iter() {
        println!("{:?}, {}, {}", hand.cards, hand.kind.description(), hand.bet)
    }
    let result: u32 = hands.iter()
        .enumerate()
        .map(|(i, hand)| (i as u32 +1)*hand.bet)
        .sum();
    println!("{result}")
}



fn hand_type(hand: &[u32; 5]) -> HandType {
    let mut map = card_map(&hand);
    // construct an optimal map by replacing jokers
    let n_jokers: u32 = *map.get(&1).unwrap_or(&0);
    if n_jokers > 0 {
        map.remove(&1);
        if map.is_empty() {
            // had only jokers -- replace them with whatever to get Five of a kind
            return Five
        } else {
            // add all jokers to the most frequent value in the map
            // this should improve the type as much as possible
            let top_val = map.values_mut().max().unwrap();
            *top_val = *top_val + n_jokers
        }
    }

    match map.len() {
        1 => Five,
        2 => {
            // Four or Full
            let &first_count = map.values().nth(0).unwrap();
            if (first_count == 1) || (first_count == 4) {
                Four
            } else { // Must be 2 or 3
                Full
            }
        }
        3 => {
            // Three or TwoPairs
            // vals can be 311 or 221 
            if map.values().product::<u32>() == 3 {
                Three
            } else {
                TwoPairs
            }
        }
        4 => OnePair,
        5 => High,
        _ => panic!("Card map with more than 5 fields"),
    }
}
fn card_map(hand: &[u32; 5]) -> HashMap<u32, u32> {
    let mut map = HashMap::new();
    for &card in hand {
        let count = map.entry(card).or_insert(0);
        *count += 1
    }
    map
}


// PARSING
impl Hand {
    fn new(line: &str) -> Hand {
        let mut vals = line.split_whitespace();

        let cards = hand_string_to_vec(vals.nth(0).unwrap());
        let bet = vals.nth(0).unwrap().parse::<u32>().unwrap();

        Hand {
            cards,
            kind: hand_type(&cards),
            bet,
        }
    }
}
fn parse_input(file: &str) -> Vec<Hand> {
    let mut hands: Vec<[u32; 5]> = Vec::new();
    let mut bets: Vec<u32> = Vec::new();

    let handle = fs::read_to_string(file).expect("Unable to read file");
    handle.lines()
        .map(|l| Hand::new(&l))
        .collect()
}
fn hand_string_to_vec(hand_string: &str) -> [u32; 5] {
    let mut hand = [0, 0, 0, 0, 0];
    for (i, c) in hand_string.chars().enumerate() {
        hand[i] = match c.to_digit(10) {
            Some(val) => val,
            None => match c {
                'A' => 14,
                'K' => 13,
                'Q' => 12,
                'J' => 1,
                'T' => 10,
                _ => panic!("Got {c} in {hand_string}")
            }
        }
    }
    hand
}