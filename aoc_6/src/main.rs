use std::fs;
fn main() {
    let file = "input.txt";
    let race = parse_input(&file);

    println!("Big race: time {}, record {}", race.time, race.record);
    let (t1, t2) = race.min_max_press_time();
    println!("Min/Max: {t1}/{t2}");
    println!("Solution to part 1: {}", t2-t1+1);
    // part 1 
    // for race in races.iter() {
    //     let (t1, t2) = race.min_max_press_time();
    //     println!("Min/Max: {t1}/{t2}");
    //     println!("Number of winning strats {}", t2-t1+1);
    //     ans *= t2-t1+1
    // }
    // println!("Solution to part 1: {}", ans)
}

impl Race {
    fn min_max_press_time(&self) -> (u64, u64) {
        let delta = u64::pow(self.time, 2) - 4*self.record;

        let sdelta = f64::sqrt(delta as f64);
        let t1: u64 = {
            let t = ((self.time as f64) - sdelta)/2.;
            if t.ceil() == t.floor() {
                (t as u64) + 1
            } else {
                t.ceil() as u64
            }
        };
        let t2: u64 = {
            let t = ((self.time as f64) + sdelta)/2.;
            if t.ceil() == t.floor() {
                (t as u64) - 1
            } else {
                t.floor() as u64
            }
        };
        (t1, t2) 
    }
}

struct Race{
    time: u64,
    record: u64,
}

fn parse_input(file: &str) -> Race{
    let binding = fs::read_to_string(file)
        .unwrap();
    let input: Vec<&str> = binding.lines()
        .collect();

    if input.len() != 2 {panic!("Input file should have two lines")}

    let time: u64 = {
        let digit_line = input[0].split(':').nth(1).unwrap();
        digit_line.replace(" ", "")
            .parse()
            .unwrap()
    };   
    let record: u64 = {
        let digit_line = input[1].split(':').nth(1).unwrap();
        digit_line.replace(" ", "")
            .parse()
            .unwrap()
    };  

    Race{time, record}
}   