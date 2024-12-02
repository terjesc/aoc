mod day01;
mod day02;

pub fn solve(input: String, day: u8) {
    match day {
        1 => day01::solve(input),
        2 => day02::solve(input),
        _ => unimplemented!(),
    }
}
