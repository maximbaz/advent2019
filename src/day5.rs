use std::fs;

pub fn run_part1() -> i32 {
    part1(input())
}

pub fn run_part2() -> i32 {
    part2(input())
}

fn input() -> Vec<i32> {
    fs::read_to_string("data/day5.txt")
        .expect("Error reading the file")
        .trim()
        .split(",")
        .flat_map(str::parse)
        .collect()
}

fn part1(input: Vec<i32>) -> i32 {
    solve(input, 1)
}

fn part2(input: Vec<i32>) -> i32 {
    solve(input, 5)
}

fn solve(mut mem: Vec<i32>, input: i32) -> i32 {
    let mut pos = 0;
    let mut out = 0;

    loop {
        match execute(mem, input, pos, out) {
            Ok((mem_new, pos_new, out_new)) => {
                mem = mem_new;
                pos = pos_new;
                out = out_new;
            }
            Err(_) => return out,
        }
    }
}

fn execute(
    memr: Vec<i32>,
    input: i32,
    pos: usize,
    output: i32,
) -> Result<(Vec<i32>, usize, i32), ()> {
    let mut memw = memr.clone();

    let is_pos_mode = |i: usize| (memr[pos] / 10i32.pow(i as u32 + 1)) % 10 == 0;

    let read = |i: usize| {
        if is_pos_mode(i) {
            memr[memr[pos + i] as usize]
        } else {
            memr[pos + i]
        }
    };

    let mut write = |i: usize, val: i32| {
        memw[memr[pos + i] as usize] = val;
    };

    match memr[pos] % 100 {
        99 => Err(()),
        1 => {
            write(3, read(1) + read(2));
            Ok((memw, pos + 4, output))
        }
        2 => {
            write(3, read(1) * read(2));
            Ok((memw, pos + 4, output))
        }
        3 => {
            write(1, input);
            Ok((memw, pos + 2, output))
        }
        4 => Ok((memw, pos + 2, read(1))),
        5 => {
            if read(1) != 0 {
                Ok((memw, read(2) as usize, output))
            } else {
                Ok((memw, pos + 3, output))
            }
        }
        6 => {
            if read(1) == 0 {
                Ok((memw, read(2) as usize, output))
            } else {
                Ok((memw, pos + 3, output))
            }
        }
        7 => {
            write(3, if read(1) < read(2) { 1 } else { 0 });
            Ok((memw, pos + 4, output))
        }
        8 => {
            write(3, if read(1) == read(2) { 1 } else { 0 });
            Ok((memw, pos + 4, output))
        }
        _ => unreachable!(),
    }
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
