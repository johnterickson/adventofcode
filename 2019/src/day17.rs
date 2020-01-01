use aoc_runner_derive::{aoc, aoc_generator};
use std::num::ParseIntError;
use std::collections::{BTreeMap, BTreeSet};
use std::collections::VecDeque;

use crate::intcode::*;

#[aoc_generator(day17)]
fn parse_input(input: &str) -> Result<Vec<isize>, ParseIntError> {
    input.split(',').map(|l| l.parse()).collect()
}

#[aoc(day17, part1)]
fn part1(program: &[isize]) -> usize {
    let (_rows, align_sum) = create_map(program);
    align_sum
}

fn create_map(program: &[isize]) -> (Vec<Vec<char>>, usize) {
    let mut comp = IntCode::new(program);
    let mut row = Vec::new();
    let mut rows = Vec::new();
    comp.run(
        |action: CallbackAction| {
            match action {
                CallbackAction::ReadInput => {
                    unimplemented!();
                }
                CallbackAction::WriteOutput(output) => {
                    match output {
                        10 => {
                            if row.len() > 0 {
                                rows.push(row.to_vec());
                            }
                            row.clear();
                        }
                        c => {
                            row.push(c as u8 as char);
                        },
                    }

                    None
                }
            }
        });

    let max_y = rows.len() - 1;
    let max_x = rows[0].len() - 1;
    let dirs: &[(isize,isize)]= &[(-1,0),(1,0),(0,1),(0,-1)];

    let mut align_sum = 0;

    for y in 0..=max_y {
        for x in 0..=max_x {
            if rows[y][x] == '#' && dirs.iter().all(|(dx,dy)| {
                let xx = x as isize + dx;
                let yy = y as isize + dy;
                if xx < 0 || yy < 0 {
                    return false;
                }
                let xx = xx as usize;
                let yy = yy as usize;
                if xx > max_x || yy > max_y {
                    false
                } else if rows[yy as usize][xx as usize] != '#' {
                    false
                } else {
                    true
                }
            }) {
                rows[y][x] = 'O';
                align_sum += y*x;
            }
        }
    }

    

    for row in &rows {
        for c in row {
            print!("{}", c);
        }
        println!();
    }
    println!();
    (rows, align_sum)
}

#[aoc(day17, part2)]
fn part2(program: &[isize]) -> usize {
    let (rows, align_sum) = create_map(program);

    unimplemented!();
}

