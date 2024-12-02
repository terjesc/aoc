pub fn solve(input: String) {
    fn is_safe(series: &[i32]) -> bool {
        #[derive(PartialEq)]
        enum Direction {
            Down,
            Up,
        }

        fn is_step_small_enough(first: &i32, last: &i32) -> bool {
            let diff = num::abs(first - last);
            (1..=3).contains(&diff)
        }

        fn direction_of_step(first: &i32, last: &i32) -> Direction {
            if first < last {
                Direction::Up
            } else {
                Direction::Down
            }
        }

        let direction = direction_of_step(&series[0], &series[1]);

        series.windows(2).all(|window| match window {
            [first, last] => {
                is_step_small_enough(first, last) && direction_of_step(first, last) == direction
            }
            _ => unreachable!(),
        })
    }

    let part1 = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|number| number.parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .filter(|series| is_safe(series))
        .count();

    println!("Day 2 part 1: {}", part1);

    fn is_safe_dampened(series: &[i32]) -> bool {
        fn dampened_permutations(series: &[i32]) -> Vec<Vec<i32>> {
            let mut permutations: Vec<Vec<i32>> = Vec::new();

            permutations.push(series.to_vec());

            for i in 0..series.len() {
                let mut permutation: Vec<i32> = Vec::new();

                for number in series.iter().take(i) {
                    permutation.push(*number);
                }
                for number in series.iter().skip(i + 1) {
                    permutation.push(*number);
                }

                assert!(permutation.len() == series.len() - 1);
                permutations.push(permutation);
            }

            permutations
        }

        dampened_permutations(series)
            .iter()
            .any(|series| is_safe(series))
    }

    let part2 = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|number| number.parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .filter(|series| is_safe_dampened(series))
        .count();

    println!("Day 2 part 2: {}", part2);
}
