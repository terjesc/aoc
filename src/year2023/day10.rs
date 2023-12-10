pub fn solve(input: String) {

    #[derive(Clone, Copy)]
    struct Pipe {
        north: bool,
        south: bool,
        east: bool,
        west: bool,
        start: bool,
    }

    impl Pipe {
        fn from_char(c: char) -> Option<Pipe> {
            match c {
                '.' => None,
                'S' => Some(Pipe { north: true, south: true, east: true, west: true, start: true }),
                '|' => Some(Pipe { north: true, south: true, east: false, west: false, start: false }),
                '-' => Some(Pipe { north: false, south: false, east: true, west: true, start: false }),
                'F' => Some(Pipe { north: false, south: true, east: true, west: false, start: false }),
                'J' => Some(Pipe { north: true, south: false, east: false, west: true, start: false }),
                'L' => Some(Pipe { north: true, south: false, east: true, west: false, start: false }),
                '7' => Some(Pipe { north: false, south: true, east: false, west: true, start: false }),
                _ => unreachable!(),
            }
        }
    }

    let grid: Vec<Vec<Option<Pipe>>> = input.lines()
            .map(|line| line.chars().map(|c| Pipe::from_char(c)).collect())
            .collect();
    let dimensions = (grid.len(), grid[0].len());

    // Find start position
    let mut start_position: (usize, usize) = (usize::MAX, usize::MAX);
    for (y, pipe_row) in grid.iter().enumerate() {
        for (x, pipe) in pipe_row.iter().enumerate() {
            if let Some(pipe) = pipe {
                if pipe.start {
                    start_position = (x, y);
                }
            }
        }
    }
    let start_position = start_position;

    let mut position = start_position;
    let mut came_from: (usize, usize);
    let mut path: Vec<(usize, usize)> = Vec::new();

    // Find one neighbor of start, which is connected to start, and start on that neighbor
    let east = grid[position.1][position.0 + 1];
    let west = grid[position.1][position.0 - 1];
    let south = grid[position.1 + 1][position.0];

    if east.is_some() && east.unwrap().west {
        came_from = position;
        position = (position.0 + 1, position.1);
    } else if west.is_some() && west.unwrap().east {
        came_from = position;
        position = (position.0 - 1, position.0);
    } else if south.is_some() && south.unwrap().north {
        came_from = position;
        position = (position.0, position.1 + 1);
    } else {
        unreachable!();
    }

    fn northward(position: &(usize, usize)) -> (usize, usize) {
        (position.0, position.1 - 1)
    }

    fn southward(position: &(usize, usize)) -> (usize, usize) {
        (position.0, position.1 + 1)
    }

    fn eastward(position: &(usize, usize)) -> (usize, usize) {
        (position.0 + 1, position.1)
    }

    fn westward(position: &(usize, usize)) -> (usize, usize) {
        (position.0 - 1, position.1)
    }

    // The start position is on the pipe path
    path.push(start_position);

    // Traverse the pipe
    while position != start_position {
        let current_pipe = grid[position.1][position.0].unwrap();
        let next_position = if current_pipe.north && came_from != northward(&position) {
            northward(&position)
        } else if current_pipe.south && came_from != southward(&position) {
            southward(&position)
        } else if current_pipe.east && came_from != eastward(&position) {
            eastward(&position)
        } else if current_pipe.west && came_from != westward(&position) {
            westward(&position)
        } else {
            unreachable!()
        };

        path.push(position);

        came_from = position;
        position = next_position;
    }

    // The distance we are looking for is half-way through the pipe
    let part1 = path.len() / 2;

    println!("Day 10 part 1: {}", part1);

    fn is_corner_or_start(pipe: &Pipe) -> bool {
        pipe.start || pipe.north && pipe.east || pipe.north && pipe.west || pipe.south && pipe.east || pipe.south && pipe.west
    }

    // Calculate a compressed path of only corner pieces, to save some calculation later
    let corner_path: Vec<(usize, usize)> = path.iter()
            .filter(|(x, y)| {
                if let Some(pipe) = grid[*y][*x] {
                    is_corner_or_start(&pipe)
                } else {
                    false
                }
            })
            .cloned()
            .collect();

    // This algorithm checks if, from the perspective of 'position', tracing the 'path' leaves your
    // net rotation at zero (which means you are outside the path) or at a number of rotations
    // (which means the path surrounds you a number of times.
    fn is_inside_path(position: &(usize, usize), path: &Vec<(usize, usize)>) -> bool {
        #[derive(Clone, Copy, PartialEq)]
        enum Quadrant { NE, NW, SE, SW }

        fn quadrant(a: &(usize, usize), b: &(usize, usize)) -> Quadrant {
            let north: bool = a.1 < b.1;
            let west: bool = a.0 < b.0;

            match (north, west) {
                (true, true) => Quadrant::NW,
                (true, false) => Quadrant::NE,
                (false, true) => Quadrant::SW,
                (false, false) => Quadrant::SE,
            }
        }

        fn quadrant_rotation(a: &Quadrant, b: &Quadrant) -> i64 {
            match (a, b) {
                (Quadrant::NE, Quadrant::NW)
                    | (Quadrant::NW, Quadrant::SW)
                    | (Quadrant::SW, Quadrant::SE)
                    | (Quadrant::SE, Quadrant::NE) => 1,
                (Quadrant::NW, Quadrant::NE)
                    | (Quadrant::SW, Quadrant::NW)
                    | (Quadrant::SE, Quadrant::SW)
                    | (Quadrant::NE, Quadrant::SE) => -1,
                _ => if a == b { 0 } else { unreachable!() },
            }
        }

        let last_quadrant = quadrant(position, path.last().unwrap());

        let (_, rotation) = path.iter()
                .fold((last_quadrant, 0i64), |acc, path_position| {
                    let (last_quadrant, rotation) = acc;
                    let quadrant = quadrant(&position, &path_position);
                    (quadrant, rotation + quadrant_rotation(&last_quadrant, &quadrant))
                });

        // If rotation is zero then we are outside the path. Otherwise, we are inside.
        rotation != 0
    }

    let mut inside = 0;

    for x in 0..dimensions.0 {
        for y in 0..dimensions.1 {
            if !path.contains(&(x, y)) && is_inside_path(&(x, y), &corner_path) {
                inside += 1;
            }
        }
    }

    let part2 = inside;

    println!("Day 10 part 2: {}", part2);
}
