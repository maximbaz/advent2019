use std::env;
use std::process;

mod config;
use config::*;

mod day1;
mod day2;
mod day3;
mod day4;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Error parsing arguments: {}", err);
        process::exit(1);
    });

    match config.part {
        Part::One => match config.day {
            Day::Day1 => println!("Solution: {}", day1::run_part1()),
            Day::Day2 => println!("Solution: {}", day2::run_part1()),
            Day::Day3 => println!("Solution: {}", day3::run_part1()),
            Day::Day4 => println!("Solution: {}", day4::run_part1()),
            _ => panic!("this day is not solved yet!"),
        },
        Part::Two => match config.day {
            Day::Day1 => println!("Solution: {}", day1::run_part2()),
            Day::Day2 => println!("Solution: {}", day2::run_part2()),
            Day::Day3 => println!("Solution: {}", day3::run_part2()),
            Day::Day4 => println!("Solution: {}", day4::run_part2()),
            _ => panic!("this day is not solved yet!"),
        },
    };
}
