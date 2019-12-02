pub enum Day {
    Day1,
    Day2,
    Day3,
    Day4,
    Day5,
    Day6,
    Day7,
    Day8,
    Day9,
    Day10,
    Day11,
    Day12,
    Day13,
    Day14,
    Day15,
    Day16,
    Day17,
    Day18,
    Day19,
    Day20,
    Day21,
    Day22,
    Day23,
    Day24,
    Day25,
}

pub enum Part {
    One,
    Two,
}

pub struct Config {
    pub day: Day,
    pub part: Part,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let day = match args[1].as_str() {
            "day1" => Day::Day1,
            "day2" => Day::Day2,
            "day3" => Day::Day3,
            "day4" => Day::Day4,
            "day5" => Day::Day5,
            "day6" => Day::Day6,
            "day7" => Day::Day7,
            "day8" => Day::Day8,
            "day9" => Day::Day9,
            "day10" => Day::Day10,
            "day11" => Day::Day11,
            "day12" => Day::Day12,
            "day13" => Day::Day13,
            "day14" => Day::Day14,
            "day15" => Day::Day15,
            "day16" => Day::Day16,
            "day17" => Day::Day17,
            "day18" => Day::Day18,
            "day19" => Day::Day19,
            "day20" => Day::Day20,
            "day21" => Day::Day21,
            "day22" => Day::Day22,
            "day23" => Day::Day23,
            "day24" => Day::Day24,
            "day25" => Day::Day25,
            _ => return Err("unexpected value for 'day' argument"),
        };

        let part = match args[2].as_str() {
            "part1" => Part::One,
            "part2" => Part::Two,
            _ => return Err("unexpected value for 'part' argument"),
        };

        Ok(Config { day, part })
    }
}
