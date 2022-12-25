use std::collections::BTreeSet;

use aoc_runner_derive::{aoc, aoc_generator};

type Input = Vec<Vec<usize>>;

#[aoc_generator(day8)]
fn parse_input(input: &str) -> Input {
    input.trim()
        .lines()
        .map(|line| line.trim().chars().map(|c| (c as u8 - '0' as u8) as usize).collect())
        .collect()
}

#[aoc(day8, part1)]
fn part1(grid: &Input) -> usize {
    let rows = grid.len();
    let cols = grid[0].len();

    let mut count = 0;

    for r in 0..rows {
        for c in 0..cols {
            let height = grid[r][c];
            let top = (0..r).all(|rr| grid[rr][c] < height);
            let bottom = (r+1..rows).all(|rr| grid[rr][c] < height);
            let left = (0..c).all(|cc| grid[r][cc] < height);
            let right = (c+1..cols).all(|cc| grid[r][cc] < height);
            if top || bottom || left || right {
                dbg!(r,c,height, top, bottom, left, right);
                count += 1;
            }
        }
    }
    
    count
}

#[aoc(day8, part2)]
fn part2(input: &Input) -> usize {
    todo!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_examples() {
        let input = parse_input(r#"
30373
25512
65332
33549
35390
        "#); 
        assert_eq!(part1(&input), 21);
    }
}
