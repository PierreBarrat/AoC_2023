use std::env;
use std::fs;
use std::process;
use regex::Regex;

fn main() {
    let file = env::args().nth(1).unwrap_or_else(|| {
        println!("Need to provide file as first argument");
        process::exit(1);
    });
    println!("Using file {file}");

    // for (i, line) in fs::read_to_string(&file).unwrap().lines().enumerate() {
    //     println!("{i} - {line}");
    //     ParsedLine::new(&line);
    // }

    let parsed_lines: Vec<ParsedLine> = fs::read_to_string(&file)
        .expect("Unable to read input file")
        .lines()
        .map(|l| ParsedLine::new(&l))
        .collect();

    //  Part 1
    let result: u32 = parsed_lines.iter().enumerate().map(|(i, line)|{
        // println!("Line {i}");
        let part_numbers = if (i > 0) && (i < parsed_lines.len() - 1){
            line.find_part_numbers(&parsed_lines[i-1], &parsed_lines[i+1])
        } else if i == 0 {
            line.find_part_numbers(&parsed_lines[i+1], &parsed_lines[i+1])
        } else {
            line.find_part_numbers(&parsed_lines[i-1], &parsed_lines[i-1])
        };
        // dbg!(&part_numbers);
        part_numbers.iter().sum::<u32>()
    }).collect::<Vec<u32>>()
    .iter()
    .sum();
    println!("Result for part 1: {}", result);

    // Part 2
    let result: u32 = parsed_lines.iter().enumerate().map(|(i, line)| {
        let gear_values = if (i > 0) && (i < parsed_lines.len() - 1){
            line.get_gear_values(&parsed_lines[i-1], &parsed_lines[i+1])
        } else if i == 0 {
            line.get_gear_values(&parsed_lines[i+1], &parsed_lines[i+1])
        } else {
            line.get_gear_values(&parsed_lines[i-1], &parsed_lines[i-1])
        };
        gear_values.iter().sum::<u32>()
    }).sum();
    println!("Result for part 2: {}", result);
}

struct Number {
    val: u32,
    start: usize,
    end: usize,
}
struct Symbol {
    c: char,
    pos: usize,
}
struct ParsedLine {
    numbers: Vec<Number>,
    symbols: Vec<Symbol>,
}


fn are_adjacent(n: &Number, s: &Symbol) -> bool {
    let i_s = s.pos;
    let s = n.start;
    let e = n.end;
    ((s < 2) || (i_s > s-2)) && (i_s <= e)
}

impl Number {
    fn is_part_number(
        &self, 
        self_symbols: &Vec<Symbol>, // on the same line
        above_symbols: &Vec<Symbol>, // on line above
        below_symbols: &Vec<Symbol>, // one line below
    ) -> bool {
        let mut symbols = above_symbols.iter()
            .chain(below_symbols.iter())
            .chain(self_symbols.iter());
        symbols.any(|symbol| are_adjacent(self, symbol))
    }
}

impl Symbol {
    // find all numbers adjacent to Symbol and return their values as a vec
    fn adjacent_numbers(
        &self, 
        self_numbers: &Vec<Number>,
        above_numbers: &Vec<Number>,
        below_numbers: &Vec<Number>,
    ) -> Vec<u32> {
        let mut numbers = self_numbers.iter()
            .chain(above_numbers.iter())
            .chain(below_numbers.iter());

        numbers.filter(|number| are_adjacent(number, self))
            .map(|number| number.val)
            .collect()
    }
}

impl ParsedLine {
    fn find_part_numbers(&self, above: &ParsedLine, below: &ParsedLine) -> Vec<u32> {
        self.numbers.iter()
            .filter(|&n| n.is_part_number(&self.symbols, &above.symbols, &below.symbols))
            .map(|n| n.val)
            .collect()
    }
    fn get_gear_values(&self, above: &ParsedLine, below: &ParsedLine) -> Vec<u32> {
        self.symbols.iter()
            .map(|symbol| symbol.adjacent_numbers(&self.numbers, &above.numbers, &below.numbers))
            .filter(|nbs| nbs.len() == 2)
            .map(|nbs| nbs[0] * nbs[1])
            .collect()
    }
    fn new(line: &str) -> ParsedLine {
        // all numbers
        let re = Regex::new(r"\d+").unwrap();
        let numbers: Vec<Number> = re.find_iter(line).map(|m|{
            Number{
                val: m.as_str().parse::<u32>().expect("Unable to parse number"),
                start: m.start(),
                end: m.end(),
            }
        }).collect();

        // all symbols (not . and not digit)
        let re = Regex::new(r"[^\.\d]+").unwrap();
        let symbols: Vec<Symbol> = re.find_iter(line)
            .map(|m| Symbol{
                c: m.as_str().chars().nth(0).unwrap(),
                pos: m.start(),
            }).collect();


        ParsedLine{
            numbers: numbers,
            symbols: symbols,
        }
    }
}

