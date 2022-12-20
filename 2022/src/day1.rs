use std::collections::BTreeSet;

use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day1)]
fn parse_input(input: &str) -> Vec<u32> {
    let mut sum = 0;
    let mut sums = Vec::new();
    for line in input.lines().map(|l| l.parse::<u32>().ok()) {
        if let Some(line) = line {
            sum += line;
        } else {
            sums.push(sum);
            sum = 0;
        }
    }
    sums.push(sum);
    sums
}

#[aoc(day1, part1)]
fn part1(sums: &[u32]) -> u32 {
    *sums.iter().max().unwrap()
}

#[aoc(day1, part2)]
fn part2(sums: &[u32]) -> u32 {
    let mut sums_set = BTreeSet::new();
    for sum in sums {
        sums_set.insert(sum);
        if sums_set.len() > 3 {
            let _ = sums_set.pop_first();
        }
    }
    sums_set.iter().cloned().sum()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let input = parse_input(r#"
        1000
2000
3000

4000

5000
6000

7000
8000
9000

10000"#.trim());
        assert_eq!(part1(&input), 24000);
    }

    #[test]
    fn part2_example() {
        let input = parse_input(r#"
        1000
2000
3000

4000

5000
6000

7000
8000
9000

10000"#.trim());
        assert_eq!(part2(&input), 45000);
    }
}