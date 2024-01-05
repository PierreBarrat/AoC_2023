use std::collections::HashMap; 
use std::fmt;
use std::fs;

fn main() {
    let input = "input.txt";
    let (instructions, graph) = parse_input(&input);
    // checking I parsed correctly
    // for node in graph.map.values() {
    //     println!("{node}")
    // }
    // for d in instructions.iter() {
    //     match d {
    //         Right => println!("right"),
    //         Left => println!("left"),
    //     }
    // }

    // 
    // let steps = graph.parallel_navigate(&instructions);
    let positions = graph.initial_positions();
    for pos in positions {
        println!("Starting from {}", pos);
        let steps = graph.navigate(&instructions, pos);
        println!("It took {steps} steps");
    }
}

enum Direction {
    Left,
    Right,
}
use Direction::*;
impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Right => write!(f, "right"),
            Left => write!(f, "left"),
        }
    }
}

struct Node {
    name: String, // owns its name
    left: String,
    right: String,
}

impl Node {
    fn new(name: &str, left: &str, right: &str) -> Node {
        Node{
            name: name.to_string(), 
            left: left.to_string(), 
            right: right.to_string(),
        }
    }
    fn go(&self, dir: &Direction) -> &str {
        match dir {
            Left => &self.left,
            Right => &self.right,
        }
    }
}
impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} --> {} / {}", self.name, self.left, self.right)
    }
}

struct Graph {
    map: HashMap<String, Node>
}

impl Graph {
    fn new() -> Graph {
        Graph{
            map: HashMap::new()
        }
    }
    fn add(&mut self, name: &str, left: &str, right: &str) -> Option<Node> {
        self.map.insert(name.to_string(), Node::new(name, left, right))
    }

    fn initial_positions(&self) -> Vec<&str> {
        self.map.keys()
            .filter(|name| is_start_name(name))
            .map(|x| x.as_str())
            .collect()
    }
    
    fn go(&self, position: &str, dir: &Direction) -> Option<&str> {
        let node = self.map.get(position).expect("Unknown position");

        match dir {
            Left => self.map.get(&node.left),
            Right => self.map.get(&node.right),
        }.map(|n| n.name.as_str())
    }

    fn parallel_navigate(&self, instructions: &Vec<Direction>) -> u64 {
        let mut positions = self.initial_positions();
        let mut count_iter = 0;
        for dir in instructions.iter().cycle() {
            if count_iter > 1_000_000_000 {
                println!("Stopped because too long");
                break;
            } else if positions.iter().all(|pos| is_end_name(pos)) {
                println!("Found {:?}", positions);
                break;
            }
            count_iter += 1;
            // println!("At {:?}, going {}", positions, &dir);
            positions.iter_mut().for_each(|pos| {
                *pos = self.go(pos, &dir).unwrap()
            });
        }
        count_iter
    }

    fn navigate(&self, instructions: &Vec<Direction>, start: &str) -> u32 {
        let mut position = start;
        let mut count_iter = 0;
        let mut reached = 0;
        for dir in instructions.iter().cycle() {
            // println!("At {}, going {}", position, &dir);
            if is_end_name(position) {
                println!("Reached {position} after {count_iter} steps");
                count_iter = 0;
                reached += 1;
                if reached > 2 {
                    println!("Reached this one {} times, stopping", reached);
                    break;
                }
            } else if count_iter > 1_000_000 {
                println!("Stopped because too long");
                break;
            } 
            count_iter += 1;
            position = self.go(position, &dir).unwrap()
        }
        count_iter
    }
}

fn is_start_name(name: &str) -> bool {
    name.chars().last().unwrap() == 'A'
}
fn is_end_name(name: &str) -> bool {
    name.chars().last().unwrap() == 'Z'
}

// parsing
fn parse_line(line: &str) -> [&str; 3] {
    let name = &line[0..3];
    let left = &line[7..10];
    let right = &line[12..15];
    
    [name, left, right]
}
fn parse_instructions(line: &str) -> Vec<Direction> {
    line.chars()
        .map(|c| {
            match c {
                'L' => Left,
                'R' => Right,
                _ => panic!("Got {c} in instruction line {line}"),
            }
        }).collect()
}
fn parse_input(input: &str) -> (Vec<Direction>, Graph) {
    let mut graph = Graph::new();
    let input = fs::read_to_string(input).unwrap();

    let instructions = parse_instructions(input.lines().nth(0).unwrap());

    for line in input.lines().skip(2) {
        let names = parse_line(&line);
        match graph.add(names[0], names[1], names[2]) {
            Some(x) => panic!("Key {} was already mapping to node {x}", names[0]),
            None => {},
        }
    }
    (instructions, graph)
}
