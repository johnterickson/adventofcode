use std::ops::RangeInclusive;

use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day4)]
fn parse_input(input: &str) -> Vec<(RangeInclusive<u32>, RangeInclusive<u32>)> {
    input
        .trim()
        .lines()
        .map(|l| {
            let parse_range = |s: &str| {
                let mut tokens = s.trim().split('-');
                let start = tokens.next().unwrap().parse().unwrap();
                let end = tokens.next().unwrap().parse().unwrap();
                start..=end
            };
            let mut ranges = l.trim().split(',');
            let first = parse_range(ranges.next().unwrap());
            let second = parse_range(ranges.next().unwrap());
            (first, second)
        })
        .collect()
}

#[aoc(day4, part1)]
fn part1(pairs: &[(RangeInclusive<u32>, RangeInclusive<u32>)]) -> usize {
    pairs
        .iter()
        .filter(|(left, right)| {
               (left.contains(right.start()) && left.contains(right.end()))
            || (right.contains(left.start()) && right.contains(left.end()))
        })
        .count()
}

#[aoc(day4, part2)]
fn part2(pairs: &[(RangeInclusive<u32>, RangeInclusive<u32>)]) -> usize {
    pairs
        .iter()
        .filter(|(left, right)| {
               (left.contains(right.start()) || left.contains(right.end()))
            || (right.contains(left.start()) || right.contains(left.end()))
        })
        .count()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let input = parse_input(r#"
        2-4,6-8
        2-3,4-5
        5-7,7-9
        2-8,3-7
        6-6,4-6
        2-6,4-8
        "#.trim());
        assert_eq!(part1(&input), 2);
    }

    #[test]
    fn part2_example() {
        let input = parse_input(r#"
        2-4,6-8
        2-3,4-5
        5-7,7-9
        2-8,3-7
        6-6,4-6
        2-6,4-8
        "#.trim());
        assert_eq!(part2(&input), 4);
    }
}