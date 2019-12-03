use std::env;
use std::process;

mod config;
use config::*;

mod day1;
mod day2;
mod day3;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Error parsing arguments: {}", err);
        process::exit(1);
    });

    match config.part {
        Part::One => match config.day {
            Day::Day1 => println!("Solution: {}", day1::part1(day1::input())),
            Day::Day2 => println!("Solution: {}", day2::part1(day2::input())),
            Day::Day3 => println!("Solution: {}", day3::part1(day3::input())),
            _ => panic!("this day is not solved yet!"),
        },
        Part::Two => match config.day {
            Day::Day1 => println!("Solution: {}", day1::part2(day1::input())),
            Day::Day2 => println!("Solution: {}", day2::part2(day2::input())),
            Day::Day3 => println!("Solution: {}", day3::part2(day3::input())),
            _ => panic!("this day is not solved yet!"),
        },
    };
}
