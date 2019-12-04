use std::fs;

pub fn run_part1() -> i32 {
    let (line1, line2) = input();
    part1(line1, line2)
}

pub fn run_part2() -> i32 {
    let (line1, line2) = input();
    part2(line1, line2)
}

#[derive(Clone)]
enum Segment {
    U(i32),
    D(i32),
    L(i32),
    R(i32),
}

#[derive(PartialEq, Clone)]
enum Square {
    Empty,
    Line1(i32),
    Line2,
    Cross(i32),
}

type Line = Vec<Segment>;

fn input() -> (Line, Line) {
    let lines: Vec<Vec<Segment>> = fs::read_to_string("data/day3.txt")
        .expect("Error reading the file")
        .trim()
        .lines()
        .map(|line| {
            line.split(",")
                .map(|s| match s.chars().next().unwrap() {
                    'U' => Segment::U(s[1..].parse().unwrap()),
                    'D' => Segment::D(s[1..].parse().unwrap()),
                    'L' => Segment::L(s[1..].parse().unwrap()),
                    'R' => Segment::R(s[1..].parse().unwrap()),
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect();

    (lines[0].clone(), lines[1].clone())
}

const SIZE: usize = 24000;
const START: usize = 10000;

fn build_map(line1: Line, line2: Line) -> Vec<Vec<Square>> {
    println!("allocating");
    let mut map: Vec<Vec<Square>> = vec![vec![Square::Empty; SIZE]; SIZE];

    println!("calculating");
    let mut x = START;
    let mut y = START;
    let mut step = 0;
    for segment in line1 {
        match segment {
            Segment::U(n) => {
                for _i in 0..n {
                    x -= 1;
                    step += 1;
                    map[x][y] = Square::Line1(step);
                }
            }
            Segment::D(n) => {
                for _i in 0..n {
                    x += 1;
                    step += 1;
                    map[x][y] = Square::Line1(step);
                }
            }
            Segment::L(n) => {
                for _i in 0..n {
                    y -= 1;
                    step += 1;
                    map[x][y] = Square::Line1(step);
                }
            }
            Segment::R(n) => {
                for _i in 0..n {
                    y += 1;
                    step += 1;
                    map[x][y] = Square::Line1(step);
                }
            }
        }
    }

    println!("steps: {}", step);

    x = START;
    y = START;
    step = 0;

    for segment in line2 {
        match segment {
            Segment::U(n) => {
                for _i in 0..n {
                    x -= 1;
                    step += 1;
                    map[x][y] = if let Square::Line1(n) = map[x][y] {
                        Square::Cross(n + step)
                    } else {
                        Square::Line2
                    };
                }
            }
            Segment::D(n) => {
                for _i in 0..n {
                    x += 1;
                    step += 1;
                    map[x][y] = if let Square::Line1(n) = map[x][y] {
                        Square::Cross(n + step)
                    } else {
                        Square::Line2
                    };
                }
            }
            Segment::L(n) => {
                for _i in 0..n {
                    y -= 1;
                    step += 1;
                    map[x][y] = if let Square::Line1(n) = map[x][y] {
                        Square::Cross(n + step)
                    } else {
                        Square::Line2
                    };
                }
            }
            Segment::R(n) => {
                for _i in 0..n {
                    y += 1;
                    step += 1;
                    map[x][y] = if let Square::Line1(n) = map[x][y] {
                        Square::Cross(n + step)
                    } else {
                        Square::Line2
                    };
                }
            }
        }
    }

    map
}

fn part1(line1: Line, line2: Line) -> i32 {
    let map = build_map(line1, line2);
    println!("searching");

    let mut min_dist = 999999999;

    for i in 0..map.len() {
        for j in 0..map.len() {
            if let Square::Cross(_) = map[i][j] {
                let dist = (i as i32 - START as i32).abs() + (j as i32 - START as i32).abs();
                if dist < min_dist {
                    min_dist = dist;
                }
            }
        }
    }

    min_dist
}

fn part2(line1: Line, line2: Line) -> i32 {
    let map = build_map(line1, line2);
    println!("searching");

    let mut min_dist = 999999999;

    for i in 0..map.len() {
        for j in 0..map.len() {
            if let Square::Cross(dist) = map[i][j] {
                if dist < min_dist {
                    min_dist = dist;
                }
            }
        }
    }

    min_dist
}

#[cfg(test)]
mod tests {
    use super::Segment::*;
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(
            6,
            part1(vec![R(8), U(5), L(5), D(3)], vec![U(7), R(6), D(4), L(4)])
        );

        assert_eq!(
            159,
            part1(
                vec![R(75), D(30), R(83), U(83), L(12), D(49), R(71), U(7), L(72)],
                vec![U(62), R(66), U(55), R(34), D(71), R(55), D(58), R(83)]
            )
        );

        assert_eq!(
            135,
            part1(
                vec![
                    R(98),
                    U(47),
                    R(26),
                    D(63),
                    R(33),
                    U(87),
                    L(62),
                    D(20),
                    R(33),
                    U(53),
                    R(51)
                ],
                vec![
                    U(98),
                    R(91),
                    D(20),
                    R(16),
                    D(67),
                    R(40),
                    U(7),
                    R(15),
                    U(6),
                    R(7)
                ]
            )
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            30,
            part2(vec![R(8), U(5), L(5), D(3)], vec![U(7), R(6), D(4), L(4)])
        );

        assert_eq!(
            610,
            part2(
                vec![R(75), D(30), R(83), U(83), L(12), D(49), R(71), U(7), L(72)],
                vec![U(62), R(66), U(55), R(34), D(71), R(55), D(58), R(83)]
            )
        );

        assert_eq!(
            410,
            part2(
                vec![
                    R(98),
                    U(47),
                    R(26),
                    D(63),
                    R(33),
                    U(87),
                    L(62),
                    D(20),
                    R(33),
                    U(53),
                    R(51)
                ],
                vec![
                    U(98),
                    R(91),
                    D(20),
                    R(16),
                    D(67),
                    R(40),
                    U(7),
                    R(15),
                    U(6),
                    R(7)
                ]
            )
        );
    }
}
