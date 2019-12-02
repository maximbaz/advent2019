use itertools;
use std::fs;

pub fn prepare(input: &str) -> Vec<i32> {
    fs::read_to_string(input)
        .expect("Error reading the file")
        .lines()
        .flat_map(str::parse::<i32>)
        .collect()
}

pub fn part1(input: Vec<i32>) -> i32 {
    input.iter().map(|f| f / 3 - 2).sum::<i32>()
}

pub fn part2(input: Vec<i32>) -> i32 {
    input
        .iter()
        .map(|m| {
            itertools::iterate(*m, |f| f / 3 - 2)
                .skip(1)
                .take_while(|f| *f >= 0)
                .sum::<i32>()
        })
        .sum::<i32>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(2, part1(vec![12]));
        assert_eq!(2, part1(vec![14]));
        assert_eq!(654, part1(vec![1969]));
        assert_eq!(33583, part1(vec![100756]));
    }

    #[test]
    fn test_part2() {
        assert_eq!(2, part2(vec![14]));
        assert_eq!(966, part2(vec![1969]));
        assert_eq!(50346, part2(vec![100756]));
    }
}
