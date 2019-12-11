use std::collections::HashSet;
use std::fs;
use std::iter::FromIterator;

pub fn run_part1() -> usize {
    part1(&input(&read_file())).0
}

pub fn run_part2() -> usize {
    part2(&input(&read_file()))
}

type Map = Vec<Vec<bool>>;

#[derive(PartialEq, Eq, Clone, Hash, Debug)]
struct Point {
    row: usize,
    col: usize,
}

impl Point {
    fn slope(self: &Self, to: &Self) -> i32 {
        const PRECISION: i32 = 1_000_000;

        let to_deg = |rad| (rad * 180.0 * PRECISION as f64 / std::f64::consts::PI) as i32;
        let rotate = |deg| deg - 90 * PRECISION;
        let normalize = |deg| (deg + 360 * PRECISION) % (360 * PRECISION);

        let slope_rad = (to.row as f64 - self.row as f64).atan2(to.col as f64 - self.col as f64);
        normalize(rotate(to_deg(slope_rad)))
    }

    fn distance(self: &Self, to: &Self) -> f64 {
        (((to.row as i32 - self.row as i32).pow(2) + (to.col as i32 - self.col as i32).pow(2))
            as f64)
            .sqrt()
    }
}

fn read_file() -> String {
    fs::read_to_string("data/day10.txt").expect("Error reading the file")
}

fn input(data: &String) -> Map {
    data.trim()
        .lines()
        .map(|line| line.to_owned().chars().map(|c| c == '#').collect())
        .collect()
}

fn scan_asteroids(map: &Map, relative_to: &Point) -> HashSet<Point> {
    let mut result = HashSet::new();

    let cols = map.len();
    let rows = map[0].len();

    for row in 0..rows {
        for col in 0..cols {
            if map[row][col] {
                let point = Point { row, col };
                if point == *relative_to {
                    continue;
                }
                result.insert(point);
            }
        }
    }

    result
}

fn find_best_asteroid(map: &Map) -> (usize, Point) {
    let cols = map.len();
    let rows = map[0].len();
    let mut best_can_see = 0;
    let mut best = Point { row: 0, col: 0 };

    for row in 0..rows {
        for col in 0..cols {
            if !map[row][col] {
                continue;
            }
            let from = Point { row, col };
            let scan_results = scan_asteroids(map, &from);
            let can_see =
                HashSet::<i32>::from_iter(scan_results.iter().map(|a| a.slope(&from))).len();
            if can_see > best_can_see {
                best_can_see = can_see;
                best = from;
            }
        }
    }

    (best_can_see, best)
}

fn part1(map: &Map) -> (usize, Point) {
    find_best_asteroid(map)
}

fn part2(map: &Map) -> usize {
    let (_, best) = find_best_asteroid(map);
    let mut scan_results = scan_asteroids(map, &best);
    let mut cnt = 0;
    loop {
        let mut vect = Vec::from_iter(scan_results.clone());
        vect.sort_by(|a, b| {
            let res = a.slope(&best).partial_cmp(&b.slope(&best)).unwrap();
            if res == std::cmp::Ordering::Equal {
                return a.distance(&best).partial_cmp(&b.distance(&best)).unwrap();
            }
            res
        });

        let mut i = 0;
        let mut remaining = HashSet::new();
        loop {
            let el = &vect[i];
            cnt += 1;
            if cnt == 200 {
                return el.col * 100 + el.row;
            }
            i += 1;
            while i < vect.len() && vect[i].slope(&best) == el.slope(&best) {
                remaining.insert(vect[i].clone());
                i += 1;
            }
            if i == vect.len() {
                break;
            }
        }
        scan_results = remaining;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(
            (8, Point { col: 3, row: 4 }),
            part1(&input(
                &"
.#..#
.....
#####
....#
...##
"
                .to_owned()
            ))
        );

        assert_eq!(
            (33, Point { col: 5, row: 8 }),
            part1(&input(
                &"
......#.#.
#..#.#....
..#######.
.#.#.###..
.#..#.....
..#....#.#
#..#....#.
.##.#..###
##...#..#.
.#....####
"
                .to_owned()
            ))
        );

        assert_eq!(
            (35, Point { col: 1, row: 2 }),
            part1(&input(
                &"
#.#...#.#.
.###....#.
.#....#...
##.#.#.#.#
....#.#.#.
.##..###.#
..#...##..
..##....##
......#...
.####.###.
"
                .to_owned()
            ))
        );

        assert_eq!(
            (41, Point { col: 6, row: 3 }),
            part1(&input(
                &"
.#..#..###
####.###.#
....###.#.
..###.##.#
##.##.#.#.
....###..#
..#.#..#.#
#..#.#.###
.##...##.#
.....#.#..
"
                .to_owned()
            ))
        );

        assert_eq!(
            (210, Point { col: 11, row: 13 }),
            part1(&input(
                &"
.#..##.###...#######
##.############..##.
.#.######.########.#
.###.#######.####.#.
#####.##.#.##.###.##
..#####..#.#########
####################
#.####....###.#.#.##
##.#################
#####.##.###..####..
..######..##.#######
####.##.####...##..#
.#####..#.######.###
##...#.##########...
#.##########.#######
.####.#.###.###.#.##
....##.##.###..#####
.#.#.###########.###
#.#.#.#####.####.###
###.##.####.##.#..##
"
                .to_owned()
            ))
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            802,
            part2(&input(
                &"
.#..##.###...#######
##.############..##.
.#.######.########.#
.###.#######.####.#.
#####.##.#.##.###.##
..#####..#.#########
####################
#.####....###.#.#.##
##.#################
#####.##.###..####..
..######..##.#######
####.##.####...##..#
.#####..#.######.###
##...#.##########...
#.##########.#######
.####.#.###.###.#.##
....##.##.###..#####
.#.#.###########.###
#.#.#.#####.####.###
###.##.####.##.#..##
"
                .to_owned()
            ))
        );
    }
}
