use aoc_runner_derive::{aoc, aoc_generator};
use std::num::ParseIntError;

use std::collections::BTreeSet;

#[aoc_generator(day10)]
fn parse_input(input: &str) -> Result<BTreeSet<(isize,isize)>, ParseIntError> {
    Ok(input.lines().enumerate().map(|(y, line)| {
        line.trim().chars().enumerate().filter(|(x,pixel)| *pixel == '#').map(move |(x, _)| (x as isize, y as isize))
    }).flatten().collect())
}

fn count_visible(asteroid: (isize,isize), asteroids: &BTreeSet<(isize,isize)>) -> usize {
    let mut blocked = BTreeSet::new();
    let (home_x, home_y) = asteroid;
    let (max_x, max_y) = (
        *asteroids.iter().map(|(x,_y)| x).max().unwrap(), 
        *asteroids.iter().map(|(_x,y)| y).max().unwrap()
    );
    for a in asteroids {
        if *a == asteroid {
            continue;
        }

        let (blocker_x, blocker_y) = a;
        let (dx, dy) = (blocker_x - home_x, blocker_y - home_y);
        let mut multiple = 2;
        loop {
            let x = home_x + multiple*dx;
            let y = home_y + multiple*dy;
            if x < 0 || y < 0 || x > max_x || y > max_y {
                break;
            }

            if asteroids.contains(&(x,y)) {
                blocked.insert((x,y));
            }

            multiple += 1;
        }
    }

    asteroids.len() - blocked.len() - 1
}

fn find_best(cells: &BTreeSet<(isize,isize)>) -> ((usize,usize), usize) {
    unimplemented!();
}

#[aoc(day10, part1)]
fn part1(input: &BTreeSet<(isize,isize)>) -> u32 {
    unimplemented!();
}

#[aoc(day10, part2)]
fn part2(input: &BTreeSet<(isize,isize)>) -> u32 {
    unimplemented!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let input = parse_input(
       ".#..#
        .....
        #####
        ....#
        ...##"
        ).unwrap();
        assert_eq!(8, count_visible((3,4), &input));
    }

    #[test]
    fn part2_example() {
    }
}
