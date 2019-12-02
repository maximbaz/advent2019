use std::fs;

pub fn input() -> Vec<i64> {
    fs::read_to_string("data/day2.txt")
        .expect("Error reading the file")
        .trim()
        .split(",")
        .flat_map(str::parse)
        .collect()
}

pub fn part1(input: Vec<i64>) -> i64 {
    solve(input, 12, 2)
}

pub fn part2(input: Vec<i64>) -> i64 {
    for start1 in 0..99 {
        for start2 in 0..99 {
            let solution = solve(input.clone(), start1, start2);
            if solution == 19690720 {
                return 100 * start1 + start2;
            }
        }
    }
    unreachable!()
}

fn solve(mut memory: Vec<i64>, start1: i64, start2: i64) -> i64 {
    memory[1] = start1;
    memory[2] = start2;

    let mut pos = 0;

    while memory[pos] != 99 {
        let a = memory[memory[pos + 1] as usize];
        let b = memory[memory[pos + 2] as usize];
        let dst = memory[pos + 3] as usize;
        memory[dst] = if memory[pos] == 1 { a + b } else { a * b };
        pos += 4;
    }

    return memory[0];
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        assert_eq!(
            3500,
            solve(vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50], 9, 10)
        );

        assert_eq!(2, solve(vec![1, 0, 0, 0, 99], 0, 0));
        assert_eq!(2, solve(vec![2, 3, 0, 3, 99], 3, 0));
        assert_eq!(2, solve(vec![2, 4, 4, 5, 99, 0], 4, 4));
        assert_eq!(30, solve(vec![1, 1, 1, 4, 99, 5, 6, 0, 99], 1, 1));
    }
}
