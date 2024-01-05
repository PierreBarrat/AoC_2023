use std::fs;
use std::fmt;
use std::cmp;

fn main() {
    let input_file = "input.txt";
    let mut universe = parse_universe(&input_file);

    println!("Row size: {:?}", universe.row_size);
    println!("Col size: {:?}", universe.col_size);

    let mut S = 0;
    for (i, g1) in universe.galaxies.iter().enumerate() {
        for (j, g2) in universe.galaxies.iter().skip(i+1).enumerate() {
            let (dx, dy) = universe.distance(&g1, &g2);
            println!(
                "Galaxies {i} ({}, {}) and {} ({}, {}): dx={dx}, dy={dy}, distance={}",
                g1.x, g1.y, i+j+1, g2.x, g2.y, dx+dy
            );
            S += dx + dy;
        }
    }
    println!("Result: {S}");
}


struct Universe {
    X: usize,
    Y: usize,
    galaxies: Vec<Galaxy>,
    row_size: Vec<u64>,
    col_size: Vec<u64>, 
}

struct Galaxy {
    x: usize,
    y: usize,
}

impl Universe {
    fn distance(&self, g1: &Galaxy, g2: &Galaxy) -> (u64, u64) {
        let (x1, x2) = (cmp::min(g1.x, g2.x), cmp::max(g1.x, g2.x));
        let dx: u64 = self.row_size[x1..x2].iter().sum();

        let (y1, y2) = (cmp::min(g1.y, g2.y), cmp::max(g1.y, g2.y));
        let dy: u64 = self.col_size[y1..y2].iter().sum();
        (dx, dy)
    }
}

fn parse_universe(file: &str) -> Universe {
    let input = fs::read_to_string(file).unwrap();
    let X = input.lines().count();
    let Y = input.lines().nth(0).unwrap().chars().count();

    let mut galaxies: Vec<Galaxy> = Vec::new();
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                galaxies.push(Galaxy{x, y});
                println!("Found galaxy at ({x}, {y})");
            }
        }
    }

    let mut row_size: Vec<u64> = (0..X).map(|_| 1000000).collect();
    let mut col_size: Vec<u64> = (0..Y).map(|_| 1000000).collect();
    for g in galaxies.iter() {
        row_size[g.x] = 1;
        col_size[g.y] = 1;
    }

    Universe{
        X, Y, galaxies, col_size, row_size,
    }
}

impl fmt::Display for Galaxy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
impl fmt::Display for Universe {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.X, self.Y)
    }
}