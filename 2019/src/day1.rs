use aoc_runner_derive::{aoc, aoc_generator};
use std::num::ParseIntError;

#[aoc_generator(day1)]
fn parse_input(input: &str) -> Result<Vec<u32>, ParseIntError> {
    input.lines().map(|l| l.parse()).collect()
}

fn get_fuel(mass: u32) -> u32 {
    if mass < 6 { 
        0
    } else {
        let mut fuel = (mass / 3) - 2;
        fuel += get_fuel(fuel);
        fuel
    }
}

#[aoc(day1, part1)]
fn part1(masses: &[u32]) -> u32 {
    masses.iter().map(|mass| (mass / 3) - 2).sum()
}

#[aoc(day1, part2)]
fn part2(masses: &[u32]) -> u32 {
    masses.iter().map(|m| get_fuel(*m)).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(part1(&[12]), 2);
        assert_eq!(part1(&[14]), 2);
        assert_eq!(part1(&[1969]), 654);
        assert_eq!(part1(&[100756]), 33583);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&[14]), 2);
        assert_eq!(part2(&[1969]), 966);
        assert_eq!(part2(&[100756]), 50346);
    }
}
