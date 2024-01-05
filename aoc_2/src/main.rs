use std::fs;
use std::env;
use std::process;

fn main() {
    let file = env::args().nth(1).unwrap_or_else(|| {
        println!("Need at least one argument");
        process::exit(1);
    }).clone();
    println!("Using input file {file}");    

    let target_game = SetReveal{red: 12, green: 13, blue: 14};

    let result: usize = fs::read_to_string(&file)
        .expect("Could not read file")
        .lines()
        .enumerate()
        .map(|(i, line)|{
            let reveals = parse_line(line);
            if target_game.is_possible(&reveals) {i+1} else {0}
        })
        .sum();
    println!("The result for part 1 is {result}");

    let result: u32 = fs::read_to_string(&file)
        .expect("Could not read file")
        .lines()
        .map(|line| min_possible(&parse_line(line)))
        .map(|game| game.power())
        .sum();
    println!("The result for part 2 is {result}")
}

struct SetReveal {
    red: u32,
    green: u32,
    blue: u32,
}

fn min_possible(v: &Vec<SetReveal>) -> SetReveal {
    SetReveal{
        red: v.into_iter().max_by(|x, y| x.red.cmp(&y.red)).expect("Empty game").red,
        green: v.into_iter().max_by(|x, y| x.green.cmp(&y.green)).expect("Empty game").green,
        blue: v.into_iter().max_by(|x, y| x.blue.cmp(&y.blue)).expect("Empty game").blue,
    }
}

impl SetReveal {
    fn power(&self) -> u32 {
        self.red * self.green * self.blue
    }
    fn is_possible(&self, v: &Vec<SetReveal>) -> bool {
        !v.into_iter().any(|x| {
            if x.red > self.red {
                // println!("Found {} red, higher than {}", x.red, self.red);
                true
            } else if x.green > self.green {
                // println!("Found {} green, higher than {}", x.green, self.green);
                true
            } else if x.blue > self.blue {
                // println!("Found {} blue, higher than {}", x.blue, self.blue);
                true
            } else {
                false
            }
        })
    }
}
fn parse_line(line: &str) -> Vec<SetReveal> {
    let mut v: Vec<SetReveal> = Vec::new();
    for set in line.split(&[':', ';']).skip(1) {
        let reveal = match parse_set_reveal(set) {
            Some(val) => val,
            None => panic!("Could not parse set {set}"),
        };
        v.push(reveal)
    }
    v
}
fn parse_set_reveal(reveal: &str) -> Option<SetReveal> {
    let mut out = SetReveal{red:0, green:0, blue:0};
    for r in reveal.split(',') {
        // r is of the form "22 blue"
        let mut s = r.split_whitespace();

        let n = match s.next() {
            Some(x) => {
                match x.parse::<u32>() {
                    Ok(val) => val,
                    Err(e) => {
                        println!("{}", e);
                        return None
                    }
                }
            },
            None => {
                println!("Found no element in {:?}", s);
                return None
            }
        };

        match s.next() {
            Some("red") => out.red += n,
            Some("green") => out.green += n,
            Some("blue") => out.blue += n,
            Some(x) => {
                println!("{x} is not a known colour");
                return None
            }
            None => {
                println!("Not enough elements in {reveal}");
                return None
            }
        };
    }
    Some(out)
}