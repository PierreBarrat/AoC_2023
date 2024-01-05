use std::cmp;
use std::env;
use std::fs;
use std::process;
use regex::Regex;

fn main() {
    let file = env::args().nth(1).unwrap_or_else(|| {
        println!("Need to provide at least one argument");
        process::exit(1);
    });
    let file = fs::read_to_string(file).expect("Problem in reading file");
    let data: Vec<&str> = file.lines()
        .collect();

    let base: u32 = 2;
    let mut total_score = 0;
    for line in &data {
        let card = Card::new(&line);
        let score = card.compute_score();
        let score = if score > 0 { 
            base.pow(score - 1)
        } else {
            0
        };
        total_score += score;
    }
    println!("Part 1: total score = {total_score}");

    let cards: Vec<Card> = data.iter()
        .map(|&l| Card::new(&l))
        .collect();
    let mut card_count = vec![1; cards.len()];
    for (i, card) in cards.iter().enumerate() {
        let score: usize = card.compute_score().try_into().unwrap();
        // println!("Card {} won {score} times", i+1);

        for j in ((i+1)..=(i+score)).filter(|&x| x < cards.len()) {
            card_count[j] += card_count[i];
            // println!("Got a copy of card {}", j+1);
        }
    }

    for (i, c) in card_count.iter().enumerate() {
        println!("Got {c} copies of {}", i+1)
    }
    println!("Result for part 2: {}", card_count.iter().sum::<u32>());
}



struct Card {
    index: u32,
    winning_numbers: Vec<u32>,
    drawn_numbers: Vec<u32>,
}

impl Card {
    // fn 

    fn compute_score(&self) -> u32 {
        self.drawn_numbers.iter()
            .filter(|&n| self.winning_numbers.contains(n))
            .count()
            .try_into()
            .unwrap()
    }
    fn new(line: &str) -> Card {
        let line_split: Vec<&str> = line.split(&[':', '|']).collect();
        if line_split.len() != 3 {
            panic!("Could not parse line {line}");
        }
        let re = Regex::new(r"\d+").unwrap();

        let index: u32 = re.find(&line_split[0])
            .expect("Could not find number in card")
            .as_str()
            .parse()
            .unwrap();

        let winning_numbers: Vec<u32> = re.find_iter(&line_split[1])
            .map(|m| m.as_str().parse::<u32>().unwrap())
            .collect();

        let drawn_numbers: Vec<u32> = re.find_iter(&line_split[2])
            .map(|m| m.as_str().parse::<u32>().unwrap())
            .collect();

        Card{index, winning_numbers, drawn_numbers}
    }
}