use std::{ops::{RangeInclusive}, cmp};

use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day17)]
fn parse_input(input: &str) -> (RangeInclusive<i64>, RangeInclusive<i64>) {
    let tokens: Vec<_> = input.trim().split(&['=','.',','][..]).collect();
    (
        tokens[1].parse().unwrap()..=tokens[3].parse().unwrap(),
        tokens[5].parse().unwrap()..=tokens[7].parse().unwrap()
    )
}

fn simulate(target: &(RangeInclusive<i64>, RangeInclusive<i64>), vi: (i64,i64)) -> Option<i64> {
    let mut x = 0;
    let mut y = 0;
    let mut vx = vi.0;
    let mut vy = vi.1;

    let mut max_y = 0;

    while x <= *target.0.end() && y >= *target.1.end() {
        x += vx;
        y += vy;
        match vx.cmp(&0) {
            cmp::Ordering::Less => vx += 1,
            cmp::Ordering::Greater => vx -= 1,
            cmp::Ordering::Equal => {},
        }
        vy -= 1;

        max_y = cmp::max(max_y, y);

        if target.0.contains(&x) && target.1.contains(&y) {
            return Some(max_y);
        }
    }

    None
}

#[aoc(day17, part1)]
fn part1(target: &(RangeInclusive<i64>, RangeInclusive<i64>)) -> i64 { 

    let mut max_y = 0;

    for vix in 0..=*target.0.end() {
        for viy in *target.1.start()..=target.1.start().abs() {
            if let Some(y) = simulate(target, (vix, viy)) {
                max_y = cmp::max(max_y, y);
            }
        }
    }
    
    max_y
}

#[aoc(day17, part2)]
fn part2(bits: &(RangeInclusive<i64>, RangeInclusive<i64>)) -> u64 { 
    todo!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_examples() {
        let target = parse_input("target area: x=20..30, y=-10..-5");
        assert_eq!(Some(3), simulate(&target, (7,2)));
        assert_eq!(Some(6), simulate(&target, (6,3)));
        assert_eq!(Some(0), simulate(&target, (9,0)));
        assert_eq!(None, simulate(&target, (17,-4)));

        assert_eq!(45, part1(&target));
    }
}