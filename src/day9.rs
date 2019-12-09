use std::fs;

use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;

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

fn solve(mut memory: Vec<i64>, input: i64) -> i64 {
    // cheat by adding some buffer to the vector
    for _ in 0..100000 {
        memory.push(0);
    }

    let (tx_input, rx_input) = mpsc::channel();
    let (tx_final, rx_final) = mpsc::channel();
    thread::spawn(move || run(memory, &rx_input, None, Some(&tx_final)));

    tx_input.send(input).unwrap();

    rx_final.recv().unwrap()
}

enum Result {
    Done,
    OutputProduced(Vec<i64>, usize, i64),
    RelPosChanged(Vec<i64>, usize, usize),
    OtherOk(Vec<i64>, usize),
}

fn run(
    mut mem: Vec<i64>,
    rx_input: &Receiver<i64>,
    tx_output: Option<&Sender<i64>>,
    tx_last_output: Option<&Sender<i64>>,
) -> i64 {
    let mut pos = 0;
    let mut rel_pos = 0;
    let mut last_output = 0;

    loop {
        match execute(mem, pos, rel_pos, rx_input) {
            Result::Done => {
                if let Some(last_output_channel) = tx_last_output {
                    last_output_channel.send(last_output).unwrap();
                }
                return last_output;
            }
            Result::OutputProduced(new_mem, new_pos, new_output) => {
                mem = new_mem;
                pos = new_pos;
                last_output = new_output;
                if let Some(output_channel) = tx_output {
                    output_channel.send(new_output).unwrap();
                }
            }
            Result::RelPosChanged(new_mem, new_pos, new_rel_pos) => {
                mem = new_mem;
                pos = new_pos;
                rel_pos = new_rel_pos;
            }
            Result::OtherOk(new_mem, new_pos) => {
                mem = new_mem;
                pos = new_pos;
            }
        }
    }
}

fn execute(memr: Vec<i64>, pos: usize, rel_pos: usize, rx_input: &Receiver<i64>) -> Result {
    let mut memw = memr.clone();

    let is_pos_mode = |i: usize| (*memr.get(pos).unwrap_or(&0) / 10i64.pow(i as u32 + 1)) % 10 == 0;
    let is_rel_mode = |i: usize| (*memr.get(pos).unwrap_or(&0) / 10i64.pow(i as u32 + 1)) % 10 == 2;

    let read = |i: usize| {
        if is_pos_mode(i) {
            memr[memr[pos + i] as usize]
        } else if is_rel_mode(i) {
            memr[(rel_pos as i64 + memr[pos + i]) as usize]
        } else {
            memr[pos + i]
        }
    };

    let mut write = |i: usize, val: i64| {
        if is_pos_mode(i) {
            memw[memr[pos + i] as usize] = val;
        } else if is_rel_mode(i) {
            memw[(rel_pos as i64 + memr[pos + i]) as usize] = val;
        } else {
            panic!("write can never occur in immediate mode!");
        }
    };

    match memr[pos] % 100 {
        99 => Result::Done,
        1 => {
            write(3, read(1) + read(2));
            Result::OtherOk(memw, pos + 4)
        }
        2 => {
            write(3, read(1) * read(2));
            Result::OtherOk(memw, pos + 4)
        }
        3 => {
            write(1, rx_input.recv().unwrap());
            Result::OtherOk(memw, pos + 2)
        }
        4 => Result::OutputProduced(memw, pos + 2, read(1)),
        5 => {
            if read(1) != 0 {
                Result::OtherOk(memw, read(2) as usize)
            } else {
                Result::OtherOk(memw, pos + 3)
            }
        }
        6 => {
            if read(1) == 0 {
                Result::OtherOk(memw, read(2) as usize)
            } else {
                Result::OtherOk(memw, pos + 3)
            }
        }
        7 => {
            write(3, if read(1) < read(2) { 1 } else { 0 });
            Result::OtherOk(memw, pos + 4)
        }
        8 => {
            write(3, if read(1) == read(2) { 1 } else { 0 });
            Result::OtherOk(memw, pos + 4)
        }
        9 => Result::RelPosChanged(memw, pos + 2, (rel_pos as i64 + read(1)) as usize),
        _ => unreachable!(),
    }
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
