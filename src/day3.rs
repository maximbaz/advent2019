use std::collections::HashMap;
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

impl Segment {
    fn n(&self) -> i32 {
        match *self {
            Segment::U(n) => n,
            Segment::D(n) => n,
            Segment::L(n) => n,
            Segment::R(n) => n,
        }
    }

    fn x_offset(&self) -> i32 {
        match *self {
            Segment::U(_) => -1,
            Segment::D(_) => 1,
            _ => 0,
        }
    }

    fn y_offset(&self) -> i32 {
        match *self {
            Segment::L(_) => -1,
            Segment::R(_) => 1,
            _ => 0,
        }
    }
}

enum Square {
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

fn build_map(line1: Line, line2: Line) -> HashMap<(i32, i32), Square> {
    let mut map = HashMap::new();

    let mut x = 0;
    let mut y = 0;
    let mut step = 0;
    for segment in line1 {
        for _i in 0..segment.n() {
            x += segment.x_offset();
            y += segment.y_offset();
            step += 1;
            map.insert((x, y), Square::Line1(step));
        }
    }

    x = 0;
    y = 0;
    step = 0;

    for segment in line2 {
        for _i in 0..segment.n() {
            x += segment.x_offset();
            y += segment.y_offset();
            step += 1;
            map.insert(
                (x, y),
                match map.get(&(x, y)) {
                    Some(Square::Line1(n)) => Square::Cross(n + step),
                    _ => Square::Line2,
                },
            );
        }
    }

    map
}

fn part1(line1: Line, line2: Line) -> i32 {
    let map = build_map(line1, line2);

    let mut min_dist = i32::max_value();

    for ((i, j), segment) in map {
        if let Square::Cross(_) = segment {
            let dist = i.abs() + j.abs();
            if dist < min_dist {
                min_dist = dist;
            }
        }
    }

    min_dist
}

fn part2(line1: Line, line2: Line) -> i32 {
    let map = build_map(line1, line2);

    let mut min_dist = i32::max_value();

    for (_, segment) in map {
        if let Square::Cross(dist) = segment {
            if dist < min_dist {
                min_dist = dist;
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
