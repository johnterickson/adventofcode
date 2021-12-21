
use std::collections::BTreeSet;

use aoc_runner_derive::{aoc, aoc_generator};

struct Grid {
    depths: Vec<Vec<u64>>,
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
                if let Some(row) = self.grid.depths.get(yy) {
                    if let Some(depth) = row.get(xx) {
                        return Some((yy, xx, *depth));
                    }
                }
            }
        }
    }
}

#[aoc_generator(day9)]
fn parse_input(input: &str) -> Grid {
    let depths = input.lines().map(|line| {
        line.trim().chars().map(|c| {
            assert!(c <= '9');
            assert!(c >= '0');
            ((c as u8) - ('0' as u8)) as u64
        }).collect()
    }).collect();

    Grid { depths }
}


#[aoc(day9, part1)]
fn part1(grid: &Grid) -> u64 { 
    let depths = &grid.depths;
    let rows = depths.len();
    let cols = depths[0].len();

    let mut risk = 0;
    for y in 0..rows {
        for x in 0..cols {
            let depth = depths[y][x];
            if grid.neighbors(y,x).all(|(_n_y,_n_x,n_depth)| {
                // println!("{},{}={} {},{}={}", y, x, depth, n_y, n_x, n_depth);
                //assert_ne!(depth, n_depth);
                depth < n_depth
            }) {
                // println!("{},{}={}", y, x, depth);
                risk += depth + 1;
            }
        }
    }

    risk
}

#[aoc(day9, part2)]
fn part2(grid: &Grid) -> usize { 
    let depths = &grid.depths;
    let rows = depths.len();
    let cols = depths[0].len();

    let mut basins = Vec::new();

    for y in 0..rows {
        for x in 0..cols {
            let depth = depths[y][x];
            if grid.neighbors(y,x).all(|(_n_y,_n_x,n_depth)| {
                depth < n_depth
            }) {
                let mut basin = BTreeSet::new();
                basin.insert((y,x));
                basins.push(basin);
            }
        }
    }

    // dbg!(&basins);

    let mut found_more = true;
    while found_more {
        found_more = false;

        'next: for y in 0..rows {
            for x in 0..cols {
                if basins.iter().any(|b| b.contains(&(y,x))) {
                    continue;
                }

                let depth = depths[y][x];

                if depth == 9 { continue; }

                for (n_y,n_x,n_depth) in grid.neighbors(y,x) {
                    if n_depth < depth {
                        if let Some(b) = basins.iter_mut().filter(|b| b.contains(&(n_y,n_x))).next() {
                            if b.insert((y,x)) {
                                // dbg!(&b, y, x);
                                found_more = true;
                                continue 'next;
                            }
                        }
                    }
                }
            }
        }
    }

    // for b in &basins {
    //     for y in 0..rows {
    //         for x in 0..cols {
    //             if b.contains(&(y,x)) {
    //                 print!(".");
    //             } else {
    //                 print!("{}", depths[y][x]);
    //             }
    //         }
    //         println!();
    //     }
    //     println!();
    // }

    basins.sort_by(|a,b| b.len().cmp(&a.len()));

    // dbg!(&basins);

    basins[0].len() * basins[1].len() * basins[2].len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
       let input = parse_input(
r#"2199943210
3987894921
9856789892
8767896789
9899965678"#);
       assert_eq!(part1(&input), 15);
    }


    #[test]
    fn part2_example() {
       let input = parse_input(
r#"2199943210
3987894921
9856789892
8767896789
9899965678"#);
       assert_eq!(part2(&input), 1134);
    }
}