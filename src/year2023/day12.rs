use regex::Regex;

pub fn solve(input: String) {
    let re = Regex::new(r"^([.#?]+) ([0-9,]+)$").unwrap();

    let records: Vec<_> = input
        .lines()
        .map(|line| {
            let caps = re.captures(line).unwrap();
            let record: Vec<char> = caps[1].chars().collect();
            let partitions: Vec<usize> = caps[2]
                .split(',')
                .map(|s| s.parse::<usize>().unwrap())
                .collect();
            (record, partitions)
        })
        .collect();

    let part1 = count_possible_partitions(&records);

    println!("Day 12 part 1: {}", part1);

    let _records = unfold(&records);

    // TODO the current approach does not scale!
    //    let part2 = count_possible_partitions(&records);
    //    println!("Day 12 part 2: {}", part2);
}

// For an input or filled out string (actually vector or slice) of records,
// calculate the current partitions for that string
fn calculate_partitions(record: &[char]) -> Vec<usize> {
    let (last, mut partitions) = record.iter().fold((0usize, Vec::new()), |mut acc, &c| {
        if c == '#' {
            (acc.0 + 1, acc.1)
        } else if acc.0 == 0 {
            (0usize, acc.1)
        } else {
            acc.1.push(acc.0);
            (0usize, acc.1)
        }
    });

    if last != 0 {
        partitions.push(last);
    }

    partitions
}

// For an input or filled out string (actually vector or slice) of records,
// calculate all ways to choose n of the '?' characters to replace with '#'.
// Returns each such way as a vector of indexes.
fn calculate_combinations(elements: &[usize], n: usize) -> Vec<Vec<usize>> {
    fn combinations(
        // Accumulator collecting the full set of combinations
        acc: &mut Vec<Vec<usize>>,
        // One combination, in the process of being built
        combination: Vec<usize>,
        // The remainder of the array to choose from
        rest: &[usize],
        // The remaining number of elements to be chosen
        n: usize,
    ) {
        if n == 0 {
            acc.push(combination)
        } else {
            for i in 0..rest.len() {
                let mut combination = combination.clone();
                combination.push(rest[i]);
                combinations(acc, combination, &rest[i + 1..], n - 1);
            }
        }
    }

    let mut result = Vec::new();
    combinations(&mut result, Vec::new(), elements, n);

    result
}

fn count_possible_partitions(records: &Vec<(Vec<char>, Vec<usize>)>) -> usize {
    let mut count = 0;

    for (record, partition) in records {
        let known_broken_count = record.iter().filter(|&c| *c == '#').count();
        let total_broken_count: usize = partition.iter().sum();

        let unknown_broken_count = total_broken_count - known_broken_count;

        let unknown_indexes: Vec<usize> = record
            .iter()
            .enumerate()
            .filter(|&(_, c)| *c == '?')
            .map(|(index, _)| index)
            .collect();

        let candidate_partitions = calculate_combinations(&unknown_indexes, unknown_broken_count);

        let valid_partition_count = candidate_partitions
            .iter()
            .map(|candidate| {
                let mut record = record.clone();
                for &index in candidate {
                    record[index] = '#';
                }
                record
            })
            .filter(|record| calculate_partitions(record) == *partition)
            .count();

        count += valid_partition_count;
    }

    count
}

fn unfold(records: &[(Vec<char>, Vec<usize>)]) -> Vec<(Vec<char>, Vec<usize>)> {
    records
        .iter()
        .map(|(record, partitions)| {
            let mut record = record.clone();
            record.push('?');
            (
                record
                    .iter()
                    .cycle()
                    .take(record.len() * 5 - 1)
                    .copied()
                    .collect(),
                partitions
                    .iter()
                    .cycle()
                    .take(partitions.len() * 5)
                    .copied()
                    .collect(),
            )
        })
        .collect()
}
