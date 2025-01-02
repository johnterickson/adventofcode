use aoc_runner_derive::{aoc, aoc_generator};

use crate::Grid;

/*

--- Day 4: Ceres Search ---
"Looks like the Chief's not here. Next!" One of The Historians pulls out a device and pushes the only button on it. After a brief flash, you recognize the interior of the Ceres monitoring station!

As the search for the Chief continues, a small Elf who lives on the station tugs on your shirt; she'd like to know if you could help her with her word search (your puzzle input). She only has to find one word: XMAS.

This word search allows words to be horizontal, vertical, diagonal, written backwards, or even overlapping other words. It's a little unusual, though, as you don't merely need to find one instance of XMAS - you need to find all of them. Here are a few ways XMAS might appear, where irrelevant characters have been replaced with .:


..X...
.SAMX.
.A..A.
XMAS.S
.X....
The actual word search will be full of letters instead. For example:

MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
In this word search, XMAS occurs a total of 18 times; here's the same word search again, but where letters not involved in any XMAS have been replaced with .:

....XXMAS.
.SAMXMS...
...S..A...
..A.A.MS.X
XMASAMX.MM
X.....XA.A
S.S.S.S.SS
.A.A.A.A.A
..M.M.M.MM
.X.X.XMASX
Take a look at the little Elf's word search. How many times does XMAS appear?

 */

#[aoc_generator(day4)]
fn parse_input(input: &str) -> Grid<char> {
    let input = input.trim();
    Grid(input.lines().map(|l| l.trim().chars().collect()).collect())
}

const WORD: [char; 4] = ['X', 'M', 'A', 'S'];
const DIRS: [(isize, isize); 8] = [
    (0, 1),
    (1, 0),
    (1, 1),
    (1, -1),
    (0, -1),
    (-1, 0),
    (-1, 1),
    (-1, -1),
];

#[aoc(day4, part1)]
fn part1(grid: &Grid<char>) -> u32 {
    let mut count = 0;
    for (r, row) in grid.0.iter().enumerate() {
        for (c, ch) in row.iter().enumerate() {
            if *ch == WORD[0] {
                'dir: 
                for (dr, dc) in DIRS.iter() {
                    for i in 1..WORD.len() {
                        if grid.get_offset(r, i as isize * dr, c, i as isize * dc) != Some(WORD[i]) {
                            continue 'dir;
                        }
                    }

                    count += 1;
                }
            }
        }
    }

    count
}

#[aoc(day4, part2)]
fn part2(grid: &Grid<char>) -> u32 {
    let mut count = 0;
    for (r, row) in grid.0.iter().enumerate() {
        for (c, ch) in row.iter().enumerate() {
            if *ch != 'A' {
                continue;
            }

            let up_left = grid.get_offset(r, -1, c, -1);
            let up_right = grid.get_offset(r, -1, c, 1);
            let down_left = grid.get_offset(r, 1, c, -1);
            let down_right = grid.get_offset(r, 1, c, 1);

            let up_left_diag = 
                (up_left == Some('M') && down_right == Some('S')) ||
                (up_left == Some('S') && down_right == Some('M'));

            let down_left_diag = 
                (down_left == Some('M') && up_right == Some('S')) ||
                (down_left == Some('S') && up_right == Some('M'));

            if up_left_diag && down_left_diag {
                count += 1;
            }
        }
    }
    count
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let input = parse_input(
            r#"
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
        "#
            .trim(),
        );
        assert_eq!(part1(&input), 18);
    }

    #[test]
    fn part2_example() {
        let input = parse_input(
            r#"
.M.S......
..A..MSMS.
.M.S.MAA..
..A.ASMSM.
.M.S.M....
..........
S.S.S.S.S.
.A.A.A.A..
M.M.M.M.M.
..........
        "#
            .trim(),
        );
        assert_eq!(part2(&input), 9);
    }
}
