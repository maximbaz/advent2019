use std::env;
use std::process;

mod config;
use config::*;

mod day1;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Error parsing arguments: {}", err);
        process::exit(1);
    });

    match config.part {
        Part::One => match config.day {
            Day::Day1 => day1::part1(&config.input),
            _ => panic!("this day is not solved yet!"),
        },
        Part::Two => match config.day {
            Day::Day1 => day1::part2(&config.input),
            _ => panic!("this day is not solved yet!"),
        },
    };
}
