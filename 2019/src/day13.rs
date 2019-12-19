use aoc_runner_derive::{aoc, aoc_generator};
use std::num::ParseIntError;

#[aoc_generator(day13)]
fn parse_input(input: &str) -> Result<Vec<isize>, ParseIntError> {
    input.split(',').map(|l| l.parse()).collect()
}

use std::collections::BTreeMap;

struct IntCode {
    memory: Vec<isize>,
    relative_base: isize,
    pc: usize,
}

#[derive(Debug)]
enum CallbackAction {
    ReadInput,
    WriteOutput(isize),
}

impl IntCode {

    fn new(initial: &[isize]) -> IntCode {
        let mut memory : Vec<isize> = initial.to_vec();
        memory.resize(memory.len()*100, 0);
        IntCode {
            memory,
            relative_base: 0,
            pc: 0,
        }
    }

    fn get_value(&self, mode: isize, value: isize) -> isize {
        match mode {
            0 => self.memory[value as usize],
            1 => value,
            2 => self.memory[(self.relative_base + value) as usize],
            i => panic!("unimplemented mode {}", i)
        }
    }

    fn get_mut_ref(&mut self, mode: isize, value: isize) -> &mut isize {
        match mode {
            0 => &mut self.memory[value as usize],
            1 => panic!("can't write to a constant"),
            2 => &mut self.memory[(self.relative_base + value) as usize],
            i => panic!("unimplemented mode {}", i)
        }
    }
    
    fn run<F: FnMut(CallbackAction) -> Option<isize>>(&mut self, mut callback: F) -> () {
        loop {
            let mut instruction = self.memory[self.pc];
            let opcode = instruction % 100; instruction /= 100;
            let mode1 = instruction % 10; instruction /= 10;
            let mode2 = instruction % 10; instruction /= 10;
            let mode3 = instruction % 10; instruction /= 10;
            assert_eq!(0, instruction);

            match opcode {
                99 => {
                    return;
                },
                1 | 2 | 7 | 8 => {
                    let in1 = self.memory[self.pc+1];
                    let in1 = self.get_value(mode1, in1);
                    let in2 = self.memory[self.pc+2];
                    let in2 = self.get_value(mode2, in2);
                    let out = self.memory[self.pc+3];
                    let out = self.get_mut_ref(mode3, out);

                    match opcode {
                        1 => { *out = in1 + in2; }
                        2 => { *out = in1 * in2; }
                        7 => { *out = if in1 < in2 { 1 } else { 0 } }
                        8 => { *out = if in1 == in2 { 1 } else { 0 } }
                        _ => unreachable!(),
                    }

                    self.pc += 4;
                },
                3 => {
                    let input = (callback)(CallbackAction::ReadInput).unwrap();
                    let out = self.memory[self.pc+1];
                    let out = self.get_mut_ref(mode1, out);

                    *out = input;

                    self.pc += 2;
                },
                4 => {
                    let in1 = self.memory[self.pc+1];
                    let in1 = self.get_value(mode1, in1);
                    assert_eq!(0, mode2);
                    assert_eq!(0, mode3);
                    let _ = (callback)(CallbackAction::WriteOutput(in1));

                    self.pc += 2;
                },
                5 => {
                    let in1 = self.memory[self.pc+1];
                    let in1 = self.get_value(mode1, in1);
                    let in2 = self.memory[self.pc+2];
                    let in2 = self.get_value(mode2, in2);
                    if in1 != 0 {
                        self.pc = in2 as usize;
                    } else {
                        self.pc += 3;
                    }
                },
                6 => {
                    let in1 = self.memory[self.pc+1];
                    let in1 = self.get_value(mode1, in1);
                    let in2 = self.memory[self.pc+2];
                    let in2 = self.get_value(mode2, in2);

                    if in1 == 0 {
                        self.pc = in2 as usize;
                    } else {
                        self.pc += 3;
                    }
                },
                9 => {
                    let in1 = self.memory[self.pc+1];
                    let in1 = self.get_value(mode1, in1);

                    self.relative_base += in1;
                    self.pc += 2;
                }

                _ => {
                    panic!("Unexpected opcode {} at {}.", opcode, self.pc);
                }
            }
        }
    }
}

#[derive(Clone,Copy,PartialEq)]
enum Tile {
    Empty,
    Wall,
    Block,
    Paddle,
    Ball
}

impl Tile {
    fn from(i: &isize) -> Tile {
        match i {
            0 => Tile::Empty,
            1 => Tile::Wall,
            2 => Tile::Block,
            3 => Tile::Paddle,
            4 => Tile::Ball,
            i => panic!(format!("Unknown tile {}", i)),
        }
    }
}

impl std::fmt::Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "{}", match self {
            Tile::Empty => ' ',
            Tile::Wall => '+',
            Tile::Block => 'X',
            Tile::Paddle => '_',
            Tile::Ball => '*',
        })
    }
}

#[aoc(day13, part1)]
fn part1(program: &[isize]) -> usize {
    let mut comp = IntCode::new(program);

    let mut panels = BTreeMap::new();

    let (mut x, mut y) = (None, None);

    comp.run(
        |action: CallbackAction| {
            // println!("{:?} {},{} facing {}", action, x, y, dir);
            match action {
                CallbackAction::ReadInput => {
                    unimplemented!();
                }
                CallbackAction::WriteOutput(output) => {
                    if x.is_none() {
                        x = Some(output);
                    } else if y.is_none() {
                        y = Some(output);
                    } else {
                        let tile = Tile::from(&output);

                        *panels.entry((x.unwrap(),y.unwrap())).or_insert(tile) = tile;

                        x = None;
                        y = None;
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
            print!("{}", panels.get(&(x,y)).unwrap_or(&Tile::Empty));
        }
        println!();
    }

    // 9621 is too high
    panels.values().filter(|v| **v == Tile::Block).count()
}

use std::io::{Write, stdout};
use crossterm::{ExecutableCommand, cursor};

#[aoc(day13, part2)]
fn part2(program: &[isize]) -> isize {
    let mut program = program.to_vec();
    program[0] = 2;

    let mut comp = IntCode::new(&program);

    let mut panels = BTreeMap::new();

    let (mut x, mut y) = (None, None);

    let mut ball_x = None;
    let mut paddle_x = None;

    let mut score = None;

    let mut stdout = stdout();

    let mut input_requested = false;

    comp.run(
        |action: CallbackAction| {
            // println!("{:?} {},{} facing {}", action, x, y, dir);
            match action {
                CallbackAction::ReadInput => {
                    input_requested = true;

                    stdout
                        .execute(cursor::SavePosition).unwrap()
                        .execute(cursor::MoveTo(0, 21)).unwrap();

                    let paddle_move = if let (Some(ball_x), Some(paddle_x)) = (ball_x, paddle_x) {
                        print!("Because ball_x is {} and paddle_x is {}.", ball_x, paddle_x);

                        if paddle_x < ball_x {
                            Some(1)
                        } else if paddle_x == ball_x {
                            Some(0)
                        } else {
                            Some(-1)
                        }
                    } else {
                        print!("Because ball_x or paddle_x is unknown.");
                        Some(0)
                    };
                    println!(" Move is {:?}", paddle_move);

                    stdout
                        .execute(cursor::RestorePosition).unwrap();

                    paddle_move
                }
                CallbackAction::WriteOutput(output) => {
                    if x.is_none() {
                        x = Some(output);
                    } else if y.is_none() {
                        y = Some(output);
                    } else {
                        let (tile_x, tile_y) = (x.unwrap(), y.unwrap());

                        if (-1,0) == (tile_x, tile_y) {
                            stdout
                                .execute(cursor::SavePosition).unwrap()
                                .execute(cursor::MoveTo(0, 20)).unwrap();
                            score = Some(output);
                            println!("Score is: {}", output);
                            stdout.execute(cursor::RestorePosition).unwrap();

                        } else {
                            let tile = Tile::from(&output);

                            if tile == Tile::Ball {
                                ball_x = Some(tile_x);
                            } else if tile == Tile::Paddle {
                                paddle_x = Some(tile_x);
                            }

                            *panels.entry((tile_x, tile_y)).or_insert(tile) = tile;

                            stdout
                                .execute(cursor::SavePosition).unwrap()
                                .execute(cursor::MoveTo(tile_x as u16, tile_y as u16)).unwrap();
                            print!("{}", &tile);
                            stdout.execute(cursor::RestorePosition).unwrap();

                            if input_requested {
                                // std::thread::sleep(std::time::Duration::from_millis(10));
                            }
                        }

                        x = None;
                        y = None;
                    }
                    None
                }
            }
        });

    score.unwrap()
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
