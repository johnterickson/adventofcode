use std::{collections::{HashMap, HashSet}, fmt::{Debug, Display}, hash::Hash};

use aoc_runner_derive::{aoc, aoc_generator};

use crate::Grid;

/*
--- Day 8: Resonant Collinearity ---
You find yourselves on the roof of a top-secret Easter Bunny installation.

While The Historians do their thing, you take a look at the familiar huge antenna. Much to your surprise, it seems to have been reconfigured to emit a signal that makes people 0.1% more likely to buy Easter Bunny brand Imitation Mediocre Chocolate as a Christmas gift! 
Unthinkable!

Scanning across the city, you find that there are actually many such antennas. Each antenna is tuned to a specific frequency indicated by a single lowercase letter, uppercase letter, or digit. You create a map (your puzzle input) of these antennas. For example:

............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............
The signal only applies its nefarious effect at specific antinodes based on the resonant frequencies of the antennas. 
In particular, an antinode occurs at any point that is perfectly in line with two antennas of the same frequency - but only when one of the antennas is twice as far away as the other. 
This means that for any pair of antennas with the same frequency, there are two antinodes, one on either side of them.

So, for these two antennas with frequency a, they create the two antinodes marked with #:

..........
...#......
..........
....a.....
..........
.....a....
..........
......#...
..........
..........
Adding a third antenna with the same frequency creates several more antinodes. It would ideally add four antinodes, but two are off the right side of the map, so instead it adds only two:

..........
...#......
#.........
....a.....
........a.
.....a....
..#.......
......#...
..........
..........
Antennas with different frequencies don't create antinodes; A and a count as different frequencies. However, antinodes can occur at locations that contain antennas. In this diagram, the lone antenna with frequency capital A creates no antinodes but has a lowercase-a-frequency antinode at its location:

..........
...#......
#.........
....a.....
........a.
.....a....
..#.......
......A...
..........
..........
The first example has antennas with two different frequencies, so the antinodes they create look like this, plus an antinode overlapping the topmost A-frequency antenna:

......#....#
...#....0...
....#0....#.
..#....0....
....0....#..
.#....A.....
...#........
#......#....
........A...
.........A..
..........#.
..........#.
Because the topmost A-frequency antenna overlaps with a 0-frequency antinode, there are 14 total unique locations that contain an antinode within the bounds of the map.

Calculate the impact of the signal. How many unique locations within the bounds of the map contain an antinode?

*/

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Cell {
    Empty,
    Antenna(char),
    Antinode
}

impl Debug for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Cell::Empty => write!(f, "."),
            Cell::Antenna(c) => write!(f, "{}", c),
            Cell::Antinode => write!(f, "#"),
        }
    }
}

#[aoc_generator(day8)]
fn parse(input: &str) -> Grid<Cell> {
    Grid(input.trim().lines().map(|line| {
        line.chars().map(|c| match c {
            '.' => Cell::Empty,
            c if c.is_alphanumeric() => Cell::Antenna(c),
            _ => panic!("Invalid character in input"),
        }).collect()
    }).collect())
}

#[aoc(day8, part1)]
fn part1(input: &Grid<Cell>) -> u32 {
    let mut antennas = HashMap::new();
    for (r, row) in input.0.iter().enumerate() {
        for (c, cell) in row.iter().enumerate() {
            if let Cell::Antenna(freq) = cell {
                antennas.entry(*freq).or_insert_with(HashSet::new).insert((r, c));
            }
        }
    }

    let mut antinodes = HashSet::new();

    for (_, antennas) in antennas {
        let antennas : Vec<_> = antennas.iter().copied().collect();
        for i in 0..antennas.len() {
            for j in i + 1..antennas.len() {
                    let (r1, c1) = antennas[i];
                    let (r2, c2) = antennas[j];
                    
                    let dr = r2 as isize - r1 as isize;
                    let dc = c2 as isize - c1 as isize;

                    if let Some((r,c, cell)) = input.get_offset(r2, dr, c2, dc) {
                        // if cell == Cell::Empty 
                        {
                            antinodes.insert((r,c));
                        }
                    }

                    if let Some((r,c, cell)) = input.get_offset(r1, -dr, c1, -dc) {
                        // if cell == Cell::Empty 
                        {
                            antinodes.insert((r,c));
                        }
                    }
                }
            }
        }

    // dbg!(&input);

    // let mut result = input.clone();
    // for an in &antinodes {
    //     result.0[an.0][an.1] = Cell::Antinode;
    // }

    // dbg!(&result);


    antinodes.len() as u32
}

#[aoc(day8, part2)]
fn part2(input: &Grid<Cell>) -> u32 {
    todo!()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(r#"
............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............
            "#)), 14);
    }

    // #[test]
    // fn part2_example() {
    //     assert_eq!(part2(&parse("<EXAMPLE>")), "<RESULT>");
    // }
}