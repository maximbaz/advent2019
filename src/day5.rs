use std::fs;

use super::intcode::IntCode;

pub fn run_part1() -> i64 {
    part1(input())
}

pub fn run_part2() -> i64 {
    part2(input())
}

fn input() -> Vec<i64> {
    fs::read_to_string("data/day5.txt")
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
    solve(input, 5)
}

fn solve(mem: Vec<i64>, input: i64) -> i64 {
    let mut result = 0;

    IntCode::new(&mem).run(|| input, |output| result = output);

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_equal_less() {
        assert_eq!(0, solve(vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8], 7));
        assert_eq!(1, solve(vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8], 8));
        assert_eq!(0, solve(vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8], 9));

        assert_eq!(1, solve(vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8], 7));
        assert_eq!(0, solve(vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8], 8));
        assert_eq!(0, solve(vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8], 9));

        assert_eq!(0, solve(vec![3, 3, 1108, -1, 8, 3, 4, 3, 99], 7));
        assert_eq!(1, solve(vec![3, 3, 1108, -1, 8, 3, 4, 3, 99], 8));
        assert_eq!(0, solve(vec![3, 3, 1108, -1, 8, 3, 4, 3, 99], 9));

        assert_eq!(1, solve(vec![3, 3, 1107, -1, 8, 3, 4, 3, 99], 7));
        assert_eq!(0, solve(vec![3, 3, 1107, -1, 8, 3, 4, 3, 99], 8));
        assert_eq!(0, solve(vec![3, 3, 1107, -1, 8, 3, 4, 3, 99], 9));
    }

    #[test]
    fn test_solve_jump() {
        assert_eq!(
            0,
            solve(
                vec![3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9],
                0
            )
        );
        assert_eq!(
            1,
            solve(
                vec![3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9],
                9
            )
        );

        assert_eq!(
            0,
            solve(vec![3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1], 0)
        );
        assert_eq!(
            1,
            solve(vec![3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1], 9)
        );
    }

    #[test]
    fn test_solve_large() {
        assert_eq!(
            999,
            solve(
                vec![
                    3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0,
                    36, 98, 0, 0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46,
                    1101, 1000, 1, 20, 4, 20, 1105, 1, 46, 98, 99
                ],
                7
            )
        );
        assert_eq!(
            1000,
            solve(
                vec![
                    3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0,
                    36, 98, 0, 0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46,
                    1101, 1000, 1, 20, 4, 20, 1105, 1, 46, 98, 99
                ],
                8
            )
        );
        assert_eq!(
            1001,
            solve(
                vec![
                    3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0,
                    36, 98, 0, 0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46,
                    1101, 1000, 1, 20, 4, 20, 1105, 1, 46, 98, 99
                ],
                9
            )
        );
    }
}
