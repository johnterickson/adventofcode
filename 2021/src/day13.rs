use std::{fmt::Display, collections::BTreeSet, str::Lines};

use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Clone)]
struct SparseGrid {
    dots: BTreeSet<(usize,usize)>
}

impl SparseGrid {
    fn get(&self, x: usize, y: usize) -> bool {
        self.dots.contains(&(x,y))
    }

    fn rows(&self) -> usize {
        self.dots.iter().map(|(_,y)| *y).max().unwrap() + 1
    }

    fn cols(&self) -> usize {
        self.dots.iter().map(|(x,_)| *x).max().unwrap() + 1
    }

    fn parse(mut lines: &mut Lines) -> SparseGrid {
        let mut dots = BTreeSet::new();
        for line in &mut lines {
            let line = line.trim();
            if line.len() == 0 {
                break;
            }
            let mut points = line.split(',');
            dots.insert((
                usize::from_str_radix(points.next().unwrap().trim(), 10).unwrap(),
                usize::from_str_radix(points.next().unwrap().trim(), 10).unwrap()
            ));
        }
        SparseGrid { dots }
    }
}

impl Display for SparseGrid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f)?;
        for y in 0..self.rows() {
            for x in 0..self.cols() {
                write!(f, "{}", if self.dots.contains(&(x,y)) { "#" } else { "." })?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[derive(Clone, Copy)]
enum Fold {
    X(usize),
    Y(usize)
}

impl Fold {
    fn parse(line: &str) -> Fold {
        let line = line.trim();
        let mut tokens = line.split('=');
        let axis = tokens.next().unwrap().trim().chars().last().unwrap();
        let point = usize::from_str_radix(tokens.next().unwrap(),10).unwrap();
        match axis {
            'x' => Self::X(point),
            'y' => Self::Y(point),
            _ => panic!(),
        }
    }
}

#[aoc_generator(day13)]
fn parse_input(input: &str) -> (SparseGrid,Vec<Fold>) {
    let mut lines = input.lines();

    let dots = SparseGrid::parse(&mut lines);
    let folds = lines.map(Fold::parse).collect();

    (dots, folds)
}

fn fold(grid: &mut SparseGrid, fold: Fold)
{
    match fold {
        Fold::X(axis) => {
            for y in 0..grid.rows() {
                assert!(!grid.get(axis,y));
            }

            let points_to_flip: Vec<_> = grid.dots.iter()
                .filter(|(x,_)| *x > axis)
                .cloned()
                .collect();
            for p in points_to_flip {
                assert!(grid.dots.remove(&p));
                let (x,y) = p;
                let x = axis - (x-axis);
                grid.dots.insert((x,y));
            }
        }
        Fold::Y(axis) => {
            for x in 0..grid.cols() {
                assert!(!grid.get(x,axis));
            }

            let points_to_flip: Vec<_> = grid.dots.iter()
                .filter(|(_,y)| *y > axis)
                .cloned()
                .collect();
            for p in points_to_flip {
                assert!(grid.dots.remove(&p));
                let (x,y) = p;
                let y = axis - (y-axis);
                grid.dots.insert((x,y));
            }
        }
    }
}

#[aoc(day13, part1)]
fn part1(ins: &(SparseGrid,Vec<Fold>)) -> usize { 
    let (grid, folds) = ins;
    let mut grid: SparseGrid = grid.clone();

    fold(&mut grid, folds[0]);

    grid.dots.len()
}

#[aoc(day13, part2)]
fn part2(ins: &(SparseGrid,Vec<Fold>)) -> SparseGrid { 
    let (grid, folds) = ins;
    let mut grid: SparseGrid = grid.clone();

    for f in folds {
        fold(&mut grid, *f);
    }

    grid
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fmt::Write;


    #[test]
    fn part1_example1() {
       let mut input = parse_input(
r#"6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5"#);
        let mut grid = String::new();
        write!(&mut grid, "{}", &input.0).unwrap();
        assert_eq!(grid.as_str(), r#"
...#..#..#.
....#......
...........
#..........
...#....#.#
...........
...........
...........
...........
...........
.#....#.##.
....#......
......#...#
#..........
#.#........
"#);

        fold(&mut input.0, input.1[0]);

        let mut grid = String::new();
        write!(&mut grid, "{}", &input.0).unwrap();

        assert_eq!(grid.as_str(), r#"
#.##..#..#.
#...#......
......#...#
#...#......
.#.#..#.###
"#);

        assert_eq!(input.0.dots.len(), 17);
    

        fold(&mut input.0, input.1[1]);

        let mut grid = String::new();
        write!(&mut grid, "{}", &input.0).unwrap();

        assert_eq!(grid.as_str(), r#"
#####
#...#
#...#
#...#
#####
"#);
    }
}