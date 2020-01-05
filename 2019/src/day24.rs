use aoc_runner_derive::{aoc, aoc_generator};

use std::collections::HashSet;
use ndarray::Array2;

#[aoc_generator(day24)]
fn parse_input(input: &str) -> Result<Array2<char>, ndarray::ShapeError> {
    let rows : Vec<Vec<_>> = input.lines().map(|line| line.trim().chars().collect()).collect();
    let cells : Vec<_> = rows.iter().map(|row| row.iter()).flatten().cloned().collect();
    Array2::from_shape_vec((rows.len(), rows[0].len()), cells)
}

const ADJACENT : &[(isize,isize)]= &[(0,1),(0,-1),(1,0),(-1,0)];

fn evolve(cells: &Array2<char>) -> Array2<char> {
    let mut next = Array2::from_elem(cells.dim(), '?');

    for ((x,y), cell) in cells.indexed_iter() {
        let adjacent_bugs = ADJACENT.iter().filter(|(xx,yy)| {
            let (xx,yy) = (x as isize + xx, y as isize +yy);
            if xx < 0 || yy < 0 {
                // println!(" {} {} out of bounds", xx, yy);
                false
            } else {
                let cell = cells.get((xx as usize, yy as usize));
                let has_bug = cell == Some(&'#');
                // println!(" {} {} {:?} {}", xx, yy, cell, has_bug);
                has_bug
            }
        }).count();

        // println!("{} {} {}", x, y, adjacent_bugs);

        if cell == &'#' {
            if adjacent_bugs == 1 {
                next[(x,y)] = '#';
            } else {
                next[(x,y)] = '.';
            }
        } else if cell == &'.' {
            if adjacent_bugs == 1 || adjacent_bugs == 2 {
                next[(x,y)] = '#';
            } else {
                next[(x,y)] = '.';
            }
        } else {
            unreachable!();
        }
    }

    next
}

fn biodiversity(cells: &Array2<char>) -> usize {
    let mut rating = 0;
    for ((x,y), cell) in cells.indexed_iter() {
        if cell == &'#' {
            let index = cells.dim().0 * x + y;
            rating |= 1 << index;
            // println!("{} {} {}", x,y, index);
        }
    }
    rating
}

#[aoc(day24, part1)]
fn part1(cells: &Array2<char>) -> usize {
    // println!("{:?}", cells);
    let mut seen = HashSet::new();
    let mut current = cells.clone();
    loop {
        let next = evolve(&current);
        if seen.contains(&next) {
            return biodiversity(&next);
        }
        seen.insert(next.clone());
        current = next;
    }
}

#[aoc(day24, part2)]
fn part2(program: &Array2<char>) -> isize {
    unimplemented!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let input = parse_input(
            "....#
            #..#.
            #..##
            ..#..
            #...."
        ).unwrap();
        let next = evolve(&input);
        let expected = parse_input(
            "#..#.
            ####.
            ###.#
            ##.##
            .##.."
        ).unwrap();
        assert_eq!(expected, next);
    }

    #[test]
    fn part1_example2() {
        assert_eq!(2129920, biodiversity(&parse_input(
           ".....
            .....
            .....
            #....
            .#..."
        ).unwrap()));
    }
}