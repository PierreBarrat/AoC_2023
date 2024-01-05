use std::fs;
use std::fmt;
use std::iter;
use std::collections::HashMap;

fn main() {
    let input = "input.txt";
    let input = fs::read_to_string(input).unwrap();
    // let some_line = input.lines().nth(0).unwrap();
    // let (record, damaged) = parse_input_line(&some_line);
    // for (i, s) in record.springs.iter().enumerate() {
    //     println!("Spring {i}: {s}");
    // }
    // println!("{:?}", damaged);
    // for i in 0..record.len() {
    //     let x = 6;
    //     println!("Spring of length {x} at {i}: {}", record.fits(x, i))
    // }
    
    let mut S: Vec<u64> = Vec::new();
    for (i, line) in input.lines().enumerate() {
        let mut record = parse_input_line(&line);
        let s = record.count_fits_from(0, 0);
        println!("{i} -- {line} -- {s}");
        // S += s;
        S.push(s);
        // if s == 0 {break}
    }
    // println!("{S}")
    S.sort();
    // println!("{:?}", S);
    println!("{}", S.iter().sum::<u64>())
}


use Spring::*;
enum Spring {
    Functional,
    Broken,
    Unknown,
}
impl Spring {
    fn broken_or(&self) -> bool {
        match self {
            Functional => false,
            Broken => true,
            Unknown => true,
        }
    }
    fn broken(&self) -> bool {matches!(self, Broken)}
}

// whether a stretch of broken springs of length `n` can fit at `pos`
impl Record {
    fn len(&self) -> usize {self.springs.len()}
    fn fits(&self, n: usize, pos: usize) -> bool {
        if n + pos > self.springs.len() {
            // if there is not enough space for a stretch of size n
            return false
        } else if pos > 0 && self.springs[pos-1].broken() {
            return false
        } else if n + pos < self.springs.len() && self.springs[n+pos].broken() {
            // if the (spring at pos + n) is broken, then the stretch is longer
            return false
        }
        return self.springs[pos..(pos+n)].iter()
            .all(|s| s.broken_or())
    }

    // count number of ways to fit damaged springs starting at pos `start_pos`
    fn count_fits_from(&mut self, damages_from: usize, start_pos: usize) -> u64 {
        // println!("Start pos {start_pos} -- Number of ways to fit {:?}", damaged);
        if damages_from >= self.damaged.len() {
            if start_pos < self.len() && self.springs[start_pos..].iter().any(|s| matches!(s, Broken)) {
                // println!("Start pos {start_pos} {:?} -- Found 0", damaged);
                return 0
            } else {
                // println!("Start pos {start_pos} {:?} -- Found 1", damaged);
                return 1
            }
        }

        let mut S = 0;
        let dsum = self.damaged[damages_from..].iter().sum::<usize>();
        let mut jump_flag = false;
        for pos in start_pos..self.len() {
            if pos + dsum > self.len() || jump_flag {break}
            if matches!(self.springs[pos], Broken) {jump_flag = true}
            if self.fits(self.damaged[damages_from], pos) {
                let next_damage = damages_from + 1;
                let next_pos = pos + self.damaged[damages_from] + 1;
                if let Some(s) = self.cached.get(&(next_damage, next_pos)) {
                    S += s
                } else {
                    let s = self.count_fits_from(next_damage, next_pos);
                    self.cached.insert((next_damage, next_pos), s);
                    S += s
                }
                // S += self.count_fits_from(next_damage, next_pos);
            }
        }
        // println!("Start pos {start_pos} {:?} -- Found {S}", damaged);
        S
    }
}
struct Record {
    springs: Vec<Spring>,
    damaged: Vec<usize>,
    cached: HashMap<(usize, usize), u64>,
}



//  parsing and all
impl Spring {
    fn new(c: &char) -> Spring {
        match c {
            '.' => Functional,
            '#' => Broken,
            '?' => Unknown,
            _ => panic!("Unknown char {c}"),
        }
    }
}

impl fmt::Display for Spring {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Functional => write!(f, "functional"),
            Broken => write!(f, "broken"),
            Unknown => write!(f, "unknown"),
        }
    }
}



fn parse_input_line(line: &str) -> Record {
    let mut parts = line.split_whitespace();
    // first part is the record
    let mut springs: Vec<char> = parts.nth(0)
        .unwrap()
        .chars()
        // .map(|c| Spring::new(&c))
        .collect();
    springs.push('?');
    let mut springs: Vec<Spring> = iter::repeat(springs.iter())
        .take(5)
        .flatten()
        .map(|c| Spring::new(c))
        .collect();
    springs.pop();

    let mut damaged: Vec<usize> = parts.nth(0)
        .unwrap()
        .split(',')
        .map(|s| s.parse::<usize>().unwrap())
        .collect();
    damaged = iter::repeat(damaged.iter())
        .take(5)
        .flatten()
        .map(|&x| x)
        .collect();

    Record{springs, damaged, cached: HashMap::new()}
}

