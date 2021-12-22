
use std::collections::BTreeSet;

use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Clone, Debug, PartialEq)]
struct Grid(Vec<Vec<u64>>);

impl Grid {
    fn neighbors<'a>(&'a self, y: usize, x: usize) -> NeighborIterator<'a> {
        NeighborIterator {
            y,
            x,
            grid: &self,
            i: 0,
        }
    }

    fn rows(&self) -> usize { self.0.len() }
    fn cols(&self) -> usize { self.0[0].len() }
}

struct NeighborIterator<'a> {
    y: usize,
    x: usize,
    grid: &'a Grid,
    i: usize,
}

impl<'a> Iterator for NeighborIterator<'a> {
    type Item = (usize,usize,u64);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let (yy,xx) = match self.i {
                0 => (self.y.checked_sub(1), self.x.checked_sub(1)),
                1 => (self.y.checked_sub(1), Some(self.x)),
                2 => (self.y.checked_sub(1), self.x.checked_add(1)),
                3 => (Some(self.y), self.x.checked_sub(1)),
                4 => (Some(self.y), self.x.checked_add(1)),
                5 => (self.y.checked_add(1), self.x.checked_sub(1)),
                6 => (self.y.checked_add(1), Some(self.x)),
                7 => (self.y.checked_add(1), self.x.checked_add(1)),
                _ => return None,
            };

            self.i += 1;

            if let (Some(yy),Some(xx)) = (yy,xx) {
                if let Some(row) = self.grid.0.get(yy) {
                    if let Some(value) = row.get(xx) {
                        return Some((yy, xx, *value));
                    }
                }
            }
        }
    }
}

#[aoc_generator(day11)]
fn parse_input(input: &str) -> Grid {
    let grid = input.lines().map(|line| {
        line.trim().chars().map(|c| {
            assert!(c <= '9');
            assert!(c >= '0');
            ((c as u8) - ('0' as u8)) as u64
        }).collect()
    }).collect();

    Grid(grid)
}


fn step(grid: &mut Grid) -> usize {
    
    // bump energy by 1
    for row in grid.0.iter_mut() {
        for energy in row.as_mut_slice() {
            *energy += 1;
        }
    }

    let rows = grid.rows();
    let cols = grid.cols();
    let mut flashed = BTreeSet::new();

    let mut flash = true;
    while flash {
        flash  = false;
        for y in 0..rows {
            for x in 0..cols {
                let energy = grid.0[y][x];
                if energy > 9 && flashed.insert((y,x)) {
                    let neighbors: Vec<_> = grid.neighbors(y, x).collect();
                    for (n_y, n_x, _) in neighbors {
                        grid.0[n_y][n_x] += 1;
                    }
                    flash = true;
                }
            }
        }
    }

    for (y,x) in flashed.iter() {
        grid.0[*y][*x] = 0;
    }

    flashed.len()
}

#[aoc(day11, part1)]
fn part1(grid: &Grid) -> usize { 
    let mut grid: Grid = grid.clone();
    let mut flashes = 0;
    for _ in 0..100 {
        flashes += step(&mut grid);
    }

    flashes
}

#[aoc(day11, part2)]
fn part2(grid: &Grid) -> usize { 
    let mut grid: Grid = grid.clone();
    let octopus_count = grid.rows() * grid.cols();

    let mut step_count = 0;
    loop {
        step_count += 1;
        if octopus_count == step(&mut grid) {
            return step_count;
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example1() {
       let mut grid = parse_input(
r#"11111
19991
19191
19991
11111"#);
        assert_eq!(step(&mut grid), 9, "{:?}", &grid);
        assert_eq!(&parse_input(
r#"34543
40004
50005
40004
34543"#), &grid);
        assert_eq!(step(&mut grid), 0);
    }

    #[test]
    fn part1_example2() {
       let mut grid = parse_input(
r#"5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526"#);
       assert_eq!(step(&mut grid), 0);
       assert_eq!(&parse_input(
r#"6594254334
3856965822
6375667284
7252447257
7468496589
5278635756
3287952832
7993992245
5957959665
6394862637"#), &grid);
    }

    #[test]
    fn part1_example3() {
       let grid = parse_input(
r#"5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526"#);
       assert_eq!(part1(&grid), 1656);
    }

    #[test]
    fn part_example() {
       let grid = parse_input(
r#"5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526"#);
       assert_eq!(part2(&grid), 195);
    }
}