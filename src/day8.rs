use std::collections::HashMap;

use regex::Regex;

pub fn solve(input: String) {
    let mut input = input.lines();

    let directions = input.next().unwrap().chars();

    // Between the directions and the maps, there should be an empty line
    assert_eq!(input.next().unwrap(), "");

    let mut maps: HashMap<String, (String, String)> = HashMap::new();

    let re = Regex::new(r"^([A-Z]{3}) = \(([A-Z]{3}), ([A-Z]{3})\)$").unwrap();

    for line in input {
        let caps = re.captures(line).unwrap();
        maps.insert(caps[1].to_string(), (caps[2].to_string(), caps[3].to_string()));
    }

    let mut current: String = "AAA".to_string();
    let mut count: usize = 0;
    let mut directions1 = directions.clone().cycle();

    while current != "ZZZ" {
        if let Some((left, right)) = maps.get(&current) {
            current = match directions1.next() {
                Some('L') => left.clone(),
                Some('R') => right.clone(),
                _ => unreachable!(),
            };
            count += 1;
        } else {
            panic!("No match for current location of {}", current);
        }
    }

    let part1 = count;

    println!("Day 8 part 1: {}", part1);

    count = 0;
    let mut directions2 = directions.cycle();
    let mut current: Vec<String> = maps.clone().into_keys()
            .filter(|k| k.chars().nth(2) == Some('A'))
            .collect();

    println!("{:?}", current);

    fn are_all_ending(locations: &Vec<String>) -> bool {
        locations.into_iter().all(|location| location.chars().nth(2) == Some('Z'))
    }

    while !are_all_ending(&current) {
        count += 1;
        let direction = directions2.next().unwrap();

        current = current.iter()
                .map(|location|{
                    if let Some((left, right)) = maps.get(location) {
                        match direction {
                            'L' => left.clone(),
                            'R' => right.clone(),
                            _ => unreachable!(),
                        }
                    } else {
                        panic!("No match for location of {}", location);
                    }
                })
                .collect();
    }

    let part2 = count;

    println!("Day 8 part 2: {}", part2);
}
