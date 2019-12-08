use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;

pub fn run_part1() -> u32 {
    part1(input(&read_file()))
}

pub fn run_part2() -> u32 {
    part2(input(&read_file()))
}

type Pair<'a> = (&'a str, &'a str);
type Input<'a> = Vec<Pair<'a>>;
type Graph<'a, 'b> = HashMap<&'a str, Vec<&'b str>>;

fn read_file() -> String {
    fs::read_to_string("data/day6.txt").expect("Error reading the file")
}

fn input<'a>(data: &'a String) -> Input<'a> {
    data.trim().lines().map(split).collect()
}

fn split(s: &str) -> Pair {
    let parts = s.split(")").collect::<Vec<_>>();
    (parts[0], parts[1])
}

fn build_graph(input: Input, bi_directional: bool) -> Graph {
    let mut graph = HashMap::new();

    for line in input {
        graph.entry(line.0).or_insert(Vec::new()).push(line.1);

        if bi_directional {
            graph.entry(line.1).or_insert(Vec::new()).push(line.0);
        }
    }

    graph
}

fn part1(input: Input) -> u32 {
    let graph = build_graph(input, false);
    part1_search(&graph, "COM", 0)
}

fn part2(input: Input) -> u32 {
    let graph = build_graph(input, true);
    part2_search(&graph, "YOU", &mut HashSet::new(), 0) - 1
}

fn part1_search(graph: &Graph, item: &str, dist: u32) -> u32 {
    let mut sum = dist;

    if let Some(children) = graph.get(&item) {
        for v in children {
            sum += part1_search(graph, v, dist + 1);
        }
    }

    sum
}

fn part2_search<'a>(
    graph: &'a Graph,
    item: &'a str,
    visited: &mut HashSet<&'a str>,
    dist: u32,
) -> u32 {
    let mut santa_dist = 0;
    visited.insert(item);

    if let Some(children) = graph.get(&item) {
        for v in children {
            if v == &"SAN" {
                return dist;
            }
            if !visited.contains(v) {
                santa_dist += part2_search(graph, v.clone(), visited, dist + 1);
            }
        }
    }

    santa_dist
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(
            42,
            part1(vec![
                split("COM)B"),
                split("B)C"),
                split("C)D"),
                split("D)E"),
                split("E)F"),
                split("B)G"),
                split("G)H"),
                split("D)I"),
                split("E)J"),
                split("J)K"),
                split("K)L"),
            ])
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            4,
            part2(vec![
                split("COM)B"),
                split("B)C"),
                split("C)D"),
                split("D)E"),
                split("E)F"),
                split("B)G"),
                split("G)H"),
                split("D)I"),
                split("E)J"),
                split("J)K"),
                split("K)L"),
                split("K)YOU"),
                split("I)SAN"),
            ])
        );
    }
}
