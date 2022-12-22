use std::{collections::HashSet};

use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

fn priority(c: char) -> u32 {
    (match c {
        'a'..='z' => c as u8 - 'a' as u8 + 1,
        'A'..='Z' => c as u8 - 'A' as u8 + 27,
        _ => panic!("unexpected {}", c)
    }) as u32
}

#[aoc_generator(day3)]
fn parse_input(input: &str) -> Vec<(HashSet<char>, HashSet<char>)> {
    input
        .trim()
        .lines()
        .map(|l| {
            let chars: Vec<_> = l.trim().chars().collect();
            let chars = chars.split_at(chars.len()/2);
            (chars.0.iter().cloned().collect(), chars.1.iter().cloned().collect())
        })
        .collect()
}

#[aoc(day3, part1)]
fn part1(pairs: &[(HashSet<char>, HashSet<char>)]) -> u32 {
    pairs
        .iter()
        .map(|(left, right)| {
            let common = *left.intersection(right).next().unwrap();
            let priority = priority(common);
            // dbg!(left, right, common, priority);
            priority
        })
        .sum()
}

#[aoc(day3, part2)]
fn part2(pairs: &[(HashSet<char>, HashSet<char>)]) -> u32 {
    pairs.iter()
        .map(|(l,r)| l | r)
        .chunks(3)
        .into_iter()
        .map(|mut group| {
            let common = group.next().unwrap();
            let common = &common & &group.next().unwrap();
            let common = &common & &group.next().unwrap();
            priority(*common.iter().next().unwrap())
        })
        .sum()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let input = parse_input(r#"
        vJrwpWtwJgWrhcsFMMfFFhFp
        jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
        PmmdzqPrVvPwwTWBwg
        wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
        ttgJtRGJQctTZtZT
        CrZsJsPPZsGzwwsLwLmpwMDw
        "#.trim());
        assert_eq!(part1(&input), 157);
    }

    #[test]
    fn part2_example() {
        let input = parse_input(r#"
        vJrwpWtwJgWrhcsFMMfFFhFp
        jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
        PmmdzqPrVvPwwTWBwg
        wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
        ttgJtRGJQctTZtZT
        CrZsJsPPZsGzwwsLwLmpwMDw
        "#.trim());
        assert_eq!(part2(&input), 70);
    }
}