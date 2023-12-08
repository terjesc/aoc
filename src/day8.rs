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

    #[derive(Clone, Debug)]
    struct Cycle {
        start: Option<usize>,
        length: Option<usize>,
    }

    let mut cycles: Vec<Cycle> = vec![Cycle { start: None, length: None }; current.len()];

    fn are_all_known(cycles: &Vec<Cycle>) -> bool {
        cycles.into_iter().all(|cycle| cycle.start.is_some() && cycle.length.is_some())
    }

    while !are_all_known(&cycles) {
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

        for (index, location) in current.iter().enumerate() {
            if location.chars().nth(2) == Some('Z') {
                if cycles[index].start.is_none() {
                    cycles[index].start = Some(count);
                } else if  cycles[index].length.is_none() {
                    cycles[index].length = Some(count - cycles[index].start.unwrap());
                }
            }
        }
    }

    // Observation: The data set is such that the path to the start of the cycle equals the length
    // of the cycle.
    for cycle in &cycles {
        assert_eq!(cycle.start, cycle.length);
    }
    // We can therefore use least common multiple (LCM) for calculating when we first reach
    // locations all ending in Z. (I.e. the point where all cycles align.)
    let part2 = cycles.iter()
        .map(|cycle| cycle.start.unwrap())
        .reduce(|acc, n| num::integer::lcm(acc, n))
        .unwrap();

    println!("Day 8 part 2: {}", part2);
}
