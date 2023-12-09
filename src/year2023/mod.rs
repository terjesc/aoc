mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

pub fn solve(input: String, day: u8) {
    match day {
        1 => day1::solve(input),
        2 => day2::solve(input),
        3 => day3::solve(input),
        4 => day4::solve(input),
        5 => day5::solve(input),
        6 => day6::solve(input),
        7 => day7::solve(input),
        8 => day8::solve(input),
        9 => day9::solve(input),
        _ => unimplemented!(),
    }
}
