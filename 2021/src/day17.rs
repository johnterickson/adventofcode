use std::{ops::{RangeInclusive}, cmp, collections::BTreeSet};

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

    loop {
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
            // println!("HIT  vi={},{} p={},{} v={},{}", vi.0, vi.1, x, y, vx, vy);
            return Some(max_y);
        }

        if x > *target.0.end() && vx >= 0 {
            break;
        }

        if y < *target.1.start() && vy <= 0 {
            break;
        }
    }

    // println!("MISS vi={},{} p={},{} v={},{}", vi.0, vi.1, x, y, vx, vy);
    None
}

#[aoc(day17, part1)]
fn part1(target: &(RangeInclusive<i64>, RangeInclusive<i64>)) -> i64 { 

    let mut max_y = 0;

    let viy_max_abs = cmp::max(target.1.start().abs(), target.1.end().abs());

    for vix in 0..=*target.0.end() {
        for viy in -viy_max_abs..=viy_max_abs {
            if let Some(y) = simulate(target, (vix, viy)) {
                max_y = cmp::max(max_y, y);
            }
        }
    }
    
    max_y
}

fn part2_inner(target: &(RangeInclusive<i64>, RangeInclusive<i64>)) -> BTreeSet<(i64,i64)> { 
    let mut hits = BTreeSet::new();

    let viy_max_abs = cmp::max(target.1.start().abs(), target.1.end().abs());

    for vix in 0..=*target.0.end() {
        for viy in -viy_max_abs..=viy_max_abs {
            if let Some(_) = simulate(target, (vix, viy)) {
                hits.insert((vix,viy));
            }
        }
    }

    hits
}

#[aoc(day17, part2)]
fn part2(target: &(RangeInclusive<i64>, RangeInclusive<i64>)) -> usize { 
    part2_inner(target).len()
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

    #[test]
    fn part2_examples() {
        let expected = r#"23,-10  25,-9   27,-5   29,-6   22,-6   21,-7   9,0     27,-7   24,-5
        25,-7   26,-6   25,-5   6,8     11,-2   20,-5   29,-10  6,3     28,-7
        8,0     30,-6   29,-8   20,-10  6,7     6,4     6,1     14,-4   21,-6
        26,-10  7,-1    7,7     8,-1    21,-9   6,2     20,-7   30,-10  14,-3
        20,-8   13,-2   7,3     28,-8   29,-9   15,-3   22,-5   26,-8   25,-8
        25,-6   15,-4   9,-2    15,-2   12,-2   28,-9   12,-3   24,-6   23,-7
        25,-10  7,8     11,-3   26,-7   7,1     23,-9   6,0     22,-10  27,-6
        8,1     22,-8   13,-4   7,6     28,-6   11,-4   12,-4   26,-9   7,4
        24,-10  23,-8   30,-8   7,0     9,-1    10,-1   26,-5   22,-9   6,5
        7,5     23,-6   28,-10  10,-2   11,-1   20,-9   14,-2   29,-7   13,-3
        23,-5   24,-8   27,-9   30,-7   28,-5   21,-10  7,9     6,6     21,-5
        27,-10  7,2     30,-9   21,-8   22,-7   24,-9   20,-6   6,9     29,-5
        8,-2    27,-8   30,-5   24,-7"#;
        let expected: BTreeSet<(i64,i64)> = expected.split_ascii_whitespace()
            .filter_map(|pair| {
                let pair = pair.trim();
                if pair.len() > 0 {
                    Some(pair)
                } else {
                    None
                }
            })
            .map(|pair| {
                let mut tokens = pair.split(',');
                (tokens.next().unwrap().parse().unwrap(), tokens.next().unwrap().parse().unwrap())
            })
            .collect();
        let target = parse_input("target area: x=20..30, y=-10..-5");
        let found = part2_inner(&target);
        for diff in expected.symmetric_difference(&found) {
            dbg!(diff);
        }
        assert_eq!(expected, found);
    }
}