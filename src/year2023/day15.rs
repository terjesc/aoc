use regex::Regex;

fn hash(string: &str) -> usize {
    string.chars().fold(0usize, |hash_acc, character| {
        ((hash_acc + character as usize) * 17) % 256
    })
}

#[derive(Clone)]
struct LenseBox {
    lenses: Vec<Lense>,
}

impl LenseBox {
    fn new() -> Self {
        Self { lenses: Vec::new() }
    }

    fn remove(&mut self, lense_label: &str) {
        self.lenses.retain(|lense| lense.label != lense_label);
    }

    fn replace(&mut self, new_lense: Lense) {
        for existing_lense in &mut self.lenses {
            if existing_lense.label == new_lense.label {
                existing_lense.focal_length = new_lense.focal_length;
                return;
            }
        }

        self.lenses.push(new_lense);
    }
}

#[derive(Clone)]
struct Lense {
    label: String,
    focal_length: usize,
}

pub fn solve(input: String) {
    let part1: usize = input.trim().split(',').map(hash).sum();

    println!("Day 15 part 1: {}", part1);

    let instructions: Vec<String> = input.trim().split(',').map(str::to_string).collect();

    let re = Regex::new(r"^([a-z]+)(=|-)([0-9]?)$").unwrap();

    let mut boxes: Vec<LenseBox> = vec![LenseBox::new(); 256];

    for instruction in instructions {
        let caps = re.captures(&instruction).unwrap();

        let label = &caps[1];
        let operator = &caps[2];
        let focal_length = &caps[3].parse::<usize>();

        let lense_box_index = hash(label);

        match operator {
            "-" => boxes[lense_box_index].remove(label),
            "=" => {
                let lense = Lense {
                    label: label.to_string(),
                    focal_length: focal_length.clone().unwrap(),
                };
                boxes[lense_box_index].replace(lense);
            }
            _ => unreachable!(),
        }
    }

    let part2: usize = boxes
        .iter()
        .enumerate()
        .map(|(box_index, lense_box)| {
            lense_box
                .lenses
                .iter()
                .enumerate()
                .map(|(lense_index, lense)| {
                    (box_index + 1) * (lense_index + 1) * lense.focal_length
                })
                .sum::<usize>()
        })
        .sum();

    println!("Day 15 part 2: {}", part2);
}
