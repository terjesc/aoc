use std::cmp::min;
use std::collections::HashMap;

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

    let records = unfold(&records);

    // The part 1 solution doesn't work for part 2, because it does not scale well enough
    let part2 = count_possible_partitions_improved(&records);
    println!("Day 12 part 2: {}", part2);
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

fn count_possible_partitions_improved(records: &[(Vec<char>, Vec<usize>)]) -> usize {
    records
        .iter()
        .map(|(record, partition)| calculate_combination_count(record, partition))
        .sum()
}

fn calculate_combination_count(record: &[char], partition: &[usize]) -> usize {
    fn combinations(
        // Cache
        cache: &mut HashMap<(Vec<char>, Vec<usize>), usize>,
        // The remaining record to match
        record: &[char],
        // The remaining partitions to match
        partitions: &[usize],
    ) -> usize {
        // Check cache
        if let Some(&value) = cache.get(&(record.to_vec(), partitions.to_vec())) {
            return value;
        }

        // Helper function for caching results before returning them
        let cache_result =
            |result: usize, cache: &mut HashMap<(Vec<char>, Vec<usize>), usize>| -> usize {
                cache.insert((record.to_vec(), partitions.to_vec()), result);
                result
            };

        // If there are no partitions left, we either have one match, or we have none:
        if partitions.is_empty() {
            if record.iter().filter(|&c| *c == '#').count() == 0 {
                // No more '#' means we do have a valid combination.
                return cache_result(1, cache);
            } else {
                // Remaining '#' but no partitions means the combination is invalid.
                return cache_result(0, cache);
            }
        }

        // The first partition must start at the earliest at the first '?' or '#'
        let earliest_start = record.iter().position(|&c| c == '?' || c == '#');
        if earliest_start.is_none() {
            // There are no '?' or '#' left for the remaining partitions
            return cache_result(0, cache);
        }
        let earliest_start = earliest_start.unwrap();

        // Find the end of the group of '?' and '#'
        let latest_end = match record.iter().skip(earliest_start).position(|&c| c == '.') {
            Some(index) => earliest_start + index,
            None => record.len(),
        };

        let earliest_segment = &record[earliest_start..latest_end];

        // If the earliest segment cannot hold the first partition…
        if earliest_segment.len() < partitions[0] {
            if earliest_segment.iter().any(|&c| c == '#') {
                // …and there is at least one '#' involved, then this branch has no solutions,
                // since that '#' cannot be part of the first partition.
                return cache_result(0, cache);
            } else {
                // …and there are only '?' involved, then there may be solutions further on,
                // and all those '?' are '.'.
                return cache_result(
                    combinations(cache, &record[latest_end..], partitions),
                    cache,
                );
            }
        }

        // If there is at least one '#' involved, then the partition can start at the latest
        // on that position. In any case the partition must fit within the segment, or, in the case
        // of all '?', can be left for the next segment instead.
        let latest_start = min(
            earliest_segment
                .iter()
                .position(|&c| c == '#')
                .unwrap_or(earliest_segment.len() - partitions[0]),
            earliest_segment.len() - partitions[0],
        ) + earliest_start;

        let mut count = 0;

        for start in earliest_start..=latest_start {
            let one_after = start + partitions[0];
            if one_after >= record.len() {
                if partitions.len() == 1 {
                    count += 1;
                }
            } else if record[one_after] != '#' {
                let next_start = one_after + 1;
                count += combinations(cache, &record[next_start..], &partitions[1..]);
            }
        }

        // Special case if all '?'
        if earliest_segment.iter().all(|&c| c == '?') {
            let next_start = latest_end + 1;
            if next_start < record.len() {
                count += combinations(cache, &record[next_start..], partitions);
            }
        }

        cache_result(count, cache)
    }

    let mut cache = HashMap::new();
    combinations(&mut cache, record, partition)
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
