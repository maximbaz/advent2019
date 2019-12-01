use itertools;
use std::fs;

pub fn part1(input: &str) {
    let contents = fs::read_to_string(input).expect("Error reading the file");
    let total_fuel = contents
        .lines()
        .map(|l| l.parse::<f64>().unwrap())
        .map(|f| (f / 3.0).floor() - 2.0)
        .sum::<f64>();

    println!("Day 1, Part 1: Total fuel = {}", total_fuel);
}

pub fn part2(input: &str) {
    let contents = fs::read_to_string(input).expect("Error reading the file");

    let total_fuel = contents
        .lines()
        .map(|l| l.parse::<f64>().unwrap())
        .map(|m| {
            itertools::iterate(m, |f| (f / 3.0).floor() - 2.0)
                .skip(1)
                .take_while(|f| *f >= 0.0)
                .sum::<f64>()
        })
        .sum::<f64>();

    println!("Day 1, Part 2: Total fuel = {}", total_fuel);
}
