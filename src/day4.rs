use itertools::izip;

pub fn run_part1() -> usize {
    part1(137683, 596253)
}

pub fn run_part2() -> usize {
    part2(137683, 596253)
}

fn num_to_digits(mut n: i32) -> Vec<i8> {
    let mut digits = Vec::new();
    while n > 9 {
        digits.push((n % 10) as i8);
        n = n / 10;
    }
    digits.push(n as i8);
    digits.reverse();
    digits
}

fn has_any_two_adjacent_digits(num: Vec<i8>) -> bool {
    num.iter().zip(num.iter().skip(1)).any(|(a, b)| a == b)
}

fn has_only_two_adjacent_digits(mut num: Vec<i8>) -> bool {
    num.insert(0, -1);
    num.push(-1);
    izip!(
        num.iter(),
        num.iter().skip(1),
        num.iter().skip(2),
        num.iter().skip(3)
    )
    .any(|(a, b, c, d)| a != b && b == c && c != d)
}

fn digits_never_decrease(num: Vec<i8>) -> bool {
    num.iter().zip(num.iter().skip(1)).all(|(a, b)| a <= b)
}

fn part1(from: i32, to: i32) -> usize {
    (from..=to)
        .map(num_to_digits)
        .filter(|n| has_any_two_adjacent_digits(n.clone()) && digits_never_decrease(n.clone()))
        .count()
}

fn part2(from: i32, to: i32) -> usize {
    (from..=to)
        .map(num_to_digits)
        .filter(|n| has_only_two_adjacent_digits(n.clone()) && digits_never_decrease(n.clone()))
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_num_to_digits() {
        assert_eq!(vec![1, 2, 3], num_to_digits(123));
        assert_eq!(vec![1, 0, 0, 2, 0, 0], num_to_digits(100200));
        assert_eq!(vec![0], num_to_digits(0));
    }

    #[test]
    fn test_has_two_adjacent_digits() {
        assert_eq!(true, has_any_two_adjacent_digits(vec![1, 2, 2, 3, 4]));
        assert_eq!(true, has_any_two_adjacent_digits(vec![2, 2, 3, 4, 5]));
        assert_eq!(true, has_any_two_adjacent_digits(vec![1, 1]));
        assert_eq!(false, has_any_two_adjacent_digits(vec![1]));
        assert_eq!(false, has_any_two_adjacent_digits(vec![1, 2, 3, 4, 5]));
    }

    #[test]
    fn test_digits_never_decrease() {
        assert_eq!(true, digits_never_decrease(vec![1, 2, 3, 4]));
        assert_eq!(true, digits_never_decrease(vec![1, 2, 2, 3]));
        assert_eq!(true, digits_never_decrease(vec![1, 1]));
        assert_eq!(false, digits_never_decrease(vec![1, 3, 3, 2, 4]));
    }

    #[test]
    fn test_has_only_two_adjacent_digits() {
        assert_eq!(true, has_only_two_adjacent_digits(vec![1, 2, 2, 3, 4]));
        assert_eq!(true, has_only_two_adjacent_digits(vec![1, 1, 2, 2, 3, 3]));
        assert_eq!(true, has_only_two_adjacent_digits(vec![1, 1, 1, 1, 2, 2]));
        assert_eq!(true, has_only_two_adjacent_digits(vec![2, 2, 1, 1, 1, 1]));
        assert_eq!(
            true,
            has_only_two_adjacent_digits(vec![1, 1, 1, 2, 2, 1, 1, 1])
        );
        assert_eq!(false, has_only_two_adjacent_digits(vec![1]));
        assert_eq!(false, has_only_two_adjacent_digits(vec![1, 2, 3, 4, 5]));
        assert_eq!(false, has_only_two_adjacent_digits(vec![1, 2, 3, 4, 5]));
        assert_eq!(false, has_only_two_adjacent_digits(vec![1, 2, 3, 4, 4, 4]));
    }
}
