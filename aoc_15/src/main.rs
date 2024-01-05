use std::fs;

fn main() {
    let input = "input.txt";
    let test_string = "rn";
    println!("{} --> {}", test_string, hash(&test_string));
    println!("{}", solve_part_1(input));

    solve_part_2(&input);
}

fn solve_part_2(input: &str) {
    let input = fs::read_to_string(input).unwrap();
    let mut boxes: Vec<LensBox> = (0..256).map(|_| LensBox{lenses: Vec::<Lens>::new()})
        .collect();


    for snip in input.split(',') {
        let instruction = parse_instruction(&snip);
        // println!("{} -> {:?}", snip, instruction);
        execute(&mut boxes, instruction);

        // for (i, lens_box) in boxes.iter().enumerate() {
        //     if !lens_box.lenses.is_empty() {
        //         println!("{i} - {:?}", lens_box)
        //     }
        // }
    }

    println!("Focusing power: {}", focusing_power(& boxes))
}

fn focusing_power(boxes: &Vec<LensBox>) -> usize {
    let mut pow = 0;
    for (box_id, lens_box) in boxes.iter().enumerate() {
        for (lens_slot, lens) in lens_box.lenses.iter().enumerate() {
            pow += (box_id + 1) * (lens_slot + 1) * (lens.focal as usize)
        }
    }

    pow
}

#[derive(Debug)]
struct LensBox {
    lenses: Vec<Lens>
}

#[derive(Debug)]
struct Lens {
    label: String,
    focal: u8,
}

#[derive(Debug)]
enum Operation {
    Put(usize, Lens),
    Retrieve(usize, Lens),
}

impl LensBox {
    fn position_lens_in_box(&self, lens: &Lens) -> Option<usize>{
        self.lenses.iter().position(|x| x.label == lens.label)
    }
}

fn execute(boxes: &mut Vec<LensBox>, op: Operation) {
    match op {
        Operation::Put(box_id, lens) => {
            let mut this_box = &mut boxes[box_id];
            if let Some(idx) = this_box.position_lens_in_box(&lens) {
                this_box.lenses[idx] = lens;
            } else {
                this_box.lenses.push(lens);
            }
            // println!("{:?}", this_box);
        },
        Operation::Retrieve(box_id, lens) => {
            let mut this_box = &mut boxes[box_id];
            if let Some(idx) = this_box.position_lens_in_box(&lens) {
                this_box.lenses.remove(idx);
            }
        }
    }
}

fn parse_instruction(snip: &str) -> Operation {
    if let Some('-') = snip.chars().last() {
        let L = snip.len();
        let label = &snip.to_string()[0..L-1];
        let box_id = hash(&label);
        let lens = Lens{label: label.to_string(), focal: 0};
        Operation::Retrieve(box_id, lens)
    } else {
        let mut parts = snip.split('=');
        let label = parts.nth(0).unwrap().to_string();
        let focal = parts.nth(0).unwrap().parse::<u8>().unwrap();
        let box_id = hash(&label);
        Operation::Put(box_id, Lens{label: label, focal})
    }
    // Operation::Put(1, Lens{label: "1".to_string(), focal:2})
}

fn hash(s: &str) -> usize {
    let mut val: u8 = 0;
    for c in s.chars() {
        val = val.wrapping_add(c as u8);
        val = val.wrapping_mul(17);
    }
    // println!("{} --> {}", s, val);
    val as usize
}


fn solve_part_1(input: &str) -> usize {
    let tmp = fs::read_to_string(input).unwrap();
    let sol = tmp.split(',')
        .map(|s| hash(s))
        .sum::<usize>();
    sol
}