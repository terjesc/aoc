use regex::Regex;

pub fn solve(input: String) {
    // Regex for matching mul(a, b) commands. #1 = a, #2 = b.
    let re = Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)").unwrap();

    // Add up the results from all properly formatted mul() commands
    let part1: usize = re
        .captures_iter(&input)
        .map(|capture| capture[1].parse::<usize>().unwrap() * capture[2].parse::<usize>().unwrap())
        .sum();

    println!("Day 3 part 1: {}", part1);

    // Find the parts of the input that should be included.
    // First up to the first "don't()".
    // Then repeatedly from the first following "do()",
    // to the next following "don't" or end of input.
    let mut include_intervals: Vec<(usize, usize)> = Vec::new();

    let first_dont_index = input.find("don't()").unwrap_or(input.len());
    include_intervals.push((0, first_dont_index));

    let mut index = first_dont_index;
    while let Some(start) = input
        .get(index..)
        .and_then(|s| s.find("do()").map(|i| i + index))
    {
        index = start;
        let end = input
            .get(index..)
            .and_then(|s| s.find("don't()").map(|i| i + index))
            .unwrap_or(input.len());
        index = end;

        include_intervals.push((start, end));
    }

    // Add up the results from only the mul() commands within the proper intervals
    let part2: usize = include_intervals
        .iter()
        .map(|&(start, end)| {
            re.captures_iter(&input[start..end])
                .map(|capture| {
                    capture[1].parse::<usize>().unwrap() * capture[2].parse::<usize>().unwrap()
                })
                .sum::<usize>()
        })
        .sum();

    println!("Day 3 part 2: {}", part2);
}
