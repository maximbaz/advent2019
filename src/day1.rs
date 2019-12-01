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

    fn calc(mut mass: f64) -> f64 {
        let mut total_fuel = 0.0;
        loop {
            mass = (mass / 3.0).floor() - 2.0;
            if mass <= 0.0 {
                break;
            }
            total_fuel += mass;
        }
        return total_fuel;
    }

    let total_fuel = contents
        .lines()
        .map(|l| l.parse::<f64>().unwrap())
        .map(calc)
        .sum::<f64>();

    println!("Day 1, Part 2: Total fuel = {}", total_fuel);
}
