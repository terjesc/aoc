use core::cmp::max;
use std::str::FromStr;

use regex::Regex;

pub fn solve(input: String) {

    #[derive(Debug)]
    struct Game {
        id: u32,
        reveals: Vec<Reveal>,
    }

    #[derive(Debug, PartialEq, Eq)]
    struct ParseGameError;

    impl FromStr for Game {
        type Err = ParseGameError;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let re = Regex::new(r"^Game ([0-9]+):\s*(.+)$").unwrap();
            let caps = re.captures(s).unwrap();

            let id = caps[1].parse::<u32>().unwrap();

            let reveals: Vec<Reveal> = caps[2]
                    .split(";")
                    .map(|string| Reveal::from_str(&string).unwrap())
                    .collect();

            Ok(Game { id: id, reveals: reveals })
        }
    }

    #[derive(Debug)]
    struct Reveal {
        red: u32,
        green: u32,
        blue: u32,
    }

    #[derive(Debug, PartialEq, Eq)]
    struct ParseRevealError;

    impl FromStr for Reveal {
        type Err = ParseRevealError;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let re = Regex::new(r"\s*([0-9]+)\s*([a-z]+)").unwrap();

            let mut cubes = vec![];
            for (_, [count, color]) in re.captures_iter(s).map(|c| c.extract()) {
                cubes.push((color, count.parse::<u32>().unwrap()));
            }

            let mut reveal = Reveal { red: 0, green: 0, blue: 0 };

            for (color, count) in cubes {
                match color {
                    "red" => reveal.red = count,
                    "green" => reveal.green = count,
                    "blue" => reveal.blue = count,
                    _ => return Err(ParseRevealError),
                }
            }

            Ok(reveal)
        }
    }

    let games: Vec<Game> = input.lines().map(|line| Game::from_str(&line).unwrap()).collect();

    let maximum = Reveal {red: 12, green: 13, blue: 14};
    let part1: u32 = games.iter()
            .map(|game| {
                for reveal in &game.reveals {
                    if (reveal.red > maximum.red) | (reveal.green > maximum.green) | (reveal.blue > maximum.blue) {
                        return 0;
                    }
                }
                game.id
            })
            .sum();

    println!("Day 2 part 1: {}", part1);

    let part2: u32 = games.iter()
            .map(|game| {
                let minimum_cube_counts = game.reveals.iter()
                        .fold(
                            Reveal { red: 0, green: 0, blue: 0 },
                            |acc, e| {
                                Reveal {
                                    red: max(acc.red, e.red),
                                    green: max(acc.green, e.green),
                                    blue: max(acc.blue, e.blue),
                                }
                            }
                        );
                minimum_cube_counts.red * minimum_cube_counts.green * minimum_cube_counts.blue
            })
            .sum();

    println!("Day 2 part 2: {}", part2);
}
