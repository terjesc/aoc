use crate::utils::char_matrix;

use std::collections::HashSet;

pub fn solve(input: String) {
    fn turn_right(direction: (i32, i32)) -> (i32, i32) {
        (-direction.1, direction.0)
    }

    let mut lab = char_matrix(input);

    let (start_x, start_y) = lab
        .iter()
        .enumerate()
        .filter_map(|(y, row)| {
            row.iter()
                .position(|&c| c == '^')
                .map(|x| (x, y))
        })
        .next()
        .unwrap();

    let (mut x, mut y) = (start_x, start_y);
    println!("Start @ ({}, {})", x, y);

    let mut direction: (i32, i32) = (0, -1);

    loop {
        lab[y][x] = 'X';

        let next_x = x as i32 + direction.0;
        let next_y = y as i32 + direction.1;

        if next_x < 0 || next_x >= lab[0].len() as i32 || next_y < 0 || next_y >= lab.len() as i32 {
            break;
        }

        if lab[next_y as usize][next_x as usize] == '#' {
            direction = turn_right(direction);
        } else {
            x = next_x as usize;
            y = next_y as usize;
        }
    }

    let part1: usize = lab
        .iter()
        .map(|row| row.iter().filter(|&c| *c == 'X').count())
        .sum();

    println!("Day 6 part 1: {}", part1);

    // Approach for solving part 2:
    // * Put an obstacle (#) at every location in the path (X), exluding start position
    // * The longest path cannot be longer than width * height * 4, so let's brute force it and
    // simulate that many steps. If not outside by then, there must be at least one loop, right?
    // * Yes, storing the path (position + direction) and detect loops that way is probably the
    // best way to go, but what's the fun in that, eh?

    // Detect obstruction locations
    let obstruction_locations: HashSet<(usize, usize)> = lab
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter().enumerate().filter_map(move |(x, c)| {
                if *c == 'X' && (x, y) != (start_x, start_y) {
                    Some((x, y))
                } else {
                    None
                }
            })
        })
        .collect();

    println!(
        "There are {} simulations to run, so please allow some time for brute forcing!",
        obstruction_locations.len(),
    );

    // Run once for each possible obstruction location
    let part2: usize = obstruction_locations
        .iter()
        .filter_map(|&(x, y)| {
            let mut lab = lab.clone();
            lab[y][x] = '#';

            // run the simulation
            let (mut x, mut y) = (start_x, start_y);
            let mut direction: (i32, i32) = (0, -1);

            for _ in 0..4 * lab.len() * lab[0].len() {
                let next_x = x as i32 + direction.0;
                let next_y = y as i32 + direction.1;

                if next_x < 0
                    || next_x >= lab[0].len() as i32
                    || next_y < 0
                    || next_y >= lab.len() as i32
                {
                    // Guard exited; no loop today :(
                    return None;
                }

                if lab[next_y as usize][next_x as usize] == '#' {
                    direction = turn_right(direction);
                } else {
                    x = next_x as usize;
                    y = next_y as usize;
                }
            }

            // No escaping path can be this long, so guard must be looping
            Some(())
        })
        .count();

    println!("Day 6 part 2: {}", part2);
}
