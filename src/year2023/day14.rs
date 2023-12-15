use std::fmt;

pub fn solve(input: String) {
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

        fn tilted(&self) -> Platform {
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

            Platform { rocks }
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
}
