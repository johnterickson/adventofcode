use aoc_runner_derive::{aoc, aoc_generator};
use std::num::ParseIntError;
use std::collections::BTreeMap;

use crate::intcode::*;

#[aoc_generator(day11)]
fn parse_input(input: &str) -> Result<Vec<isize>, ParseIntError> {
    input.split(',').map(|l| l.parse()).collect()
}

#[aoc(day11, part1)]
fn part1(program: &[isize]) -> usize {
    let mut comp = IntCode::new(program);

    let mut output_color = None;
    let mut output_turn = None;

    let mut panels = BTreeMap::new();

    let directions = [
        (0,-1), //up
        (1,0), //right
        (0,1), //down
        (-1,0), //left
    ];

    let mut dir = 0; // up
    let (mut x, mut y) = (0,0);

    comp.run(
        |action: CallbackAction| {
            // println!("{:?} {},{} facing {}", action, x, y, dir);
            match action {
                CallbackAction::ReadInput => {
                    Some(*panels.get(&(x,y)).unwrap_or(&0))
                }
                CallbackAction::WriteOutput(output) => {
                    if output_color.is_none() {
                        output_color = Some(output);
                    } else {
                        output_turn = Some(output);
                    }

                    if let (Some(color), Some(turn)) = (output_color, output_turn) {
                        panels.insert((x,y), color);
                        // print!("{},{} is now painted {}. Facing {}. turning {}.", x,y,color, dir, turn);

                        if turn == 0 {
                            dir += 3;
                        } else {
                            dir += 1;
                        }
                        dir %= 4;
                        x += directions[dir].0;
                        y += directions[dir].1;

                        // println!(" Now at {},{} facing {}", x, y, dir);

                        output_color = None;
                        output_turn = None;
                    }
                    None
                }
            }
        });

    let min_x = panels.iter().map(|((x,_y),_c)| *x).min().unwrap();
    let max_x = panels.iter().map(|((x,_y),_c)| *x).max().unwrap();
    let min_y = panels.iter().map(|((_x,y),_c)| *y).min().unwrap();
    let max_y = panels.iter().map(|((_x,y),_c)| *y).max().unwrap();

    dbg!((min_x,max_x,min_y,max_y));

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            print!("{}", if *panels.get(&(x,y)).unwrap_or(&0) == 0 { ' ' } else { '#' });
        }
        println!();
    }

    // 9621 is too high
    panels.len()
}

#[aoc(day11, part2)]
fn part2(program: &[isize]) -> usize {
    let mut comp = IntCode::new(program);

    let mut output_color = None;
    let mut output_turn = None;

    let mut panels = BTreeMap::new();

    let directions = [
        (0,-1), //up
        (1,0), //right
        (0,1), //down
        (-1,0), //left
    ];

    let mut dir = 0; // up
    let (mut x, mut y) = (0,0);

    panels.insert((x,y), 1);

    comp.run(
        |action: CallbackAction| {
            // println!("{:?} {},{} facing {}", action, x, y, dir);
            match action {
                CallbackAction::ReadInput => {
                    Some(*panels.get(&(x,y)).unwrap_or(&0))
                }
                CallbackAction::WriteOutput(output) => {
                    if output_color.is_none() {
                        output_color = Some(output);
                    } else {
                        output_turn = Some(output);
                    }

                    if let (Some(color), Some(turn)) = (output_color, output_turn) {
                        panels.insert((x,y), color);
                        // print!("{},{} is now painted {}. Facing {}. turning {}.", x,y,color, dir, turn);

                        if turn == 0 {
                            dir += 3;
                        } else {
                            dir += 1;
                        }
                        dir %= 4;
                        x += directions[dir].0;
                        y += directions[dir].1;

                        // println!(" Now at {},{} facing {}", x, y, dir);

                        output_color = None;
                        output_turn = None;
                    }
                    None
                }
            }
        });

    let min_x = panels.iter().map(|((x,_y),_c)| *x).min().unwrap();
    let max_x = panels.iter().map(|((x,_y),_c)| *x).max().unwrap();
    let min_y = panels.iter().map(|((_x,y),_c)| *y).min().unwrap();
    let max_y = panels.iter().map(|((_x,y),_c)| *y).max().unwrap();

    dbg!((min_x,max_x,min_y,max_y));

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            print!("{}", if *panels.get(&(x,y)).unwrap_or(&0) == 0 { ' ' } else { '#' });
        }
        println!();
    }

    // 9621 is too high
    panels.len()
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     fn test(start: &[isize], inputs: &[isize], expected: &[isize]) {
//         let mut comp = IntCode::new(start, inputs);
//         loop {
//             if let Some(outputs) = comp.step() {
//                 assert_eq!(expected, outputs.as_slice());
//                 break;
//             }
//         }
//     }

//     #[test]
//     fn part1_example() {
//         test(
//             &[109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99],
//             &[], 
//             &[109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99]);

//         test(
//             &[1102,34915192,34915192,7,4,7,99,0],
//             &[],
//             &[1219070632396864]
//         );

//         test(
//             &[104,1125899906842624,99],
//             &[],
//             &[1125899906842624]
//         );
//     }

//     #[test]
//     fn part2_example() {
//         let program = &[3,9,8,9,10,9,4,9,99,-1,8];
//         test(program, &[9], &[0]);
//         test(program, &[8], &[1]);
//         test(program, &[7], &[0]);

//         let program = &[3,9,7,9,10,9,4,9,99,-1,8];
//         test(program, &[8], &[0]);
//         test(program, &[8], &[0]);
//         test(program, &[7], &[1]);
//         test(program, &[6], &[1]);

//         let program = &[3,3,1108,-1,8,3,4,3,99];
//         test(program, &[9], &[0]);
//         test(program, &[8], &[1]);
//         test(program, &[7], &[0]);

//         let program = &[3,3,1107,-1,8,3,4,3,99];
//         test(program, &[8], &[0]);
//         test(program, &[8], &[0]);
//         test(program, &[7], &[1]);
//         test(program, &[6], &[1]);
//     }

//     #[test]
//     fn part2_example_jmp() {
//         let program = &[3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9];
//         test(program, &[0], &[0]);
//         test(program, &[1], &[1]);
//         test(program, &[2], &[1]);

//         let program = &[3,3,1105,-1,9,1101,0,0,12,4,12,99,1];
//         test(program, &[0], &[0]);
//         test(program, &[1], &[1]);
//         test(program, &[2], &[1]);

//         let program = &[3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,
//         1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,
//         999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99];
//         test(program, &[7], &[999]);
//         test(program, &[8], &[1000]);
//         test(program, &[9], &[1001]);

//     }
// }
