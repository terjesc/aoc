use std::fs::read_to_string;
use std::path::Path;

use clap::Parser;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    // Day of the challenge
    #[arg(short, long)]
    day: u8,

    // Input file for the challenge
    #[arg(short, long)]
    input_file: String,
}

fn main() {
    println!("AoC 2023");
    let args = Args::parse();

    let path = Path::new(&args.input_file);
    let input = read_to_string(&path).unwrap();

    match args.day {
        1 => day1::solve(input),
        2 => day2::solve(input),
        3 => day3::solve(input),
        4 => day4::solve(input),
        5 => day5::solve(input),
        6 => day6::solve(input),
        7 => day7::solve(input),
        8 => day8::solve(input),
        _ => unimplemented!(),
    }
}
