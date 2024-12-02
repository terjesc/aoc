pub fn solve(input: String) {
    fn is_safe(series: &Vec<i32>) -> bool {
        #[derive(PartialEq)]
        enum Direction {
            DOWN,
            UP,
        }

        fn is_step_small_enough(first: &i32, last: &i32) -> bool {
            let diff = num::abs(first - last);
            diff >= 1 && diff <= 3
        }

        fn direction_of_step(first: &i32, last: &i32) -> Direction {
            if first < last {
                Direction::UP
            } else {
                Direction::DOWN
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
                .collect()
        })
        .filter(|series| is_safe(series))
        .count();

    println!("Day 2 part 1: {}", part1);

    fn is_safe_dampened(series: &Vec<i32>) -> bool {
        fn dampened_permutations(series: &Vec<i32>) -> Vec<Vec<i32>> {
            let mut permutations: Vec<Vec<i32>> = Vec::new();

            permutations.push(series.to_vec());

            for i in 0..series.len() {
                let mut permutation: Vec<i32> = Vec::new();
                for j in 0..i {
                    permutation.push(series[j]);
                }
                for j in i + 1..series.len() {
                    permutation.push(series[j]);
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
                .collect()
        })
        .filter(|series| is_safe_dampened(series))
        .count();

    println!("Day 2 part 2: {}", part2);
}
