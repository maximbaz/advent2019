pub fn run_part1() -> usize {
    part1(137683, 596253)
}

pub fn run_part2() -> usize {
    part2(137683, 596253)
}

fn num_to_digits(n: i32) -> Vec<char> {
    n.to_string().chars().collect()
}

fn has_any_two_adjacent_digits(num: Vec<char>) -> bool {
    num.windows(2).any(|c| c[0] == c[1])
}

fn has_only_two_adjacent_digits(mut num: Vec<char>) -> bool {
    num.insert(0, '_');
    num.push('_');
    num.windows(4)
        .any(|c| c[0] != c[1] && c[1] == c[2] && c[2] != c[3])
}

fn digits_never_decrease(num: Vec<char>) -> bool {
    num.windows(2).all(|c| c[0] <= c[1])
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
    fn test_has_two_adjacent_digits() {
        assert_eq!(true, has_any_two_adjacent_digits(num_to_digits(12234)));
        assert_eq!(true, has_any_two_adjacent_digits(num_to_digits(22345)));
        assert_eq!(true, has_any_two_adjacent_digits(num_to_digits(11)));
        assert_eq!(false, has_any_two_adjacent_digits(num_to_digits(1)));
        assert_eq!(false, has_any_two_adjacent_digits(num_to_digits(12345)));
    }

    #[test]
    fn test_digits_never_decrease() {
        assert_eq!(true, digits_never_decrease(num_to_digits(1234)));
        assert_eq!(true, digits_never_decrease(num_to_digits(1223)));
        assert_eq!(true, digits_never_decrease(num_to_digits(11)));
        assert_eq!(false, digits_never_decrease(num_to_digits(13324)));
    }

    #[test]
    fn test_has_only_two_adjacent_digits() {
        assert_eq!(true, has_only_two_adjacent_digits(num_to_digits(12234)));
        assert_eq!(true, has_only_two_adjacent_digits(num_to_digits(112233)));
        assert_eq!(true, has_only_two_adjacent_digits(num_to_digits(111122)));
        assert_eq!(true, has_only_two_adjacent_digits(num_to_digits(221111)));
        assert_eq!(true, has_only_two_adjacent_digits(num_to_digits(11122111)));
        assert_eq!(false, has_only_two_adjacent_digits(num_to_digits(1)));
        assert_eq!(false, has_only_two_adjacent_digits(num_to_digits(12345)));
        assert_eq!(false, has_only_two_adjacent_digits(num_to_digits(12345)));
        assert_eq!(false, has_only_two_adjacent_digits(num_to_digits(123444)));
    }
}
