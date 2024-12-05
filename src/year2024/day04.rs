use std::collections::HashMap;

pub fn solve(input: String) {
    let xmas: Vec<char> = "XMAS".chars().collect();

    let word_matrix: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect();

    let directions: Vec<(i32, i32)> = vec![
        (1, 0),
        (-1, 0),
        (0, 1),
        (0, -1),
        (1, 1),
        (-1, -1),
        (-1, 1),
        (1, -1),
    ];

    let mut part1 = 0;

    for (dx, dy) in directions {
        for x in 0..word_matrix[0].len() as i32 {
            for y in 0..word_matrix.len() as i32 {
                for i in 0..xmas.len() as i32 {
                    let dx = i * dx;
                    let dy = i * dy;
                    if x + dx < 0 || x + dx >= word_matrix[0].len() as i32 {
                        break;
                    }
                    if y + dy < 0 || y + dy >= word_matrix.len() as i32 {
                        break;
                    }
                    if word_matrix[(y + dy) as usize][(x + dx) as usize] != xmas[i as usize] {
                        break;
                    }
                    if i == xmas.len() as i32 - 1 {
                        part1 += 1;
                    }
                }
            }
        }
    }

    println!("Day 4 part 1: {}", part1);

    let mas: Vec<char> = "MAS".chars().collect();

    let diagonal_directions: Vec<(i32, i32)> = vec![(1, 1), (-1, -1), (-1, 1), (1, -1)];

    let mut mas_a_locations: HashMap<(i32, i32), i32> = HashMap::new();

    for (unit_dx, unit_dy) in diagonal_directions {
        for x in 0..word_matrix[0].len() as i32 {
            for y in 0..word_matrix.len() as i32 {
                for i in 0..mas.len() as i32 {
                    let dx = i * unit_dx;
                    let dy = i * unit_dy;
                    if x + dx < 0 || x + dx >= word_matrix[0].len() as i32 {
                        break;
                    }
                    if y + dy < 0 || y + dy >= word_matrix.len() as i32 {
                        break;
                    }
                    if word_matrix[(y + dy) as usize][(x + dx) as usize] != mas[i as usize] {
                        break;
                    }
                    if i == mas.len() as i32 - 1 {
                        *mas_a_locations
                            .entry((x + unit_dx, y + unit_dy))
                            .or_insert(0) += 1;
                    }
                }
            }
        }
    }

    let part2 = mas_a_locations.values().filter(|&v| *v == 2).count();
    println!("Day 4 part 2: {}", part2);
}
