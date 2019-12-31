use aoc_runner_derive::{aoc, aoc_generator};
use std::num::ParseIntError;
use std::collections::{BTreeMap, BTreeSet};
use std::collections::VecDeque;

use crate::intcode::*;

#[aoc_generator(day15)]
fn parse_input(input: &str) -> Result<Vec<isize>, ParseIntError> {
    input.split(',').map(|l| l.parse()).collect()
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum Direction {
    North,
    South,
    West,
    East,
}

impl Direction {
    fn encode(&self) -> usize {
        match self {
            Direction::North => 1,
            Direction::South => 2,
            Direction::West => 3,
            Direction::East => 4
        }
    }

    fn reverse(&self) -> Direction {
        match self {
            Direction::North => Direction::South,
            Direction::South => Direction::North,
            Direction::West => Direction::East,
            Direction::East => Direction::West,
        }
    }

    fn new_coords(&self, (x,y): (isize, isize)) -> (isize, isize) {
        let (dx,dy) = match self {
            Direction::North => (0,-1),
            Direction::South => (0,1),
            Direction::West => (-1,0),
            Direction::East => (1,0)
        };
        (x+dx,y+dy)
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum Phase {
    Explore,
    Backtrack,
}

#[aoc(day15, part1)]
fn part1(program: &[isize]) -> usize {
    let mut comp = IntCode::new(program);
    let mut map : BTreeMap<(isize,isize), char> = BTreeMap::new();

    let mut cur = (0,0);

    let mut q = VecDeque::new();
    let dirs = &[Direction::South, Direction::North, Direction::East, Direction:: West ];

    for d in dirs {
        let (from, to) = (cur, d.new_coords(cur));
        q.push_back((d.reverse(), to, Phase::Backtrack, from));
        q.push_back((*d, from, Phase::Explore, to));
    }

    map.entry(cur).or_insert('d');

    let mut attempted_move = None;
    let mut oxygen = None;

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
                    assert_eq!(attempted_move, None);

                    if let Some(step) = q.pop_back() {
                        // print!("{:?} ", step);
                        let (dir, from, phase, to) = step;
                        assert_eq!(cur, from);
                        assert_eq!(to, dir.new_coords(from));
                        attempted_move = Some((dir, to));
                        // println!("{:?} -> {:?}", &cur, &attempted_move);
                        Some(dir.encode() as isize)
                    } else {
                        None
                    }
                }
                CallbackAction::WriteOutput(output) => {
                    match output {
                        0 => {
                            // wall
                            let (_dir, wall) = attempted_move.unwrap();
                            map.insert(wall, '#');
                            let _ = q.pop_back(); // remove backtrack
                            attempted_move = None;
                        },
                        1 | 2 => {
                            // moved
                            *map.get_mut(&cur).unwrap() = '.';
                            cur = attempted_move.unwrap().1;
                            *map.entry(cur).or_insert('d') = 'd';
                            attempted_move = None;

                            for d in dirs {
                                if None == map.get(&d.new_coords(cur)) {
                                    let (from, to) = (cur, d.new_coords(cur));
                                    q.push_back((d.reverse(), to, Phase::Backtrack, from));
                                    q.push_back((*d, from, Phase::Explore, to));
                                }
                            }

                            if output == 2 {
                                
                                oxygen = Some(cur);
                            }
                        },
                        _ => {
                            unreachable!();
                        }
                    };

                    None
                }
            }
        });

    let oxygen = oxygen.unwrap();
    println!("Current:{:?} -> Oxygen:{:?}", cur, oxygen);
    *map.entry(oxygen).or_insert('@') = '@';

    assert_eq!((0,0), cur);
    *map.get_mut(&cur).unwrap() = '.';

    let min_x = map.iter().map(|((x,_y),_c)| *x).min().unwrap();
    let max_x = map.iter().map(|((x,_y),_c)| *x).max().unwrap();
    let min_y = map.iter().map(|((_x,y),_c)| *y).min().unwrap();
    let max_y = map.iter().map(|((_x,y),_c)| *y).max().unwrap();

    // print!("     ");
    // for x in min_x..=max_x { 
    //     print!("{x:>width$} ", x=x, width=3);
    // }
    // println!();

    for y in min_y..=max_y {
        print!("{y:>width$} ", y=y, width=4);
        for x in min_x..=max_x {
            print!("{}", map.get(&(x,y)).unwrap_or(&' '));
        }
        println!();
    }

    // we have the map!!
    // BFS time!!

    let mut depth = 0;
    let mut seen = BTreeSet::new();
    let mut to_visit = Vec::new();
    let mut next = Vec::new();
    next.push(cur);
    while next.len() > 0 {
        assert_eq!(0, to_visit.len());
        while let Some(x) = next.pop() {
            to_visit.push(x);
        }
        assert_eq!(0, next.len());

        println!("Depth: {} to_visit:{}", depth, to_visit.len());
        while let Some(z) = to_visit.pop() {

            if let Some(c) = map.get(&z) {

                if z == oxygen {
                    return depth;
                }

                if seen.insert(z) {
                    if c == &'.' {
                        for d in dirs {
                            next.push(d.new_coords(z));
                        }
                    }

                }
            } else {
                println!("{:?} is not on map.", z);
            }
        }

        depth += 1;
    }
    

    

    unreachable!();
}

#[aoc(day15, part2)]
fn part2(program: &[isize]) -> usize {
    let mut comp = IntCode::new(program);
    let mut map : BTreeMap<(isize,isize), char> = BTreeMap::new();

    let mut cur = (0,0);

    let mut q = VecDeque::new();
    let dirs = &[Direction::South, Direction::North, Direction::East, Direction:: West ];

    for d in dirs {
        let (from, to) = (cur, d.new_coords(cur));
        q.push_back((d.reverse(), to, Phase::Backtrack, from));
        q.push_back((*d, from, Phase::Explore, to));
    }

    map.entry(cur).or_insert('d');

    let mut attempted_move = None;
    let mut oxygen = None;

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
                    assert_eq!(attempted_move, None);

                    if let Some(step) = q.pop_back() {
                        // print!("{:?} ", step);
                        let (dir, from, phase, to) = step;
                        assert_eq!(cur, from);
                        assert_eq!(to, dir.new_coords(from));
                        attempted_move = Some((dir, to));
                        // println!("{:?} -> {:?}", &cur, &attempted_move);
                        Some(dir.encode() as isize)
                    } else {
                        None
                    }
                }
                CallbackAction::WriteOutput(output) => {
                    match output {
                        0 => {
                            // wall
                            let (_dir, wall) = attempted_move.unwrap();
                            map.insert(wall, '#');
                            let _ = q.pop_back(); // remove backtrack
                            attempted_move = None;
                        },
                        1 | 2 => {
                            // moved
                            *map.get_mut(&cur).unwrap() = '.';
                            cur = attempted_move.unwrap().1;
                            *map.entry(cur).or_insert('d') = 'd';
                            attempted_move = None;

                            for d in dirs {
                                if None == map.get(&d.new_coords(cur)) {
                                    let (from, to) = (cur, d.new_coords(cur));
                                    q.push_back((d.reverse(), to, Phase::Backtrack, from));
                                    q.push_back((*d, from, Phase::Explore, to));
                                }
                            }

                            if output == 2 {
                                
                                oxygen = Some(cur);
                            }
                        },
                        _ => {
                            unreachable!();
                        }
                    };

                    None
                }
            }
        });

    let oxygen = oxygen.unwrap();
    println!("Current:{:?} -> Oxygen:{:?}", cur, oxygen);
    *map.get_mut(&oxygen).unwrap() = '.';

    assert_eq!((0,0), cur);
    *map.get_mut(&cur).unwrap() = '.';

    // we have the map
    let min_x = map.iter().map(|((x,_y),_c)| *x).min().unwrap();
    let max_x = map.iter().map(|((x,_y),_c)| *x).max().unwrap();
    let min_y = map.iter().map(|((_x,y),_c)| *y).min().unwrap();
    let max_y = map.iter().map(|((_x,y),_c)| *y).max().unwrap();

    let mut depth = 0;
    let mut seen = BTreeSet::new();
    let mut to_visit = Vec::new();
    let mut next = Vec::new();
    next.push(oxygen);
    while next.len() > 0 && map.iter().any(|(z, c)| *c == '.') {
        assert_eq!(0, to_visit.len());
        while let Some(x) = next.pop() {
            to_visit.push(x);
        }
        assert_eq!(0, next.len());

        println!("Depth: {} to_visit:{}", depth, to_visit.len());
        while let Some(z) = to_visit.pop() {

            if let Some(c) = map.get(&z) {

                if seen.insert(z) {
                    if c == &'.' {
                        for d in dirs {
                            next.push(d.new_coords(z));
                        }

                        *map.get_mut(&z).unwrap() = '@';
                    }
                }
            } else {
                println!("{:?} is not on map.", z);
            }
        }

        // for y in min_y..=max_y {
        //     for x in min_x..=max_x {
        //         print!("{}", map.get(&(x,y)).unwrap_or(&' '));
        //     }
        //     println!();
        // }
        // println!();

        depth += 1;
    }

    // 419 is too high
    depth - 1
}

