use std::collections::VecDeque;
use std::fmt;

pub fn solve(input: String) {
    #[derive(Clone, PartialEq)]
    struct Platform {
        rocks: Vec<Vec<char>>,
    }

    impl Platform {
        fn north_support_load(&self) -> usize {
            self.rocks
                .iter()
                .rev()
                .enumerate()
                .map(|(i, row)| (i + 1) * row.iter().filter(|&c| *c == 'O').count())
                .sum()
        }

        fn tilted(&self) -> Self {
            let mut rocks: Vec<Vec<char>> = self.rocks.clone();
            let mut vacant_indexes: Vec<Option<usize>> = self.rocks[0]
                .iter()
                .map(|c| match c {
                    '.' => Some(0),
                    _ => None,
                })
                .collect();

            for (y, row) in self.rocks.iter().enumerate() {
                for (x, character) in row.iter().enumerate() {
                    match *character {
                        'O' => {
                            if let Some(new_y) = vacant_indexes[x] {
                                rocks[new_y][x] = 'O';
                                rocks[y][x] = '.';
                                vacant_indexes[x] = Some(new_y + 1);
                            }
                        }
                        '.' => {
                            if vacant_indexes[x].is_none() {
                                vacant_indexes[x] = Some(y);
                            }
                        }
                        _ => vacant_indexes[x] = None,
                    }
                }
            }

            Self { rocks }
        }

        fn tilted_north(&self) -> Self {
            self.tilted()
        }

        fn tilted_west(&self) -> Self {
            self.transposed().tilted_north().transposed()
        }

        fn tilted_south(&self) -> Self {
            self.flipped().tilted_north().flipped()
        }

        fn tilted_east(&self) -> Self {
            self.transposed()
                .flipped()
                .tilted_north()
                .flipped()
                .transposed()
        }

        fn transposed(&self) -> Self {
            let x_len = self.rocks[0].len();
            let y_len = self.rocks.len();

            let mut rocks: Vec<Vec<char>> = vec![Vec::new(); x_len];

            #[allow(clippy::needless_range_loop)]
            for x in 0..x_len {
                for y in 0..y_len {
                    rocks[x].push(self.rocks[y][x]);
                }
            }
            Self { rocks }
        }

        fn flipped(&self) -> Self {
            let rocks: Vec<Vec<char>> = self.rocks.clone().into_iter().rev().collect();

            Self { rocks }
        }

        fn cycled(&self) -> Self {
            self.tilted_north()
                .tilted_west()
                .tilted_south()
                .tilted_east()
        }

        // Put this in the chain of calls returning Self,
        // for printing the state at intermediate steps.
        fn _print(&self) -> &Self {
            println!("{:?}", self);
            self
        }
    }

    impl fmt::Debug for Platform {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            for row in &self.rocks {
                writeln!(f, "{}", row.iter().collect::<String>())?
            }

            write!(f, "")
        }
    }

    let rocks: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let platform = Platform { rocks };
    let tilted_platform = platform.tilted();

    let part1 = tilted_platform.north_support_load();

    println!("Day 14 part 1: {}", part1);

    let mut platform = platform;

    let mut cache = VecDeque::new();
    cache.push_back(platform.clone());

    let mut load_at_north_support = 0usize;

    'outer: for iteration in 0..1000 {
        platform = platform.cycled();

        if cache.contains(&platform) {
            for (index, cached_platform) in cache.iter().rev().enumerate() {
                if *cached_platform == platform {
                    let cycle_length = index + 1;
                    let remaining_to_1_000_000_000 = 1_000_000_000 - iteration - 1;

                    if remaining_to_1_000_000_000 % cycle_length == 0 {
                        load_at_north_support = platform.north_support_load();
                        break 'outer;
                    }
                }
            }
        }

        cache.push_back(platform.clone());

        if cache.len() > 50 {
            let _ = cache.pop_front();
        }
    }

    let part2 = load_at_north_support;

    println!("Day 14 part 2: {}", part2);
}
