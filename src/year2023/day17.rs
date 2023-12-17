use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
enum Direction {
    North,
    West,
    South,
    East,
}

impl Direction {
    fn left(&self) -> Self {
        match self {
            Direction::North => Direction::West,
            Direction::West => Direction::South,
            Direction::South => Direction::East,
            Direction::East => Direction::North,
        }
    }

    fn right(&self) -> Self {
        match self {
            Direction::North => Direction::East,
            Direction::West => Direction::North,
            Direction::South => Direction::West,
            Direction::East => Direction::South,
        }
    }

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

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct State {
    location: (usize, usize),
    direction: Direction,
    straight_steps: usize,
}

fn path_cost_of_least_heat_loss(
    heat_loss_map: &Vec<Vec<usize>>,
    start: (usize, usize),
    goal: (usize, usize),
    valid_straight_range: (usize, usize),
) -> Option<usize> {
    // Dimensions of the heat loss map (used for not path searching outside of the map.)
    let dimensions = (heat_loss_map[0].len(), heat_loss_map.len());

    // Total heat loss for getting to a particular state
    let mut losses: HashMap<State, usize> = HashMap::new();

    // Priority queue of states to be visited
    let mut heap = BinaryHeap::new();

    // Put the start location (with all travel directions) into the queue
    heap.push((
        Reverse(0),
        State {
            location: start,
            direction: Direction::North,
            straight_steps: 0,
        },
    ));
    heap.push((
        Reverse(0),
        State {
            location: start,
            direction: Direction::West,
            straight_steps: 0,
        },
    ));
    heap.push((
        Reverse(0),
        State {
            location: start,
            direction: Direction::South,
            straight_steps: 0,
        },
    ));
    heap.push((
        Reverse(0),
        State {
            location: start,
            direction: Direction::East,
            straight_steps: 0,
        },
    ));

    // Slightly modified Dijkstra for finding the cost of the least heat lossy path
    while let Some((heat_loss, state)) = heap.pop() {
        // Return when reaching the goal location; this must be one of the best paths
        if state.location == goal {
            return Some(heat_loss.0);
        }

        // Skip if there is a different way to get to this state, with equal or lower heeat loss
        if let Some(&prior_heat_loss) = losses.get(&state) {
            if heat_loss.0 >= prior_heat_loss {
                continue;
            }
        }

        /*
        println!(
            "Heap/hashmap size: {}/{}, state: {:?} / {:?} / {}, loss: {}",
            heap.len(),
            losses.len(),
            state.location,
            state.direction,
            state.straight_steps,
            heat_loss.0,
        );
        */

        // We must be at the least heat lossy path to the current state; register the heat loss
        losses.insert(state, heat_loss.0);

        // Push all possible "next states" to the queue
        // Continue straight on, if not having moved straight for too long
        if state.straight_steps < valid_straight_range.1 {
            if let Some(next_location) = state.direction.one_step(state.location, dimensions) {
                let next_state = State {
                    location: next_location,
                    direction: state.direction,
                    straight_steps: state.straight_steps + 1,
                };
                let next_heat_loss = heat_loss.0 + heat_loss_map[next_location.1][next_location.0];
                heap.push((Reverse(next_heat_loss), next_state));
            }
        }

        // Turn, if having moved straight for long enough
        if state.straight_steps >= valid_straight_range.0 {
            for movement_direction in [state.direction.left(), state.direction.right()] {
                if let Some(next_location) = movement_direction.one_step(state.location, dimensions)
                {
                    let next_state = State {
                        location: next_location,
                        direction: movement_direction,
                        straight_steps: 1,
                    };
                    let next_heat_loss =
                        heat_loss.0 + heat_loss_map[next_location.1][next_location.0];
                    heap.push((Reverse(next_heat_loss), next_state));
                }
            }
        }
    }

    // If we empty the queue before reaching the goal, then there simply is no path from start to
    // goal. (In this case, most likely a programming error.)
    None
}

pub fn solve(input: String) {
    let heat_loss_map: Vec<Vec<usize>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect::<Vec<usize>>()
        })
        .collect();

    let start = (0, 0);
    let goal = (heat_loss_map[0].len() - 1, heat_loss_map.len() - 1);
    let valid_straight_range = (0, 3);

    if let Some(part1) =
        path_cost_of_least_heat_loss(&heat_loss_map, start, goal, valid_straight_range)
    {
        println!("Day 17 part 1: {}", part1);
    }

    let valid_straight_range = (4, 10);

    if let Some(part2) =
        path_cost_of_least_heat_loss(&heat_loss_map, start, goal, valid_straight_range)
    {
        println!("Day 17 part 2: {}", part2);
    }
}
