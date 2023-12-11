use std::cmp::{max, min};
use std::collections::HashSet;

pub fn solve(input: String) {
    // Parse the input map into a set of galaxy coordinates
    let mut galaxies: HashSet<(usize, usize)> = HashSet::new();

    for (y, line) in input.lines().enumerate() {
        for (x, character) in line.chars().enumerate() {
            if character == '#' {
                galaxies.insert((x, y));
            }
        }
    }

    // Find the highest x and y coordinates used for any galaxy
    let dimensions: (usize, usize) = galaxies.iter().fold((0, 0), |dim, galaxy| {
        (max(dim.0, galaxy.0 + 1), max(dim.1, galaxy.1 + 1))
    });

    // Count the number of galaxies at each x and y coordinate
    let mut x_counts: Vec<usize> = vec![0; dimensions.0];
    let mut y_counts: Vec<usize> = vec![0; dimensions.1];

    for &(x, y) in &galaxies {
        x_counts[x] += 1;
        y_counts[y] += 1;
    }

    // From a vector of galaxy counts for each map coordinate along an axis,
    // calculate the real space coordinate for each map coordinate.
    // Space with no galaxies is expanded. Space with galaxies is not.
    // This can then be used as an index for converting from map coordinate to real space
    // coordinate.
    fn calculate_indexes(distribution: &[usize], expansion: usize) -> Vec<usize> {
        distribution
            .iter()
            .map(|&count| if count == 0 { expansion } else { 1 })
            .fold((Vec::<usize>::new(), 0usize), |mut acc, column_size| {
                acc.1 += column_size;
                acc.0.push(acc.1);
                acc
            })
            .0
    }

    // Calculate the manhattan distance between two coordinates
    fn manhattan(a: (usize, usize), b: (usize, usize)) -> usize {
        max(a.0, b.0) - min(a.0, b.0) + max(a.1, b.1) - min(a.1, b.1)
    }

    // Convert from map coordinate to real space coordinate
    fn actual_location(
        galaxy: (usize, usize),
        x_index: &[usize],
        y_index: &[usize],
    ) -> (usize, usize) {
        (x_index[galaxy.0], y_index[galaxy.1])
    }

    // Calculate indexes with space expansion of 2, for part1
    let x_distance_lookup = calculate_indexes(&x_counts, 2);
    let y_distance_lookup = calculate_indexes(&y_counts, 2);

    // Add the manhattan distances between all pairs of galaxies (both ways)
    let mut sum: usize = 0;
    for &galaxy_a in galaxies.iter() {
        for &galaxy_b in galaxies.iter() {
            sum += manhattan(
                actual_location(galaxy_a, &x_distance_lookup, &y_distance_lookup),
                actual_location(galaxy_b, &x_distance_lookup, &y_distance_lookup),
            );
        }
    }

    // Halve the distance sum, for getting only one way distances
    let part1 = sum / 2;

    println!("Day 11 part 1: {}", part1);

    // Calculate indexes with space expansion of one million, for part2
    let x_distance_lookup = calculate_indexes(&x_counts, 1_000_000);
    let y_distance_lookup = calculate_indexes(&y_counts, 1_000_000);

    // Add the manhattan distances between all pairs of galaxies (both ways)
    let mut sum: usize = 0;
    for &galaxy_a in galaxies.iter() {
        for &galaxy_b in galaxies.iter() {
            sum += manhattan(
                actual_location(galaxy_a, &x_distance_lookup, &y_distance_lookup),
                actual_location(galaxy_b, &x_distance_lookup, &y_distance_lookup),
            );
        }
    }

    // Halve the distance sum, for getting only one way distances
    let part2 = sum / 2;

    println!("Day 11 part 2: {}", part2);
}
