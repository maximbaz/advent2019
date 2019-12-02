use itertools;
use std::fs;

pub fn part1(input: &str) {
    let contents = fs::read_to_string(input).expect("Error reading the file");
    let total_fuel = contents
        .lines()
        .flat_map(str::parse::<i32>)
        .map(|f| f / 3 - 2)
        .sum::<i32>();

    println!("Day 1, Part 1: Total fuel = {}", total_fuel);
}

pub fn part2(input: &str) {
    let contents = fs::read_to_string(input).expect("Error reading the file");

    let total_fuel = contents
        .lines()
        .flat_map(str::parse::<i32>)
        .map(|m| {
            itertools::iterate(m, |f| f / 3 - 2)
                .skip(1)
                .take_while(|f| *f >= 0)
                .sum::<i32>()
        })
        .sum::<i32>();

    println!("Day 1, Part 2: Total fuel = {}", total_fuel);
}
