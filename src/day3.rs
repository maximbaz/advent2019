use std::collections::HashSet;
use std::fs;
use std::hash::{Hash, Hasher};

pub fn run_part1() -> i32 {
    let (line1, line2) = input();
    part1(line1, line2)
}

pub fn run_part2() -> i32 {
    let (line1, line2) = input();
    part2(line1, line2)
}

#[derive(Clone, PartialEq, Debug)]
enum Segment {
    U(i32),
    D(i32),
    L(i32),
    R(i32),
}

impl Segment {
    fn size(&self) -> i32 {
        match *self {
            Segment::U(n) | Segment::D(n) | Segment::L(n) | Segment::R(n) => n,
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

struct Point {
    x: i32,
    y: i32,
    distance: i32,
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Eq for Point {}

impl Hash for Point {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
    }
}

impl Point {
    fn manhattan(&self) -> i32 {
        self.x.abs() + self.y.abs()
    }

    fn sum_distances(&self, line1: &Line, line2: &Line) -> i32 {
        line1.get(self).unwrap().distance + line2.get(self).unwrap().distance
    }
}

type Segments = Vec<Segment>;
type Line = HashSet<Point>;

fn input() -> (Segments, Segments) {
    let lines: Vec<Vec<Segment>> = fs::read_to_string("data/day3.txt")
        .expect("Error reading the file")
        .trim()
        .lines()
        .map(parse_segments)
        .collect();

    (lines[0].clone(), lines[1].clone())
}

fn parse_segments(string: &str) -> Segments {
    string
        .split(",")
        .map(|s| match s.chars().next().unwrap() {
            'U' => Segment::U(s[1..].parse().unwrap()),
            'D' => Segment::D(s[1..].parse().unwrap()),
            'L' => Segment::L(s[1..].parse().unwrap()),
            'R' => Segment::R(s[1..].parse().unwrap()),
            _ => unreachable!(),
        })
        .collect()
}

fn draw_line(segments: Segments) -> Line {
    let mut points = HashSet::new();
    let mut x = 0;
    let mut y = 0;
    let mut distance = 0;
    for segment in segments {
        for _i in 0..segment.size() {
            x += segment.x_offset();
            y += segment.y_offset();
            distance += 1;
            points.insert(Point { x, y, distance });
        }
    }

    points
}

fn part1(segments1: Segments, segments2: Segments) -> i32 {
    draw_line(segments1)
        .intersection(&draw_line(segments2))
        .min_by_key(|p| p.manhattan())
        .unwrap()
        .manhattan()
}

fn part2(segments1: Segments, segments2: Segments) -> i32 {
    let line1 = draw_line(segments1);
    let line2 = draw_line(segments2);

    line1
        .intersection(&line2)
        .min_by_key(|p| p.sum_distances(&line1, &line2))
        .unwrap()
        .sum_distances(&line1, &line2)
}

#[cfg(test)]
mod tests {
    use super::parse_segments as p;
    use super::Segment::*;
    use super::*;

    #[test]
    fn test_parse_segments() {
        assert_eq!(
            vec![R(75), D(30), U(83), L(12), D(49), R(71), U(7), L(72)],
            parse_segments("R75,D30,U83,L12,D49,R71,U7,L72")
        );
    }

    #[test]
    fn test_part1() {
        assert_eq!(6, part1(p("R8,U5,L5,D3"), p("U7,R6,D4,L4")));

        assert_eq!(
            159,
            part1(
                p("R75,D30,R83,U83,L12,D49,R71,U7,L72"),
                p("U62,R66,U55,R34,D71,R55,D58,R83")
            )
        );

        assert_eq!(
            135,
            part1(
                p("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51"),
                p("U98,R91,D20,R16,D67,R40,U7,R15,U6,R7")
            )
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(30, part2(p("R8,U5,L5,D3"), p("U7,R6,D4,L4")));

        assert_eq!(
            610,
            part2(
                p("R75,D30,R83,U83,L12,D49,R71,U7,L72"),
                p("U62,R66,U55,R34,D71,R55,D58,R83")
            )
        );

        assert_eq!(
            410,
            part2(
                p("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51"),
                p("U98,R91,D20,R16,D67,R40,U7,R15,U6,R7")
            )
        );
    }
}
