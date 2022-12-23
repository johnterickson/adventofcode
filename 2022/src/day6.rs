use std::collections::BTreeSet;

use aoc_runner_derive::{aoc, aoc_generator};

type Input = Vec<char>;

#[aoc_generator(day6)]
fn parse_input(input: &str) -> Input {
    input.trim().chars().collect()
}

#[aoc(day6, part1)]
fn part1(input: &Input) -> usize {
    let mut chars = BTreeSet::new();
    'next: for (i, w) in input.windows(4).enumerate() {
        chars.clear();
        for c in w {
            if !chars.insert(c) {
                continue 'next;
            }
        }

        return i + 4;
    }

    panic!();
}

#[aoc(day6, part2)]
fn part2(input: &Input) -> usize {
    let mut chars = BTreeSet::new();
    'next: for (i, w) in input.windows(14).enumerate() {
        chars.clear();
        for c in w {
            if !chars.insert(c) {
                continue 'next;
            }
        }

        return i + 14;
    }

    panic!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_examples() {
        let inputs = [
            (parse_input("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 7),
            (parse_input("bvwbjplbgvbhsrlpgdmjqwftvncz"), 5),
            (parse_input("nppdvjthqldpwncqszvftbrmjlhg"), 6),
            (parse_input("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 10),
            (parse_input("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 11),
        ];
        for input in &inputs {
            assert_eq!(part1(&input.0), input.1);
        }
    }

    #[test]
    fn part2_examples() {
        let inputs = [
            (parse_input("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 19),
            (parse_input("bvwbjplbgvbhsrlpgdmjqwftvncz"), 23),
            (parse_input("nppdvjthqldpwncqszvftbrmjlhg"), 23),
            (parse_input("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 29),
            (parse_input("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 26),
        ];
        for input in &inputs {
            assert_eq!(part2(&input.0), input.1);
        }
    }
}
