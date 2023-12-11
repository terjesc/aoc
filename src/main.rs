use std::fs::read_to_string;
use std::path::Path;

use clap::Parser;

mod year2023;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    // Year of the event
    #[arg(short, long, default_value_t = 2023)]
    year: u16,

    // Day of the challenge
    #[arg(short, long)]
    day: u8,

    // Input file for the challenge
    #[arg(short, long)]
    input_file: String,
}

fn main() {
    let args = Args::parse();
    println!("AoC {}", args.year);

    let path = Path::new(&args.input_file);
    let input = read_to_string(path).unwrap();

    match (args.year, args.day) {
        (2023, day) => year2023::solve(input, day),
        _ => unimplemented!(),
    }
}
