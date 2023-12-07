use std::cmp::Ordering;
use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug, Eq, PartialEq)]
struct Hand {
    cards: Vec<char>,
}

#[derive(Debug, PartialEq, Eq)]
struct ParseHandError;

impl FromStr for Hand {
    type Err = ParseHandError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Hand {
            cards: s.chars().collect(),
        })
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        fn hand_cmp(this: &Vec<char>, other: &Vec<char>) -> Ordering {
            match this.iter().zip(other.iter()).filter(|(this, other)| this != other).next() {
                None => Ordering::Equal,
                Some((&this, &other)) => card_cmp(this, other),
            }
        }

        fn card_cmp(this: char, other: char) -> Ordering {
            fn rank(card: char) -> usize {
                match card {
                    'A' => 1,
                    'K' => 2,
                    'Q' => 3,
                    'J' => 4,
                    'T' => 5,
                    '9' => 6,
                    '8' => 7,
                    '7' => 8,
                    '6' => 9,
                    '5' => 10,
                    '4' => 11,
                    '3' => 12,
                    '2' => 13,
                    _ => unreachable!(),
                }
            }

            rank(this).cmp(&rank(other))
        }

        match self.r#type().cmp(&other.r#type()) {
            Ordering::Less => Ordering::Less,
            Ordering::Greater => Ordering::Greater,
            Ordering::Equal => {
                hand_cmp(&self.cards, &other.cards)
            }
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Eq, PartialEq)]
struct HandWithBid {
    hand: Hand,
    bid: i64,
}

#[derive(Debug, PartialEq, Eq)]
struct ParseHandWithBidError;

impl FromStr for HandWithBid {
    type Err = ParseHandWithBidError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((hand, bid)) = s.split_once(" ") {
            Ok(HandWithBid {
                hand: Hand::from_str(hand).unwrap(),
                bid: bid.parse::<i64>().unwrap(),
            })
        } else {
            Err(ParseHandWithBidError)
        }
    }
}

impl Ord for HandWithBid {
    fn cmp(&self, other: &Self) -> Ordering {
        self.hand.cmp(&other.hand)
    }
}

impl PartialOrd for HandWithBid {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

impl Hand {
    fn r#type(&self) -> HandType {
        // get stats
        let mut frequencies: Vec<usize> = self.cards.clone()
                .into_iter()
                .fold(HashMap::<char, usize>::new(), |mut m, x| {
                    *m.entry(x).or_default() += 1;
                    m
                })
                .into_iter()
                .map(|(_, v)| v)
                .collect();
        frequencies.sort();
        frequencies.reverse();

        match frequencies[..] {
            [5] => HandType::FiveOfAKind,
            [4, 1] => HandType::FourOfAKind,
            [3, 2] => HandType::FullHouse,
            [3, 1, 1] => HandType::ThreeOfAKind,
            [2, 2, 1] => HandType::TwoPair,
            [2, 1, 1, 1] => HandType::OnePair,
            [1, 1, 1, 1, 1] => HandType::HighCard,
            _ => unreachable!(),
        }
    }
}

pub fn solve(input: String) {
    let mut hands: Vec<HandWithBid> = input.lines()
            .map(|line| HandWithBid::from_str(line).unwrap())
            .collect();

    hands.sort();

    let part1: i64 = hands.iter().rev()
            .enumerate()
            .map(|(rank, hand)| {
                (rank + 1) as i64 * hand.bid
            })
            .sum();

    println!("Day 7 part 1: {}", part1);
}
