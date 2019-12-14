use std::fs;

pub fn run_part1() -> usize {
    part1(input(), 25, 6)
}

pub fn run_part2() -> String {
    "\n".to_owned()
        + &part2(input(), 25, 6)
            .chunks(25)
            .map(|line| {
                line.iter()
                    .map(|&c| if c == 1 { '▅' } else { ' ' })
                    .collect()
            })
            .collect::<Vec<String>>()
            .join("\n")
}

fn input() -> Vec<u32> {
    fs::read_to_string("data/day8.txt")
        .expect("Error reading the file")
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect()
}

fn part1(img: Vec<u32>, width: usize, height: usize) -> usize {
    let count = |xs: &[u32], c: u32| xs.iter().filter(|&e| *e == c).count();
    let r = img
        .chunks(width * height)
        .min_by_key(|w| count(w, 0))
        .unwrap();
    count(r, 1) * count(r, 2)
}

fn part2(img: Vec<u32>, width: usize, height: usize) -> Vec<u32> {
    let chunk_size = width * height;
    img.chunks(chunk_size)
        .fold(vec![2; chunk_size], |mut acc, layer| {
            for i in 0..chunk_size {
                if acc[i] == 2 {
                    acc[i] = layer[i];
                }
            }
            acc
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(1, part1(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 1, 2], 3, 2));
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            vec![0, 1, 1, 0],
            part2(vec![0, 2, 2, 2, 1, 1, 2, 2, 2, 2, 1, 2, 0, 0, 0, 0], 2, 2)
        );
    }
}
