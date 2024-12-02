use std::collections::HashMap;

pub fn solve(input: String) {

    let (mut left_list, mut right_list) : (Vec<i32>, Vec<i32>) = (Vec::new(), Vec::new());

    for line in input.lines() {
        let numbers: Vec<_> = line.split_whitespace().collect();
        left_list.push(numbers[0].parse().unwrap());
        right_list.push(numbers[1].parse().unwrap());
    }

    left_list.sort();
    right_list.sort();

    let part1: i32 = std::iter::zip(left_list.clone(), right_list.clone())
            .fold(0, |acc, (left, right)| acc + num::abs(left - right));

    println!("Day 1 part 1: {}", part1);

    let mut occurences: HashMap<i32, i32> = HashMap::new();

    for number in right_list {
        *occurences.entry(number).or_insert(0) += 1;
    }

    let part2: i32 = left_list.into_iter()
            .fold(0, |acc, number| acc + number * occurences.get(&number)
            .map(|number| *number)
            .unwrap_or(0));

    println!("Day 1 part 2: {}", part2);
}
