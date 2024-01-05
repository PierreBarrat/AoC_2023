use std::fs;

fn main() {
    let input = "input.txt";
    let mut platform = Platform::from_file(&input);
    // println!("{:?}", platform);

    // platform.roll_east();
    for i in 0..1_000 {
        platform.cycle();
        println!("{}", platform.load());
    }
    // println!("{:?}", platform);
    // println!("{}", platform.load());
}


use Rock::*;
#[derive(Debug, Clone, Copy, PartialEq)]
enum Rock{
    Round,
    Cube,
    Empty,
}

#[derive(Debug, Clone)]
struct Platform {
    cols: Vec<Vec<Rock>>
}

impl Platform {
    fn roll_col_north(col: &mut Vec<Rock>) {
        let L = col.len();
        let mut first_empty = 0;
        for i in 0..L {
            let rock = col[i];
            if matches!(rock, Cube) {
                first_empty = i+1
            } else if matches!(rock, Round) {
                if i > first_empty {
                    col[first_empty] = Round;
                    col[i] = Empty;
                }
                first_empty += 1    
            }
        }
    }
    fn roll_col_south(col: &mut Vec<Rock>) {
        let L = col.len();
        let mut first_empty = L-1;
        for i in (0..L).rev() {
            let rock = col[i];
            if matches!(rock, Cube) && i > 0 {
                first_empty = i-1
            } else if matches!(rock, Round) {
                if i < first_empty {
                    col[first_empty] = Round;
                    col[i] = Empty;
                }
                if first_empty > 0 {first_empty -= 1}
            }
        }
    }

    fn roll_line_west(cols: &mut Vec<Vec<Rock>>, x: usize) {
        let L = cols.len();
        let mut first_empty = 0;
        for j in 0..L {
            let rock = cols[j][x];
            if matches!(rock, Cube) {
                first_empty = j+1
            } else if matches!(rock, Round) {
                if j > first_empty {
                    cols[first_empty][x] = Round;
                    cols[j][x] = Empty;
                }
                first_empty += 1    
            }
        }
    }
    fn roll_line_east(cols: &mut Vec<Vec<Rock>>, x: usize) {
        let L = cols.len();
        let mut first_empty = L-1;
        for j in (0..L).rev() {
            let rock = cols[j][x];
            if matches!(rock, Cube) && j > 0{
                first_empty = j-1
            } else if matches!(rock, Round) {
                if j < first_empty {
                    cols[first_empty][x] = Round;
                    cols[j][x] = Empty;
                }
                if j > 0 {first_empty -= 1}
            }
        }
    }

    fn roll_north(&mut self) {
        for col in self.cols.iter_mut() {
            // println!("Before north roll {:?}", col);
            Platform::roll_col_north(col)
            // println!("After north roll {:?}", col);
        }
    }
    fn roll_south(&mut self) {
        for col in self.cols.iter_mut() {
            // println!("Before S roll {:?}", col);
            Platform::roll_col_south(col);
            // println!("After S roll {:?}", col);
        }
    }
    fn roll_west(&mut self) {
        let nlines = self.size().0;
        for i in 0..nlines {
            // println!("Before north roll {:?}", col);
            Platform::roll_line_west(&mut self.cols, i)
            // println!("After north roll {:?}", col);
        }
    }
    fn roll_east(&mut self) {
        let nlines = self.size().0;
        for i in 0..nlines {
            // println!("Before north roll {:?}", col);
            Platform::roll_line_east(&mut self.cols, i)
            // println!("After north roll {:?}", col);
        }
    }

    fn cycle(&mut self) {
        self.roll_north();
        self.roll_west();
        self.roll_south();
        self.roll_east();
    }

    fn load(&self) -> u32 {
        let mut score: usize = 0;
        for col in self.cols.iter() {
            let L = col.len();
            score += col.iter().enumerate()
                .map(|(i,r)| {if matches!(r, Round) {L-i} else {0}})
                .sum::<usize>()
        }
        u32::try_from(score).unwrap()
    }
}







impl Platform {
    fn size(& self) -> (usize, usize) {
        let y = self.cols.len();
        if y == 0 {return (0,0)}
        (self.cols[0].len(), y)
    }

    fn from_file(input: &str) -> Self {
        let (x, y) = {
            let binding = fs::read_to_string(input).expect("Could not read file");
            let mut lines = binding.lines();
            let y = lines.nth(0).unwrap().len(); // number of columns
            let x = lines.count(); // number of lines
            (x, y)
        };

        let mut cols = (0..y).map(|_| Vec::<Rock>::with_capacity(x))
            .collect::<Vec<Vec<Rock>>>();

        let binding = fs::read_to_string(input).expect("Could not read file");
        let lines = binding.lines().enumerate();
        for (i, line) in lines {
            for (j, c) in line.chars().enumerate() {
                // println!("col {j} line {i} : {c}");
                cols[j].push(Rock::parse(c).unwrap())
            }
        }

        Platform{cols}
    }
}

impl Rock {
    fn parse(c: char) -> Option<Self> {
        match c {
            'O' => Some(Round),
            '#' => Some(Cube),
            '.' => Some(Empty),
            _ => None
        }
    }
}