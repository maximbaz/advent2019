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

fn parse_opcode(mut n: i32) -> Vec<i8> {
    let mut digits = Vec::new();
    while n > 9 {
        digits.push((n % 10) as i8);
        n = n / 10;
    }
    digits.push(n as i8);
    for _ in 0..5 {
        digits.push(0);
    }
    digits
}

fn solve(mut memory: Vec<i32>, input: i32) -> i32 {
    let mut pos = 0;

    let mut output = -1;
    while memory[pos] != 99 {
        let opcode = parse_opcode(memory[pos]);
        match opcode[0] {
            1 => {
                let a = memory[pos + 1];
                let a = if opcode[2] == 0 {
                    memory[a as usize]
                } else {
                    a
                };
                let b = memory[pos + 2];
                let b = if opcode[3] == 0 {
                    memory[b as usize]
                } else {
                    b
                };
                let dst = memory[pos + 3] as usize;
                memory[dst] = a + b;
                pos += 4;
            }
            2 => {
                let a = memory[pos + 1];
                let a = if opcode[2] == 0 {
                    memory[a as usize]
                } else {
                    a
                };
                let b = memory[pos + 2];
                let b = if opcode[3] == 0 {
                    memory[b as usize]
                } else {
                    b
                };
                let dst = memory[pos + 3] as usize;
                memory[dst] = a * b;
                pos += 4;
            }
            3 => {
                let dst = memory[pos + 1] as usize;
                memory[dst] = input;
                pos += 2;
            }
            4 => {
                let dst = memory[pos + 1] as usize;
                output = memory[dst];
                pos += 2;
            }
            5 => {
                let a = memory[pos + 1];
                let a = if opcode[2] == 0 {
                    memory[a as usize]
                } else {
                    a
                };
                let b = memory[pos + 2];
                let b = if opcode[3] == 0 {
                    memory[b as usize]
                } else {
                    b
                };
                if a != 0 {
                    pos = b as usize;
                } else {
                    pos += 3;
                }
            }
            6 => {
                let a = memory[pos + 1];
                let a = if opcode[2] == 0 {
                    memory[a as usize]
                } else {
                    a
                };
                let b = memory[pos + 2];
                let b = if opcode[3] == 0 {
                    memory[b as usize]
                } else {
                    b
                };
                if a == 0 {
                    pos = b as usize;
                } else {
                    pos += 3;
                }
            }
            7 => {
                let a = memory[pos + 1];
                let a = if opcode[2] == 0 {
                    memory[a as usize]
                } else {
                    a
                };
                let b = memory[pos + 2];
                let b = if opcode[3] == 0 {
                    memory[b as usize]
                } else {
                    b
                };
                let dst = memory[pos + 3] as usize;
                memory[dst] = if a < b { 1 } else { 0 };
                pos += 4;
            }
            8 => {
                let a = memory[pos + 1];
                let a = if opcode[2] == 0 {
                    memory[a as usize]
                } else {
                    a
                };
                let b = memory[pos + 2];
                let b = if opcode[3] == 0 {
                    memory[b as usize]
                } else {
                    b
                };
                let dst = memory[pos + 3] as usize;
                memory[dst] = if a == b { 1 } else { 0 };
                pos += 4;
            }
            _ => unreachable!(),
        }
    }
    output
}
