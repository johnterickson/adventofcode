use std::fmt::Debug;

use aoc_runner_derive::{aoc, aoc_generator};
use pathfinding::prelude::{absdiff, astar};

struct Grid {
    values: Vec<Vec<u64>>,
}

impl Grid {
    fn neighbors<'a>(&'a self, y: usize, x: usize) -> NeighborIterator<'a> {
        NeighborIterator {
            y,
            x,
            grid: &self,
            i: 0,
        }
    }

    fn rows(&self) -> usize {
        self.values.len()
    }

    fn cols(&self) -> usize {
        self.values[0].len()
    }
}

impl Debug for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.values {
            for val in row {
                write!(f, "{}", val)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
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
                0 => (self.y.checked_sub(1), Some(self.x)),
                1 => (self.y.checked_add(1), Some(self.x)),
                2 => (Some(self.y), self.x.checked_sub(1)),
                3 => (Some(self.y), self.x.checked_add(1)),
                _ => return None,
            };

            self.i += 1;

            if let (Some(yy),Some(xx)) = (yy,xx) {
                if let Some(row) = self.grid.values.get(yy) {
                    if let Some(depth) = row.get(xx) {
                        return Some((yy, xx, *depth));
                    }
                }
            }
        }
    }
}


#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
struct Pos(usize,usize);

impl Pos {
    fn distance(&self, other: &Pos) -> u64 {
        (absdiff(self.0, other.0) + absdiff(self.1, other.1)) as u64
    }

    fn successors(&self, grid: &Grid) -> Vec<(Pos, u64)> {
        grid.neighbors(self.0,self.1)
            .map(|(x, y, value)| (Pos(x, y), value))
            .collect()
    }
}

#[aoc_generator(day15)]
fn parse_input(input: &str) -> Grid {
    let values = input.lines().map(|line| {
        line.trim().chars().map(|c| {
            assert!(c <= '9');
            assert!(c >= '0');
            ((c as u8) - ('0' as u8)) as u64
        }).collect()
    }).collect();

    Grid { values }
}

fn find_path(grid: &Grid) -> (Vec<Pos>,u64) {
    let goal = Pos(grid.cols()-1, grid.rows()-1);
    astar(&Pos(0,0), |p| p.successors(grid), |p| p.distance(&goal), |p| *p == goal).unwrap()
}

#[aoc(day15, part1)]
fn part1(grid: &Grid) -> u64 { 
    let (_path, cost) = find_path(grid);
    cost
}

fn expand_grid(grid: &Grid) -> Grid {
    let mut wide_cols = Vec::new();
    for row in &grid.values {
        let mut wide_col = Vec::new();
        for repeat in 0..5 {
            for value in row {
                let value = value + repeat;
                let value = if value > 9 { value - 9 } else { value };
                wide_col.push(value);
            }
        }
        assert_eq!(5 * grid.cols(), wide_col.len());
        wide_cols.push(wide_col);
    }

    let mut wide_rows = Vec::new();
    for repeat in 0..5 {
        for row in &wide_cols {
            let mut wide_row = Vec::new();
            for value in row {
                let value = value + repeat;
                let value = if value > 9 { value - 9 } else { value };
                wide_row.push(value);
            }
            wide_rows.push(wide_row);
        }
    }

    assert_eq!(5 * grid.rows(), wide_rows.len());
    for row in &wide_rows {
        assert_eq!(5 * grid.cols(), row.len());

    }
    
    Grid { values: wide_rows }
}

#[aoc(day15, part2)]
fn part2(grid: &Grid) -> u64 { 
    let grid = expand_grid(&grid);
    let (_, cost) = find_path(&grid);
    cost
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fmt::Write;


    #[test]
    fn part1_example1() {
       let grid = parse_input(
r#"1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581"#);
        let (path, cost) = find_path(&grid);
        println!("{:?}", &path);
        assert_eq!(cost, 40);
    }

    #[test]
    fn part2_exand_grid_1_1() {
        let grid = parse_input(r#"8"#);
        let grid = expand_grid(&grid);
        let mut grid_string = String::new();
        write!(&mut grid_string, "{:?}", &grid.values).unwrap();

        assert_eq!("[[8, 9, 1, 2, 3], [9, 1, 2, 3, 4], [1, 2, 3, 4, 5], [2, 3, 4, 5, 6], [3, 4, 5, 6, 7]]", &grid_string);
    }

    #[test]
    fn part2_exand_grid_2_2() {
        let grid = parse_input(
r#"15
37"#);
        let grid = expand_grid(&grid);

        assert_eq!(&[1, 5, 2, 6, 3, 7, 4, 8, 5, 9], grid.values[0].as_slice());
        assert_eq!(&[3, 7, 4, 8, 5, 9, 6, 1, 7, 2], grid.values[1].as_slice());
        assert_eq!(&[2, 6, 3, 7, 4, 8, 5, 9, 6, 1], grid.values[2].as_slice());

        fn normalize(i: u64) -> u64 {
            if i > 9 { i - 9 } else { i}
        }
        for repeat in 0..5 {
            let mut expected = [1, 5, 2, 6, 3, 7, 4, 8, 5, 9];
            for i in expected.iter_mut() {
                *i = normalize(*i + repeat as u64);
            }

            assert_eq!(&expected, grid.values[repeat * 2].as_slice());

            let mut expected = [3, 7, 4, 8, 5, 9, 6, 1, 7, 2];
            for i in expected.iter_mut() {
                *i = normalize(*i + repeat as u64);
            }

            assert_eq!(&expected, grid.values[repeat * 2 + 1].as_slice());
        }
    }

    #[test]
    fn part2_example1() {
       let grid = parse_input(
r#"1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581"#);
        let grid = expand_grid(&grid);
        // dbg!(&grid);
        let (path, cost) = find_path(&grid);
        let path: Vec<_> = path.iter().map(|p| (grid.values[p.0][p.1], p.0, p.1)).collect();
        println!("{:?}", &path);
        assert_eq!(cost, 315);
    }
}