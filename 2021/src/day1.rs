use std::num::ParseIntError;

use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day1)]
fn parse_input(input: &str) -> Result<Vec<u32>, ParseIntError> {
    input.lines().map(|l| l.parse()).collect()
}

#[aoc(day1, part1)]
fn part1(depths: &[u32]) -> u32 {
    let mut prev = None;
    let mut count = 0;
    for d in depths {
        if let Some(prev) = prev {
            if prev < d {
                count += 1;
            }
        }
        prev = Some(d);
    }
    count
}

#[aoc(day1, part2)]
fn part2(depths: &[u32]) -> u32 {
    depths.windows(3)
        .zip(depths[1..].windows(3))
        .filter(|(w1,w2)| w1.iter().sum::<u32>() < w2.iter().sum())
        .count() as u32
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(part1(&[199,200,208,210,200,207,240,269,260,263]), 7);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part1(&[607,618,618,617,647,716,769,792]), 5);
    }
}