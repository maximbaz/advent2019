use itertools::Itertools;
use std::fs;

use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;

pub fn run_part1() -> i32 {
    part1(input())
}

pub fn run_part2() -> i32 {
    part2(input())
}

fn input() -> Vec<i32> {
    fs::read_to_string("data/day7.txt")
        .expect("Error reading the file")
        .trim()
        .split(",")
        .flat_map(str::parse)
        .collect()
}

fn part1(input: Vec<i32>) -> i32 {
    solve(input, false)
}

fn part2(input: Vec<i32>) -> i32 {
    solve(input, true)
}

fn solve(memory: Vec<i32>, feedback_loop: bool) -> i32 {
    let forward = |from: Receiver<i32>, to: Sender<i32>| loop {
        if let Ok(msg) = from.recv() {
            if let Err(_) = to.send(msg) {
                return;
            }
        } else {
            return;
        }
    };

    if feedback_loop { (5..10) } else { (0..5) }
        .permutations(5)
        .map(|perm| {
            let (tx_input1, rx_input1) = mpsc::channel();
            let (tx_input2, rx_input2) = mpsc::channel();
            let (tx_input3, rx_input3) = mpsc::channel();
            let (tx_input4, rx_input4) = mpsc::channel();
            let (tx_input5, rx_input5) = mpsc::channel();
            let (tx_output1, rx_output1) = mpsc::channel();
            let (tx_output2, rx_output2) = mpsc::channel();
            let (tx_output3, rx_output3) = mpsc::channel();
            let (tx_output4, rx_output4) = mpsc::channel();
            let (tx_output5, rx_output5) = mpsc::channel();
            let (tx_final, rx_final) = mpsc::channel();
            let memory1 = memory.clone();
            let memory2 = memory.clone();
            let memory3 = memory.clone();
            let memory4 = memory.clone();
            let memory5 = memory.clone();
            thread::spawn(move || run(memory1, rx_input1, tx_output1, None));
            thread::spawn(move || run(memory2, rx_input2, tx_output2, None));
            thread::spawn(move || run(memory3, rx_input3, tx_output3, None));
            thread::spawn(move || run(memory4, rx_input4, tx_output4, None));
            thread::spawn(move || run(memory5, rx_input5, tx_output5, Some(tx_final)));

            tx_input1.send(perm[0]).unwrap();
            tx_input2.send(perm[1]).unwrap();
            tx_input3.send(perm[2]).unwrap();
            tx_input4.send(perm[3]).unwrap();
            tx_input5.send(perm[4]).unwrap();
            tx_input1.send(0).unwrap();

            thread::spawn(move || forward(rx_output1, tx_input2));
            thread::spawn(move || forward(rx_output2, tx_input3));
            thread::spawn(move || forward(rx_output3, tx_input4));
            thread::spawn(move || forward(rx_output4, tx_input5));
            if feedback_loop {
                thread::spawn(move || forward(rx_output5, tx_input1));
            }

            rx_final.recv().unwrap()
        })
        .max()
        .unwrap()
}

enum Result {
    Done,
    OutputProduced(Vec<i32>, usize, i32),
    OtherOk(Vec<i32>, usize),
}

fn run(
    mut mem: Vec<i32>,
    rx_input: Receiver<i32>,
    tx_output: Sender<i32>,
    tx_last_output: Option<Sender<i32>>,
) -> i32 {
    let mut pos = 0;
    let mut last_output = 0;

    loop {
        match execute(mem, pos, &rx_input) {
            Result::Done => {
                if let Some(chan) = tx_last_output {
                    chan.send(last_output).unwrap();
                }
                return last_output;
            }
            Result::OutputProduced(new_mem, new_pos, new_output) => {
                mem = new_mem;
                pos = new_pos;
                last_output = new_output;
                tx_output.send(new_output).unwrap();
            }
            Result::OtherOk(new_mem, new_pos) => {
                mem = new_mem;
                pos = new_pos;
            }
        }
    }
}

fn execute(memr: Vec<i32>, pos: usize, rx_input: &Receiver<i32>) -> Result {
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
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(
            43210,
            part1(vec![
                3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0,
            ])
        );

        assert_eq!(
            54321,
            part1(vec![
                3, 23, 3, 24, 1002, 24, 10, 24, 1002, 23, -1, 23, 101, 5, 23, 23, 1, 24, 23, 23, 4,
                23, 99, 0, 0
            ])
        );

        assert_eq!(
            65210,
            part1(vec![
                3, 31, 3, 32, 1002, 32, 10, 32, 1001, 31, -2, 31, 1007, 31, 0, 33, 1002, 33, 7, 33,
                1, 33, 31, 31, 1, 32, 31, 31, 4, 31, 99, 0, 0, 0
            ])
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            139629729,
            part2(vec![
                3, 26, 1001, 26, -4, 26, 3, 27, 1002, 27, 2, 27, 1, 27, 26, 27, 4, 27, 1001, 28,
                -1, 28, 1005, 28, 6, 99, 0, 0, 5
            ])
        );

        assert_eq!(
            18216,
            part2(vec![
                3, 52, 1001, 52, -5, 52, 3, 53, 1, 52, 56, 54, 1007, 54, 5, 55, 1005, 55, 26, 1001,
                54, -5, 54, 1105, 1, 12, 1, 53, 54, 53, 1008, 54, 0, 55, 1001, 55, 1, 55, 2, 53,
                55, 53, 4, 53, 1001, 56, -1, 56, 1005, 56, 6, 99, 0, 0, 0, 0, 10
            ])
        );
    }
}
