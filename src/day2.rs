use std::fs;

fn read_file(file: &str) -> Vec<i64> {
    fs::read_to_string(file)
        .expect("Error reading the file")
        .trim()
        .split(",")
        .flat_map(str::parse)
        .collect()
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

pub fn part1(input: &str) {
    let solution = solve(read_file(input), 12, 2);
    println!("Day 2, part 1: position 0 contains {}", solution);
}

pub fn part2(input: &str) {
    let memory = read_file(input);

    for start1 in 0..99 {
        for start2 in 0..99 {
            let solution = solve(memory.clone(), start1, start2);
            if solution == 19690720 {
                println!("Day 2, part 2: the answer is {}", 100 * start1 + start2);
                return;
            }
        }
    }
}
