mod day01;

pub fn solve(input: String, day: u8) {
    match day {
        1 => day01::solve(input),
        _ => unimplemented!(),
    }
}
