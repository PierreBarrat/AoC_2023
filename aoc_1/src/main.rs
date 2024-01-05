use std::fs;
use regex::Regex;
use regex::RegexSet;
use std::cmp::Ordering;

fn main() {
    let input = "input.txt";
    let input = fs::read_to_string(input).expect("Could not read input");

    let mut S = 0;
    for line in input.lines() {
        let digits = parse_calibration_value(line);
        match digits {
            Some(tup) => {
                S += 10*tup.0 + tup.1;
            },
            None => println!("Could not parse line {line}"),
        }
    }

    println!("The result is {S}")
}


fn parse_calibration_value_num(line: &str) -> Option<((usize, u32), (usize, u32))> {
    let re_d = Regex::new(r"\d").unwrap();

    let mut matches = re_d.find_iter(line);
    let first_digit = if let Some(m) = matches.nth(0) {
        (
            m.start(), 
            m.as_str().parse::<u32>().expect("Should have been able to parse digit")
        )
    } else {
        // if there is no digit at all, stop here
        return None;
    };
    
    let last_digit = match matches.last() {
        Some(m) => {(
            m.start(), 
            m.as_str().parse::<u32>().expect("Should have been able to parse digit")
        )},
        None => first_digit
    };

    Some((first_digit, last_digit))
}

fn parse_literal_digit(digit: &str) -> Option<u32> {
    match digit {
        "one" => Some(1),
        "two" => Some(2),
        "three" => Some(3),
        "four" => Some(4),
        "five" => Some(5),
        "six" => Some(6),
        "seven" => Some(7),
        "eight" => Some(8),
        "nine" => Some(9),
        _ => None
    }
}
fn parse_calibration_value_literal(line: &str) -> Option<((usize, u32), (usize, u32))> {
    let re_set = [
        r"one",
        r"two",
        r"three",
        r"four",
        r"five",
        r"six",
        r"seven",
        r"eight",
        r"nine",
    ];

    let first_match = re_set.into_iter()
        .map(|re| Regex::new(re).unwrap().find(line))
        .filter_map(|x| x) // the first one, two, etc...
        .min_by(|x, y| {
            let ix = x.start();
            let iy = y.start();
            ix.cmp(&iy)
        });
 
    if let Some(fm) = first_match {
        let fm_start = fm.start();
        let fm_val = parse_literal_digit(fm.as_str())
            .expect("Should have been able to parse literal digit");

        let last_match = re_set.into_iter()
            .map(|re| Regex::new(re).unwrap().find_iter(line).last())
            .filter_map(|x| x) // the first one, two, etc...
            .max_by(|x, y| {
                let ix = x.start();
                let iy = y.start();
                ix.cmp(&iy)
            });

        if let Some(lm) = last_match {
            let lm_start = lm.start();
            let lm_val = parse_literal_digit(lm.as_str())
                .expect("Should have been able to parse literal digit");
            Some(((fm_start, fm_val), (lm_start, lm_val)))
        } else {
            Some(((fm_start, fm_val), (fm_start, fm_val)))
        }

    } else {
        None
    }
    // None
}


fn parse_calibration_value(line: &str) -> Option<(u32, u32)> {

    let num_match = parse_calibration_value_num(line);
    let lit_match = parse_calibration_value_literal(line);

    match (num_match, lit_match) {
        (None, None) => None,
        (Some(x), None) => Some((x.0.1, x.1.1)),
        (None, Some(y)) => Some((y.0.1, y.1.1)),
        (Some(x), Some(y)) => Some(compare_matches(x, y)),
    }
}   

fn compare_matches(
    x: ((usize, u32), (usize, u32)), y: ((usize, u32), (usize, u32))
) -> (u32, u32) {
    let first_digit = match (x.0.0).cmp(&y.0.0) {
        Ordering::Less => x.0.1,
        Ordering::Equal => panic!("Impossible situation"),
        Ordering::Greater => y.0.1
    };
    let last_digit = match (x.1.0).cmp(&y.1.0) {
        Ordering::Less => y.1.1,
        Ordering::Equal => panic!("Impossible situation"),
        Ordering::Greater => x.1.1
    };

    (first_digit, last_digit)
}