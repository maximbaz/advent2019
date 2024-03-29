use std::env;
use std::process;

mod config;
use config::*;

mod intcode;

mod day1;
mod day10;
mod day13;
mod day14;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

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
            Day::Day5 => println!("Solution: {}", day5::run_part1()),
            Day::Day6 => println!("Solution: {}", day6::run_part1()),
            Day::Day7 => println!("Solution: {}", day7::run_part1()),
            Day::Day8 => println!("Solution: {}", day8::run_part1()),
            Day::Day9 => println!("Solution: {}", day9::run_part1()),
            Day::Day10 => println!("Solution: {}", day10::run_part1()),
            Day::Day13 => println!("Solution: {}", day13::run_part1()),
            Day::Day14 => println!("Solution: {}", day14::run_part1()),
            _ => panic!("this day is not solved yet!"),
        },
        Part::Two => match config.day {
            Day::Day1 => println!("Solution: {}", day1::run_part2()),
            Day::Day2 => println!("Solution: {}", day2::run_part2()),
            Day::Day3 => println!("Solution: {}", day3::run_part2()),
            Day::Day4 => println!("Solution: {}", day4::run_part2()),
            Day::Day5 => println!("Solution: {}", day5::run_part2()),
            Day::Day6 => println!("Solution: {}", day6::run_part2()),
            Day::Day7 => println!("Solution: {}", day7::run_part2()),
            Day::Day8 => println!("Solution: {}", day8::run_part2()),
            Day::Day9 => println!("Solution: {}", day9::run_part2()),
            Day::Day10 => println!("Solution: {}", day10::run_part2()),
            Day::Day13 => println!("Solution: {}", day13::run_part2()),
            Day::Day14 => println!("Solution: {}", day14::run_part2()),
            _ => panic!("this day is not solved yet!"),
        },
    };
}
