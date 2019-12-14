use std::fs;

use super::intcode::IntCode;

pub fn run_part1() -> i64 {
    part1(input())
}

pub fn run_part2() -> i64 {
    part2(input())
}

fn input() -> Vec<i64> {
    fs::read_to_string("data/day9.txt")
        .expect("Error reading the file")
        .trim()
        .split(",")
        .flat_map(str::parse)
        .collect()
}

fn part1(input: Vec<i64>) -> i64 {
    solve(input, 1)
}

fn part2(input: Vec<i64>) -> i64 {
    solve(input, 2)
}

fn solve(memory: Vec<i64>, input: i64) -> i64 {
    let mut result = 0;

    IntCode::new(&memory).run(|| input, |output| result = output);

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(
            99,
            part1(vec![
                109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99
            ])
        );

        assert_eq!(
            1219070632396864,
            part1(vec![1102, 34915192, 34915192, 7, 4, 7, 99, 0])
        );

        assert_eq!(1125899906842624, part1(vec![104, 1125899906842624, 99]));
    }
}
