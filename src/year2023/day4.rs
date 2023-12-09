use std::cmp::min;
use std::collections::HashSet;
use std::str::FromStr;

use regex::Regex;

pub fn solve(input: String) {

    #[derive(Debug)]
    struct Card {
        id: u32,
        winning_numbers: HashSet<u32>,
        numbers_you_have: HashSet<u32>,
    }

    #[derive(Debug, PartialEq, Eq)]
    struct ParseCardError;

    impl FromStr for Card {
        type Err = ParseCardError;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let re = Regex::new(r"^Card\s+([0-9]+):\s*(.+)\s*\|\s*(.+)$").unwrap();
            let caps = re.captures(s).unwrap();

            let id = caps[1].parse::<u32>().unwrap();

            let re_number = Regex::new(r"([0-9]+)").unwrap();

            let mut winning_numbers = HashSet::new();
            for (_, [value]) in re_number.captures_iter(&caps[2]).map(|c| c.extract()) {
                winning_numbers.insert(value.parse::<u32>().unwrap());
            }

            let mut numbers_you_have = HashSet::new();
            for (_, [value]) in re_number.captures_iter(&caps[3]).map(|c| c.extract()) {
                numbers_you_have.insert(value.parse::<u32>().unwrap());
            }

            Ok(Card { id: id, winning_numbers, numbers_you_have })
        }
    }

    let cards: Vec<Card> = input.lines().map(|line| Card::from_str(&line).unwrap()).collect();

    let part1: u32 = cards.iter()
            .map(|card| {
                let win_count = card.winning_numbers
                        .intersection(&card.numbers_you_have)
                        .count();

                if win_count > 0 {
                    2_u32.pow(win_count as u32 - 1)
                } else {
                    0
                }
            })
            .sum();

    println!("Day 4 part 1: {}", part1);

    let mut card_counts: Vec<usize> = vec![1; cards.len()];

    for card in cards.iter() {
        let win_count = card.winning_numbers.intersection(&card.numbers_you_have).count();
        if win_count > 0 {
            for i in card.id as usize .. min(card.id as usize + win_count, card_counts.len()) {
                card_counts[i] += card_counts[card.id as usize - 1];
            }
        }
    }

    let part2: usize = card_counts.iter().sum();

    println!("Day 4 part 2: {}", part2);
}
