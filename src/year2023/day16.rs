use std::collections::{HashMap, VecDeque};

struct BeamFront {
    location: (usize, usize),
    direction: Direction,
}

#[derive(Clone, Copy, PartialEq)]
enum Direction {
    North,
    West,
    South,
    East,
}

impl Direction {
    fn one_step(&self, location: (usize, usize), bounds: (usize, usize)) -> Option<(usize, usize)> {
        match self {
            Direction::North => {
                if location.1 > 0 {
                    Some((location.0, location.1 - 1))
                } else {
                    None
                }
            }
            Direction::West => {
                if location.0 > 0 {
                    Some((location.0 - 1, location.1))
                } else {
                    None
                }
            }
            Direction::South => {
                if location.1 + 1 < bounds.1 {
                    Some((location.0, location.1 + 1))
                } else {
                    None
                }
            }
            Direction::East => {
                if location.0 + 1 < bounds.0 {
                    Some((location.0 + 1, location.1))
                } else {
                    None
                }
            }
        }
    }
}

fn calculate_energization(
    mirrors: &Vec<Vec<char>>,
    start: (usize, usize),
    start_direction: Direction,
) -> usize {
    let dimensions = (mirrors[0].len(), mirrors.len());
    let mut energized: HashMap<(usize, usize), Vec<Direction>> = HashMap::new();
    let mut beam_fronts: VecDeque<BeamFront> = VecDeque::new();

    beam_fronts.push_back(BeamFront {
        location: start,
        direction: start_direction,
    });

    while let Some(beam_front) = beam_fronts.pop_front() {
        // No use spinning in loops
        if let Some(directions) = energized.get(&beam_front.location) {
            if directions.contains(&beam_front.direction) {
                continue;
            }
        }

        // Register the location of the beam front as energized,
        // with the beam registered in the direction that the beam front entered the tile.
        energized
            .entry(beam_front.location)
            .and_modify(|directions| directions.push(beam_front.direction))
            .or_insert(vec![beam_front.direction]);

        let next_beam_directions: Vec<Direction> =
            match mirrors[beam_front.location.1][beam_front.location.0] {
                '.' => vec![beam_front.direction],
                '/' => match beam_front.direction {
                    Direction::North => vec![Direction::East],
                    Direction::West => vec![Direction::South],
                    Direction::South => vec![Direction::West],
                    Direction::East => vec![Direction::North],
                },
                '\\' => match beam_front.direction {
                    Direction::North => vec![Direction::West],
                    Direction::West => vec![Direction::North],
                    Direction::South => vec![Direction::East],
                    Direction::East => vec![Direction::South],
                },
                '-' => match beam_front.direction {
                    Direction::North | Direction::South => vec![Direction::East, Direction::West],
                    Direction::West => vec![Direction::West],
                    Direction::East => vec![Direction::East],
                },
                '|' => match beam_front.direction {
                    Direction::North => vec![Direction::North],
                    Direction::West | Direction::East => vec![Direction::North, Direction::South],
                    Direction::South => vec![Direction::South],
                },
                _ => unreachable!(),
            };

        for direction in next_beam_directions {
            if let Some(location) = direction.one_step(beam_front.location, dimensions) {
                beam_fronts.push_back(BeamFront {
                    location,
                    direction,
                });
            }
        }
    }

    energized.len()
}

pub fn solve(input: String) {
    let mirrors: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let part1 = calculate_energization(&mirrors, (0, 0), Direction::East);

    println!("Day 16 part 1: {}", part1);

    let left_edge = (0..mirrors.len()).map(|y| ((0usize, y), Direction::East));
    let right_edge = (0..mirrors.len()).map(|y| ((mirrors[0].len() - 1, y), Direction::West));
    let top_edge = (0..mirrors[0].len()).map(|x| ((x, 0usize), Direction::South));
    let bottom_edge = (0..mirrors[0].len()).map(|x| ((x, mirrors.len() - 1), Direction::North));

    let all_edges = left_edge
        .chain(right_edge)
        .chain(top_edge)
        .chain(bottom_edge);

    let part2 = all_edges
        .map(|(start, direction)| calculate_energization(&mirrors, start, direction))
        .max()
        .unwrap();

    println!("Day 16 part 2: {}", part2);
}
