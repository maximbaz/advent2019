use itertools::Itertools;
use std::fs;

use std::sync::mpsc;
use std::sync::mpsc::Sender;
use std::thread;

use super::intcode::IntCode;

pub fn run_part1() -> i64 {
    part1(input())
}

pub fn run_part2() -> i64 {
    part2(input())
}

fn input() -> Vec<i64> {
    fs::read_to_string("data/day7.txt")
        .expect("Error reading the file")
        .trim()
        .split(",")
        .flat_map(str::parse)
        .collect()
}

fn part1(input: Vec<i64>) -> i64 {
    solve(input, false)
}

fn part2(input: Vec<i64>) -> i64 {
    solve(input, true)
}

fn solve(memory: Vec<i64>, feedback_loop: bool) -> i64 {
    let send = |chan: &Sender<i64>, val: i64| {
        chan.send(val).unwrap_or(());
    };
    if feedback_loop { (5..10) } else { (0..5) }
        .permutations(5)
        .map(|perm| {
            let (tx_input1, rx_input1) = mpsc::channel();
            let (tx_input2, rx_input2) = mpsc::channel();
            let (tx_input3, rx_input3) = mpsc::channel();
            let (tx_input4, rx_input4) = mpsc::channel();
            let (tx_input5, rx_input5) = mpsc::channel();
            let memory1 = memory.clone();
            let memory2 = memory.clone();
            let memory3 = memory.clone();
            let memory4 = memory.clone();
            let memory5 = memory.clone();
            tx_input1.send(perm[0]).unwrap();
            tx_input2.send(perm[1]).unwrap();
            tx_input3.send(perm[2]).unwrap();
            tx_input4.send(perm[3]).unwrap();
            tx_input5.send(perm[4]).unwrap();
            tx_input1.send(0).unwrap();

            thread::spawn(move || {
                IntCode::new(&memory1).run(
                    || rx_input1.recv().unwrap(),
                    |output| send(&tx_input2, output),
                );
            });
            thread::spawn(move || {
                IntCode::new(&memory2).run(
                    || rx_input2.recv().unwrap(),
                    |output| send(&tx_input3, output),
                );
            });
            thread::spawn(move || {
                IntCode::new(&memory3).run(
                    || rx_input3.recv().unwrap(),
                    |output| send(&tx_input4, output),
                );
            });
            thread::spawn(move || {
                IntCode::new(&memory4).run(
                    || rx_input4.recv().unwrap(),
                    |output| send(&tx_input5, output),
                );
            });
            thread::spawn(move || {
                let mut result = 0;
                IntCode::new(&memory5).run(
                    || rx_input5.recv().unwrap(),
                    |output| {
                        result = output;
                        if feedback_loop {
                            send(&tx_input1, output);
                        }
                    },
                );
                result
            })
            .join()
            .unwrap()
        })
        .max()
        .unwrap()
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
