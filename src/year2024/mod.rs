mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;

pub fn solve(input: String, day: u8) {
    match day {
        1 => day01::solve(input),
        2 => day02::solve(input),
        3 => day03::solve(input),
        4 => day04::solve(input),
        5 => day05::solve(input),
        6 => day06::solve(input),
        _ => unimplemented!(),
    }
}
