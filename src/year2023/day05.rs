use std::cmp::{max, min};

pub fn solve(input: String) {
    #[derive(Debug)]
    struct ConversionRange {
        start: i64,
        end: i64,
        offset: i64,
    }

    #[derive(Debug)]
    struct ConversionMap {
        map: Vec<ConversionRange>,
    }

    let mut input = input.lines();

    // Read seeds
    let (_label, seeds) = input.next().unwrap().split_once(':').unwrap();
    let seeds: Vec<i64> = seeds
        .split_whitespace()
        .map(|number| number.parse::<i64>().unwrap())
        .collect();

    //    println!("Seeds: {:?}", seeds);

    assert_eq!(input.next().unwrap(), "");

    // Read one conversion map into a ConversionMap structure.
    // Assumes we are about to read the first row of (destination start, source start, range
    // length), i.e. that the line heading before the map numbers are already read.
    fn read_conversion_map(input: &mut std::str::Lines) -> ConversionMap {
        let mut ranges: Vec<ConversionRange> = Vec::new();

        for line in input.by_ref() {
            if line.is_empty() {
                break;
            }
            let mapping: Vec<i64> = line
                .split_whitespace()
                .map(|number| number.parse::<i64>().unwrap())
                .collect();
            let (destination_start, source_start, range_length) =
                (mapping[0], mapping[1], mapping[2]);
            ranges.push(ConversionRange {
                start: source_start,
                end: source_start + range_length - 1,
                offset: destination_start - source_start,
            });
        }

        ConversionMap { map: ranges }
    }

    // Read all the conversion maps into one array of conversion maps.
    let mut conversion_maps: Vec<ConversionMap> = Vec::new();

    while let Some(_line) = input.next() {
        //        println!("{:?}", _line);
        let conversion_map = read_conversion_map(&mut input);
        //        println!("{:?}", conversion_map);
        conversion_maps.push(conversion_map);
    }

    // Convert each seed into a location value, then return the minimum.
    let part1 = seeds
        .iter()
        .map(|seed| {
            conversion_maps.iter().fold(*seed, |acc, map| {
                for range in map.map.iter() {
                    if acc >= range.start && acc <= range.end {
                        //                            println!("{}", acc + range.offset);
                        return acc + range.offset;
                    }
                }
                //                    println!("{}", acc);
                acc
            })
        })
        .min()
        .unwrap();

    println!("Day 5 part 1: {}", part1);

    let seed_ranges: Vec<(i64, i64)> = seeds
        .chunks(2)
        .map(|numbers| (numbers[0], numbers[0] + numbers[1] - 1))
        .collect();

    //    println!("{:?}", seed_ranges);

    // Helper function: Extract overlap (intersection)
    fn intersection(a: &(i64, i64), b: &(i64, i64)) -> Option<(i64, i64)> {
        let &(a_min, a_max) = a;
        let &(b_min, b_max) = b;

        if a_max < b_min || b_max < a_min {
            None
        } else {
            Some((max(a_min, b_min), min(a_max, b_max)))
        }
    }

    // Options:
    // A: AAAAAAAAAA      AAA         AAAAAA       AAAAA
    // B:   BBBB       BBBBBBBBBB        BBBBBB   BBB
    // R: AA    AAAA                  AAA            AAA
    fn remainder(a: &(i64, i64), b: &(i64, i64)) -> Option<Vec<(i64, i64)>> {
        let &(a_min, a_max) = a;
        let &(b_min, b_max) = b;

        if a_max < b_min || b_max < a_min {
            Some(vec![*a])
        } else if a_min >= b_min && a_max <= b_max {
            None
        } else if b_min > a_min && b_max < a_max {
            Some(vec![(a_min, b_min - 1), (b_max + 1, a_max)])
        } else if b_min > a_min {
            Some(vec![(a_min, b_min - 1)])
        } else if b_max < a_max {
            Some(vec![(b_max + 1, a_max)])
        } else {
            unreachable!()
        }
    }

    let location_ranges = conversion_maps.iter().fold(seed_ranges, |acc, map| {
        let mut next_acc: Vec<(i64, i64)> = Vec::new();
        let mut remaining_acc: Vec<(i64, i64)> = Vec::new();

        // Handle all input ranges
        for input_range in acc {
            let mut remaining_range: Vec<(i64, i64)> = vec![input_range];

            // Iterate through conversion map ranges
            for range in map.map.iter() {
                if let Some(overlap) = intersection(&input_range, &(range.start, range.end)) {
                    //                            println!("overlap: {:?}", overlap);
                    // Remove overlap from remaining_range
                    let remaining_range_acc: Vec<(i64, i64)> = remaining_range
                        .iter()
                        .flat_map(|range| remainder(range, &overlap))
                        .flatten()
                        .collect();
                    remaining_range = remaining_range_acc;

                    next_acc.push((overlap.0 + range.offset, overlap.1 + range.offset));
                }
            }
            remaining_acc.append(&mut remaining_range);
        }
        //                println!("");

        next_acc.append(&mut remaining_acc);
        //                println!("List of ranges after conversion completed: {:?}", next_acc);
        next_acc
    });

    //    println!("{:?}", location_ranges);

    let part2 = location_ranges
        .iter()
        .map(|(start, _end)| start)
        .min()
        .unwrap();

    println!("Day 5 part 2: {}", part2);
}
