use aoc_runner_derive::{aoc, aoc_generator};
use std::num::ParseIntError;

use std::collections::BTreeMap;

enum Direction {
    L,R,U,D
}

struct Move {
    pub dir : Direction,
    pub steps : usize,
}

struct Wire {
    pub moves : Vec<Move>,
}

struct SparseArray<T: Clone> {
    entries : BTreeMap<usize, T>,
    default_value : T,
}

impl<T: Clone> SparseArray<T> {
    fn new(default_value: T) -> SparseArray<T> {
        SparseArray {
            entries: BTreeMap::new(),
            default_value
        }
    }

    fn non_default_values<'a>(&'a self) -> Box<dyn Iterator<Item = (&usize, &T)> + 'a> {
        Box::new(self.entries.iter())
    }
}

impl<T: Clone> std::ops::Index<usize> for SparseArray<T> {
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        match self.entries.get(&index) {
            None => &self.default_value,
            Some(e) => &e,
        }
    }
}

impl<T: Clone> std::ops::IndexMut<usize> for SparseArray<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.entries.entry(index).or_insert(self.default_value.clone())
    }
}

#[aoc_generator(day3)]
fn parse_input(input: &str) -> Result<Vec<Wire>, ParseIntError> {
    Ok(
        input.lines().map( |l| {
            let moves = l.split(",").map(|m| {
                let (dir,steps) = m.split_at(1);
                let dir = match dir.chars().next().unwrap() {
                    'L' => Direction::L,
                    'R' => Direction::R,
                    'U' => Direction::U,
                    'D' => Direction::D,
                    _ => panic!(),
                };
                let steps = steps.parse().unwrap();
                Move { dir, steps }
            }).collect();
            Wire { moves}
        }).collect()
    )
}

fn run(input: &[Wire], len: usize) -> usize {
    let (mid_x, mid_y) = (len/2, len/2);
    let mut grid : SparseArray<usize> = SparseArray::new(0);
    let mut w_bit = 1;
    for w in input {
        let (mut x, mut y) = (mid_x, mid_y);
        for m in &w.moves {
            match m.dir {
                Direction::U => {
                    for _ in 0..m.steps {
                        y += 1;
                        grid[x*len+y] |= w_bit;
                    }
                },
                Direction::D => {
                    for _ in 0..m.steps {
                        y -= 1;
                        grid[x*len+y] |= w_bit;
                    }
                },
                Direction::L => {
                    for _ in 0..m.steps {
                        x -= 1;
                        grid[x*len+y] |= w_bit;
                    }
                },
                Direction::R => {
                    for _ in 0..m.steps {
                        x += 1;
                        grid[x*len+y] |= w_bit;
                    }
                },
            }
        }

        w_bit <<= 1;
    }

    let mut min_dist = usize::max_value();
    for e in grid.non_default_values() {
        let (index, value) = e;
        let (x, y) = (index/len, index % len);
        if value.count_ones() < 2 || (x,y) == (mid_x, mid_y) {
            continue;
        }

        let d = (x as isize - mid_x as isize).abs() 
                + (y as isize - mid_y as isize).abs();
        min_dist = std::cmp::min(min_dist, d as usize);
    }

    min_dist
}

#[aoc(day3, part1)]
fn part1(input: &[Wire]) -> usize {
    run(input, 1000000)
}

fn run2(input: &[Wire], len: usize) -> usize {
    let (mid_x, mid_y) = (len/2, len/2);
    let mut grid : BTreeMap<(usize,usize), BTreeMap<usize,usize>> = BTreeMap::new();
    for (wire_index, wire) in input.iter().enumerate() {
        let (mut x, mut y) = (mid_x, mid_y);
        let mut wire_steps = 0;
        for m in &wire.moves {
            for _ in 0..m.steps {
                match m.dir {
                    Direction::U => { y += 1; },
                    Direction::D => { y -= 1; },
                    Direction::L => { x -= 1; },
                    Direction::R => { x += 1; },
                }
                wire_steps += 1;

                let cell = grid
                    .entry((x,y))
                    .or_insert_with(|| BTreeMap::new());

                cell.entry(wire_index).or_insert(wire_steps);
            }
        }
    }

    let mut min_sum = usize::max_value();
    for ((x,y), wires) in grid {
        if wires.len() < 2 || (x,y) == (mid_x, mid_y) {
            continue;
        }

        let sum = wires.iter().map(|(_wire_index, wire_steps)| wire_steps).sum();
        min_sum = std::cmp::min(min_sum, sum);
    }

    min_sum
}

#[aoc(day3, part2)]
fn part2(input: &[Wire]) -> usize {
    run2(input, 1000000)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        {
            let input = parse_input(
"R8,U5,L5,D3
U7,R6,D4,L4").unwrap();

            assert_eq!(run(&input, 100), 6);
        }
        
        {
            let input = parse_input(
"R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51
U98,R91,D20,R16,D67,R40,U7,R15,U6,R7").unwrap();

            assert_eq!(run(&input, 1000), 135);
        }

        {
            let input = parse_input(
"R75,D30,R83,U83,L12,D49,R71,U7,L72
U62,R66,U55,R34,D71,R55,D58,R83").unwrap();

            assert_eq!(run(&input, 1000), 159);
        }
    }

    #[test]
    fn part2_example() {
        {
            let input = parse_input(
"R75,D30,R83,U83,L12,D49,R71,U7,L72
U62,R66,U55,R34,D71,R55,D58,R83").unwrap();

            assert_eq!(run2(&input, 10000), 610);
        }

        {
            let input = parse_input(
"R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51
U98,R91,D20,R16,D67,R40,U7,R15,U6,R7").unwrap();

            assert_eq!(run2(&input, 10000), 410);
        }
    }
}
