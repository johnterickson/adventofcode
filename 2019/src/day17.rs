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
    let mut comp = IntCode::new(program);
    let mut map : BTreeMap<(isize,isize), char> = BTreeMap::new();

    let mut cur = (0,0);

    let mut row = Vec::new();
    let mut rows = Vec::new();

    comp.run(
        |action: CallbackAction| {

            // println!("{:?} {:?} {:?}", cur, action, attempted_move);

            // let min_x = map.iter().map(|((x,_y),_c)| *x).min().unwrap();
            // let max_x = map.iter().map(|((x,_y),_c)| *x).max().unwrap();
            // let min_y = map.iter().map(|((_x,y),_c)| *y).min().unwrap();
            // let max_y = map.iter().map(|((_x,y),_c)| *y).max().unwrap();

            // println!("Current: {:?}", &cur);
            // println!("Queue:");
            // for m in &q {
            //     println!("{:?}", m);
            // }
            // for y in min_y..=max_y {
            //     for x in min_x..=max_x {
            //         print!("{}", map.get(&(x,y)).unwrap_or(&' '));
            //     }
            //     println!();
            // }
            // println!();

            // println!("{:?} {},{} facing {}", action, x, y, dir);
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

    

    for row in &rows {
        for c in row {
            print!("{}", c);
        }
        println!();
    }
    println!();


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

    align_sum
}

#[aoc(day17, part2)]
fn part2(program: &[isize]) -> usize {
    unimplemented!();
}

