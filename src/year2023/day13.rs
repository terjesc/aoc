pub fn solve(input: String) {
    #[derive(Debug)]
    struct Pattern {
        pattern: Vec<Vec<char>>,
    }

    impl Pattern {
        fn rows(&self) -> Vec<Vec<char>> {
            self.pattern.clone()
        }

        fn cols(&self) -> Vec<Vec<char>> {
            let y_len = self.pattern.len();
            let x_len = self.pattern[0].len();

            let mut cols: Vec<Vec<char>> = vec![vec!['.'; y_len]; x_len];

            for y in 0..y_len {
                #[allow(clippy::needless_range_loop)]
                for x in 0..x_len {
                    cols[x][y] = self.pattern[y][x];
                }
            }

            cols
        }
    }

    let patterns = input
        .split("\n\n")
        .map(|raw_pattern| {
            let pattern = raw_pattern
                .lines()
                .map(|line| line.chars().collect::<Vec<char>>())
                .collect::<Vec<_>>();
            Pattern { pattern }
        })
        .collect::<Vec<_>>();

    let part1: usize = patterns
        .iter()
        .map(|pattern| {
            if let Some(reflection_index) = calculate_reflection(pattern.rows()) {
                reflection_index * 100
            } else if let Some(reflection_index) = calculate_reflection(pattern.cols()) {
                reflection_index
            } else {
                unreachable!()
            }
        })
        .sum();

    println!("Day 13 part 1: {}", part1);

    fn calculate_reflection(pattern: Vec<Vec<char>>) -> Option<usize> {
        let candidates: Vec<usize> = pattern
            .windows(2)
            .enumerate()
            .filter(|pair| pair.1[0] == pair.1[1])
            .map(|(index, _)| index)
            .collect();

        for candidate in candidates {
            // Check if it reflects
            let mut all_good = true;

            for i in 0..candidate {
                let reflected_i = candidate + 1 + candidate - i;
                if reflected_i >= pattern.len() || pattern[i] == pattern[reflected_i] {
                    continue;
                } else {
                    all_good = false;
                    break;
                }
            }

            if all_good {
                return Some(candidate + 1);
            }
        }

        None
    }

    // Part 2:
    // In order to count a reflection, do the same as for part 1, but
    // instead of checking patterns for equality, allow for one (and only one) difference
    // between the compared lines. Count as reflection if one (and only one) comparison
    // leads to a difference of one (including the candidate reflection line).

    fn difference(a: &[char], b: &[char]) -> usize {
        assert_eq!(a.len(), b.len());

        a.len() - a.iter().zip(b.iter()).filter(|(a, b)| a == b).count()
    }

    fn calculate_reflection_with_smudge(pattern: Vec<Vec<char>>) -> Option<usize> {
        let candidates: Vec<usize> = pattern
            .windows(2)
            .enumerate()
            .filter(|pair| difference(&pair.1[0], &pair.1[1]) <= 1)
            .map(|(index, _)| index)
            .collect();

        for candidate in candidates {
            // Check if it reflects
            let mut total_difference = 0;

            for i in 0..=candidate {
                let reflected_i = candidate + 1 + candidate - i;
                if reflected_i >= pattern.len() {
                    continue;
                }
                total_difference += difference(&pattern[i], &pattern[reflected_i]);
            }

            if total_difference == 1 {
                return Some(candidate + 1);
            }
        }

        None
    }

    let part2: usize = patterns
        .iter()
        .map(|pattern| {
            if let Some(index) = calculate_reflection_with_smudge(pattern.rows()) {
                index * 100
            } else if let Some(index) = calculate_reflection_with_smudge(pattern.cols()) {
                index
            } else {
                unreachable!()
            }
        })
        .sum();

    println!("Day 13 part 2: {}", part2);

    // my results:
    // Part 1 = 32723
    // Part 2 = 34536
}
