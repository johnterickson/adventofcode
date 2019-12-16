use aoc_runner_derive::{aoc, aoc_generator};
use std::num::ParseIntError;

#[aoc_generator(day4)]
fn parse_input(input: &str) -> Result<(usize,usize), ParseIntError> {
    let mut nums = input.split("-");
    let lo = nums.next().unwrap().parse()?;
    let hi = nums.next().unwrap().parse()?;
    Ok((lo, hi))
}

fn check_number(n: usize) -> bool {
    let digits = {
        let mut digits = Vec::new();
        let mut n = n;
        for _ in 0..=5 {
            digits.push(n % 10);
            n /= 10;
        }
        digits.reverse();
        digits
    };

    let mut last_digit = digits[0];
    let mut double_digit = false;
    for d in digits.iter().skip(1) {
        if d < &last_digit {
            return false;
        }
        if &last_digit == d {
            double_digit = true;
        }

        last_digit = *d;
    }

    return double_digit;
}

#[aoc(day4, part1)]
fn part1(input: &(usize, usize)) -> usize {
    let (lo, hi) = *input;
    (lo..=hi).filter(|n| check_number(*n)).count()
}

fn check_number2(n: usize) -> bool {
    let digits = {
        let mut digits = Vec::new();
        let mut n = n;
        for _ in 0..=5 {
            digits.push(n % 10);
            n /= 10;
        }
        digits.reverse();
        digits
    };

    let mut last_digit = digits[0];
    let mut run_length = 1;
    let mut double_digit = false;
    for d in digits.iter().skip(1) {
        if d < &last_digit {
            return false;
        }
        if &last_digit != d {
            if run_length == 2 {
                double_digit = true;
            }
            run_length = 0;
        }

        run_length += 1;
        last_digit = *d;
    }

    if run_length == 2 {
        double_digit = true;
    }

    return double_digit;
}

#[aoc(day4, part2)]
fn part2(input: &(usize, usize)) -> usize {
    let (lo, hi) = *input;
    (lo..=hi).filter(|n| check_number2(*n)).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        {
            let input = parse_input("111111-111111").unwrap();
            assert_eq!(1, part1(&input));
        }
        {
            let input = parse_input("223450-223450").unwrap();
            assert_eq!(0, part1(&input));
        }
        {
            let input = parse_input("123789-123789").unwrap();
            assert_eq!(0, part1(&input));
        }
    }

    #[test]
    fn part2_example() {
        {
            let input = parse_input("112233-112233").unwrap();
            assert_eq!(1, part2(&input));
        }
        {
            let input = parse_input("123444-123444").unwrap();
            assert_eq!(0, part2(&input));
        }
        {
            let input = parse_input("111122-111122").unwrap();
            assert_eq!(1, part2(&input));
        }
    }
}
