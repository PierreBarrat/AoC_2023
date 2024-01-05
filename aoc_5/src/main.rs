use std::fs;

fn main() {
    let input = "input.txt";
    let input = fs::read_to_string(input).expect("Could not open file");
    let (seeds, food_maps) = parse_input(&input);
    // let min_location: usize = seeds.iter().map(|seed_range| {
    //     println!("Considering {} seeds...", seed_range.end - seed_range.start);
    //     (seed_range.start..seed_range.end).map(|seed|{
    //         let mut tmp = seed;
    //         for fm in food_maps.iter(){
    //             tmp = fm.propagate(tmp)
    //         }
    //         // println!("Seed {seed} goes to location {tmp}");
    //         tmp
    //     }).min().unwrap()
    // }).min()
    // .unwrap();


    // println!("The result for part 2 is {min_location}");
    let mut found = false;
    let mut location = 0;
    while !found{
        // println!("Considering location {location}");
        let mut seed = location;
        for fm in food_maps.iter().rev() {
            seed = fm.rev_propagate(seed)
        }
        // println!("--> {seed}");
        if is_in_seed_ranges(seed, &seeds) {
            println!("It's in a seed range!");
            found=true;
        }
        let mut tmp = seed;
        for fm in &food_maps {
            tmp = fm.propagate(tmp);
        }
        // println!("Reverse map: {tmp}");
        location += 1
    }
    println!("Result: {}", location-1);

    // let mut location = 35;
    // print!("{location} -> ");
    // for fm in food_maps.iter().rev() {
    //     location = fm.rev_propagate(location);
    //     print!("{location} ->");
    // }
    // println!();

    // let fr = &food_maps.iter().nth(5).unwrap().ranges[1];
    // let fm = &food_maps.iter().nth(5).unwrap();
    // println!("in_start: {}, out_start: {}, length: {}", fr.in_start, fr.out_start, fr.length);
    // println!("{}", fr.rev_propagate(35).unwrap());
    // println!("{}", fm.rev_propagate(35));
}

struct FoodRange {
    in_start: usize,
    out_start: usize,
    length: usize
}
struct FoodMap {
    ranges: Vec<FoodRange>
}
struct SeedRange {
    start: usize,
    end: usize,
}

fn is_in_seed_ranges(n: usize, ranges: &Vec<SeedRange>) -> bool {
    let flag = ranges.iter()
        .find(|r| (n >= r.start) && (n < r.end));
    match flag {
        Some(_) => true,
        None => false,
    }
}

impl FoodRange {
    fn propagate(&self, n: usize) -> Option<usize> {
        if (n >= self.in_start) && (n < self.in_start + self.length) {
            let m = self.out_start + (n - self.in_start);
            Some(m)
        } else {
            None
        }
    }
    fn rev_propagate(&self, n: usize) -> Option<usize> {
        if (n >= self.out_start) && (n < self.out_start + self.length) {
            Some(self.in_start + (n - self.out_start))
        } else {
            None
        }
    }
}
impl FoodMap {
    fn propagate(&self, n: usize) -> usize {
        for range in self.ranges.iter() {
            if let Some(m) = range.propagate(n) {return m}
        }
        return n
    }
    fn rev_propagate(&self, n: usize) -> usize {
        for range in self.ranges.iter() {
            // println!("")
            if let Some(m) = range.rev_propagate(n) {return m}
        }
        return n
    }
}

/* PARSING */
fn parse_input(input: &str) -> (Vec<SeedRange>, Vec<FoodMap>){
    let chunks = input.split("map:\n");

    let mut foodmaps: Vec<FoodMap> = Vec::new();
    let mut seeds: Vec<SeedRange> = Vec::new();

    for chunk in chunks {
        if chunk.starts_with("seeds:") {
            let line = chunk.lines().nth(0).unwrap();
            parse_seeds(&mut seeds, &line)
        } else {
            foodmaps.push(FoodMap::parse_from_chunk(&chunk))
        }
    }

    (seeds, foodmaps)
}

impl FoodMap {
    fn parse_from_chunk(chunk: &str) -> FoodMap {
        let ranges: Vec<FoodRange> = chunk.lines()
            .take_while(|l| !l.is_empty()) 
            .map(|l| FoodRange::new(l))
            .collect();
        FoodMap{ranges}
    }   
}

impl FoodRange{
    fn new(line: &str) -> FoodRange {
        let values: Vec<usize> = line.split_whitespace()
            .map(|l| l.parse::<usize>().unwrap_or_else(|d| {
                panic!("Could not parse {line} to three digits because of {d}")
            })).collect();
        if values.len() != 3 {
            panic!("Got {} numbers instead of 3 on line {line}", values.len());
        }

        FoodRange{
            in_start: values[1],
            out_start: values[0],
            length: values[2],
        }
    }
}


fn parse_seeds(seeds: &mut Vec<SeedRange>, line: &str) {
    let vals: Vec<usize> = line.trim()
        .split_whitespace()
        .skip(1) // skipping "Seeds: "
        .map(|x| x.parse::<usize>().unwrap())
        .collect();
    if vals.len() % 2 != 0 {panic!("Uneven number of seeds")}
    let L = vals.len()/2;
    for i in 0..L {
        let s = vals[2*i];
        let l = vals[2*i+1];
        let seed = SeedRange{
            start: s,
            end: s+l,
        };
        seeds.push(seed)
    }   
}