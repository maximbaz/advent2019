use std::cell::Cell;
use std::fs;

use super::intcode::IntCode;

pub fn run_part1() -> i64 {
    part1(input())
}

pub fn run_part2() -> i64 {
    part2(input())
}

fn input() -> Vec<i64> {
    fs::read_to_string("data/day13.txt")
        .expect("Error reading the file")
        .trim()
        .split(",")
        .flat_map(str::parse)
        .collect()
}

fn part1(memory: Vec<i64>) -> i64 {
    let mut output_count = 0;
    let mut blocks_count = 0;

    IntCode::new(&memory).run(
        || 0,
        |output| {
            if output_count < 2 {
                output_count += 1;
                return;
            }
            output_count = 0;

            if output == 2 {
                blocks_count += 1;
            }
        },
    );

    blocks_count
}

fn part2(mut memory: Vec<i64>) -> i64 {
    memory[0] = 2;

    let mut output_queue = Vec::new();
    let mut score = 0;
    let paddle_x = Cell::new(0i64);
    let ball_x = Cell::new(0i64);

    IntCode::new(&memory).run(
        || (ball_x.get() - paddle_x.get()).signum(),
        |output| {
            if output_queue.len() < 2 {
                output_queue.push(output);
                return;
            }

            if output_queue[0] == -1 && output_queue[1] == 0 {
                score = output;
            } else if output == 3 {
                paddle_x.set(output_queue[0]);
            } else if output == 4 {
                ball_x.set(output_queue[0]);
            }

            output_queue.clear();
        },
    );

    score
}
