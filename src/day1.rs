pub fn solve(input: String) {
    fn part1_digit(line: String) -> u32 {
        let first = line.chars().find(|c| c.is_digit(10)).unwrap();
        let last = line.chars().rfind(|c| c.is_digit(10)).unwrap();
        first.to_digit(10).unwrap() * 10 + last.to_digit(10).unwrap()
    }

    let part1: u32 = input.lines().map(|line| part1_digit(line.to_string())).sum();

    println!("Day 1 part 1: {}", part1);

    fn match_digit_words(string: &str) -> Option<u32> {
        match string {
            "one" => Some(1),
            "two" => Some(2),
            "three" => Some(3),
            "four" => Some(4),
            "five" => Some(5),
            "six" => Some(6),
            "seven" => Some(7),
            "eight" => Some(8),
            "nine" => Some(9),
            _ => None,
        }
    }

    fn part2_digit(line: String) -> u32 {
        // Approach:
        // * For each position in the string, check if it is the start of a digit.
        //   Short-circuit once one is found. This is the first digit.
        // * For each position from end of string, check if it is the end of a digit.
        //   Short-circuit once one is found. This is the second digit.

        fn digit_starting_at(line: &String, position: usize) -> Option<u32> {
            let letters: Vec<char> = line.chars().collect();

            // Digits 0 through 9
            if letters[position].is_digit(10) {
                return letters[position].to_digit(10);
            }

            // Digits spelled out
            if position + 3 < line.len() {
                if let Some(number) = match_digit_words(&line[position..position+3]) {
                    return Some(number);
                }
            }

            if position + 4 < line.len() {
                if let Some(number) = match_digit_words(&line[position..position+4]) {
                    return Some(number);
                }
            }

            if position + 5 < line.len() {
                if let Some(number) = match_digit_words(&line[position..position+5]) {
                    return Some(number);
                }
            }

            None
        }

        fn digit_ending_at(line: &String, position: usize) -> Option<u32> {
            let letters: Vec<char> = line.chars().collect();

            // Digits 0 through 9
            if letters[position].is_digit(10) {
                return letters[position].to_digit(10);
            }

            // Digits spelled out
            if position >= 3 {
                if let Some(number) = match_digit_words(&line[position-2..=position]) {
                    return Some(number);
                }
            }

            if position >= 4 {
                if let Some(number) = match_digit_words(&line[position-3..=position]) {
                    return Some(number);
                }
            }

            if position >= 5 {
                if let Some(number) = match_digit_words(&line[position-4..=position]) {
                    return Some(number);
                }
            }

            None
        }

        let first = (0..line.len())
                .find_map(|index| digit_starting_at(&line, index))
                .unwrap();

        let last = (0..line.len()).rev()
                .find_map(|index| digit_ending_at(&line, index))
                .unwrap();

        first * 10 + last
    }

    let part2: u32 = input
            .lines()
            .map(|line| part2_digit(line.to_string()))
            .sum();

    println!("Day 1 part 2: {}", part2);
}

