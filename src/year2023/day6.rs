
pub fn solve(input: String) {
    let mut input = input.lines();
    
    // Read times
    let (_label, times) = input.next().unwrap().split_once(":").unwrap();
    let time_for_part2 = times;
    let times: Vec<i64> = times.split_whitespace()
        .map(|number| number.parse::<i64>().unwrap())
        .collect();

    // Read distances
    let (_label, distances) = input.next().unwrap().split_once(":").unwrap();
    let distance_for_part2 = distances;
    let distances: Vec<i64> = distances.split_whitespace()
        .map(|number| number.parse::<i64>().unwrap())
        .collect();

    fn calculate_distance(time: i64, hold_time: i64) -> i64 {
        let speed = hold_time;
        let remaining_time = time - hold_time;
        speed * remaining_time
    }

    let mut record_count_product: i64 = 1;

    for i in 0..times.len() {
        let mut record_count: i64 = 0;

        for hold_time in 1..times[i] {
            let distance = calculate_distance(times[i], hold_time);
//            println!("Time {}, hold time {}, distance {}", times[i], hold_time, distance);
            if distance > distances[i] {
                record_count += 1;
            }
        }

//        println!("{} hold times beat the record distance of {}", record_count, distances[i]);
        record_count_product *= record_count;
    }

    let part1 = record_count_product;

    println!("Day 6 part 1: {}", part1);

    let mut time = time_for_part2.to_string();
    time.retain(|c| !c.is_whitespace());
    let time = time.parse::<i64>().unwrap();

    let mut distance = distance_for_part2.to_string();
    distance.retain(|c| !c.is_whitespace());
    let distance = distance.parse::<i64>().unwrap();

    println!("Time {}, distance {}", time, distance);

    // Part 2:
    // Some variant of binary search or just calculating the zero points for the distance as a
    // function of time (minus the current record distance) would probably be a way faster and
    // better solution, but the numbers are low enough that a brute force approach works
    // wonderfully.

    let mut record_count: i64 = 0;

    for hold_time in 1..time {
        let new_distance = calculate_distance(time, hold_time);
        if new_distance > distance {
            record_count += 1;
        }
    }

    let part2 = record_count;

    println!("Day 6 part 2: {}", part2);
}
