use std::ops::{RangeInclusive};

use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day7)]
fn parse_input(input: &str) -> Vec<i64> {
    let line = input.lines().next().unwrap();
    line.trim().split(',').map(|n| i64::from_str_radix(n.trim(), 10).unwrap()).collect()
}


#[aoc(day7, part1)]
fn part1(positions: &Vec<i64>) -> i64 { 
    let mut positions = positions.clone();
    positions.sort();
    let target = positions[positions.len()/2];

    positions.iter().map(|p| (p-target).abs()).sum()
}

fn sum_of_integers(r:RangeInclusive<i64>) -> i64 {
    (r.end() - r.start() + 1)*(r.start() + r.end())/2
}

fn fuel(a: i64, b:i64) -> i64 {
    if a > b {
        fuel(b, a)
    } else {
        assert!(b>=a);
        sum_of_integers(0..=(b-a))
    }
}

#[aoc(day7, part2)]
fn part2(positions: &Vec<i64>) -> i64 {
    let min = *positions.iter().min().unwrap();
    let max = *positions.iter().max().unwrap();

    let mut min_fuel = i64::MAX;
    for i in min..=max {
        let fuel: i64 = positions.iter().map(|p| fuel(i,*p)).sum();
        // dbg!(i,&fuel);
        if fuel < min_fuel {
            min_fuel = fuel;
        }
    }

    min_fuel
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn helpers() {
        assert_eq!(sum_of_integers(1..=3), 1 + 2 + 3); 
        assert_eq!(sum_of_integers(1..=4), 1 + 2 + 3 + 4); 
        assert_eq!(sum_of_integers(2..=4), 2 + 3 + 4); 
        assert_eq!(fuel(5,16), 66); 
        assert_eq!(fuel(5,1), 10); 
    }

    #[test]
    fn part1_example() {
        let input = r#"16,1,2,0,4,2,7,1,2,14"#;
        let input = parse_input(input);
        assert_eq!(part1(&input), 37);
    }

    #[test]
    fn part2_example() {
        let input = r#"16,1,2,0,4,2,7,1,2,14"#;
        let input = parse_input(input);
        assert_eq!(part2(&input), 168);
    }
}