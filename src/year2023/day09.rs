pub fn solve(input: String) {
    fn differences(number_series: &[i64]) -> Vec<i64> {
        number_series
            .windows(2)
            .map(|value_pair| value_pair[1] - value_pair[0])
            .collect()
    }

    // Predicts the numbers (before, after) the given number series.
    fn predict_both(number_series: &[i64]) -> (i64, i64) {
        if number_series.iter().all(|&n| n == 0) {
            (0, 0)
        } else {
            let (first, last) = predict_both(&differences(number_series));
            (
                number_series.first().unwrap() - first,
                number_series.last().unwrap() + last,
            )
        }
    }

    let (part2, part1) = input
        .lines()
        // Parse the input line (string) into a number series (vector of integers)
        .map(|line| {
            line.split_whitespace()
                .map(|s| s.parse::<i64>().unwrap())
                .collect::<Vec<i64>>()
        })
        // Calculate numbers (before, after) the original series
        .map(|number_series| predict_both(&number_series))
        // Sum the befores and afters
        .reduce(|(a1, a2), (b1, b2)| (a1 + b1, a2 + b2))
        .unwrap();

    // Sum of the first predicted number after each number series
    println!("Day 9 part 1: {}", part1);

    // Sum of the first predicted number before each number series
    println!("Day 9 part 2: {}", part2);
}
