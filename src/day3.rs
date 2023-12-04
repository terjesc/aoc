pub fn solve(input: String) {

    // APPROACH
    // 1. shove everything into a 2D character array
    // 2. iterate through that in search of (series of) digits
    //      - calculate number
    //      - look at all neighbouring positions (within grid) to find non-dot, non-digit.
    // 3 Profit!?

    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let dimensions = (grid.len(), grid[0].len());

    #[derive(Debug)]
    struct PartNumber {
        position: (usize, usize),
        length: usize,
        value: u32,
    }

    // Locate all (horizontal) numbers in the whole grid
    let mut all_numbers: Vec<PartNumber> = Vec::new();
    for (y_index, row) in grid.iter().enumerate() {
        for (x_index, character) in row.iter().enumerate() {
            if character.is_digit(10) {

                // If previous character was digit then this number is already registered
                if x_index >= 1 && grid[y_index][x_index - 1].is_digit(10) {
                    continue;
                }

                // Find the full number
                let mut value = 0;
                let mut length = 0;
                for x in x_index..dimensions.1 {
                    let character = grid[y_index][x];
                    if character.is_digit(10) {
                        length = length + 1;
                        value = value * 10;
                        value = value + character.to_digit(10).unwrap();
                    } else {
                        break;
                    }
                }

                // Add number to list of numbers
                all_numbers.push(PartNumber { position: (x_index, y_index), length, value});
            }
        }
    }

    // Filter numbers based on the occurence of special symbols around them
    let valid_part_numbers: Vec<PartNumber> = all_numbers.into_iter()
            .filter(|n| {
                let validates_number = |x: usize, y: usize| -> bool {
                    let character = grid[y][x];
                    !character.is_digit(10) && character != '.'
                };

                let x_span = (
                    if n.position.0 > 0 { n.position.0 - 1 } else { 0 },
                    if n.position.0 + n.length < dimensions.0 {
                        n.position.0 + n.length
                    } else {
                        n.position.0 + n.length - 1
                    },
                );

                if n.position.1 > 0 {
                    for x in x_span.0..=x_span.1 {
                        if validates_number(x, n.position.1 - 1) {
                            return true;
                        }
                    }
                }

                if n.position.1 + 1 < dimensions.1 {
                    for x in x_span.0..=x_span.1 {
                        if validates_number(x, n.position.1 + 1) {
                            return true;
                        }
                    }
                }

                if x_span.0 < n.position.0 {
                    if validates_number(x_span.0, n.position.1) {
                        return true;
                    }
                }

                if x_span.1 >= n.position.0 + n.length {
                    if validates_number(x_span.1, n.position.1) {
                        return true;
                    }
                }

                false
            })
            .collect();

    let part1: u32 = valid_part_numbers.iter().map(|n| n.value).sum();

    println!("Day 3 part 1: {}", part1);

    // Approach part 2:
    // Identify all cog positions ('*' characters)
    // Find numbers around them
    // If more than one number: Multiply the numbers and add it to the sum.
}
