use aoc_runner_derive::{aoc, aoc_generator};
use std::num::ParseIntError;

#[aoc_generator(day9)]
fn parse_input(input: &str) -> Result<Vec<isize>, ParseIntError> {
    input.split(',').map(|l| l.parse()).collect()
}

use std::collections::VecDeque;

struct IntCode {
    memory: Vec<isize>,
    inputs: VecDeque<isize>,
    outputs: Vec<isize>,
    relative_base: isize,
    pc: usize,
}

impl IntCode {

    fn new(initial: &[isize], inputs: &[isize]) -> IntCode {
        let mut memory : Vec<isize> = initial.to_vec();
        memory.resize(memory.len()*100, 0);
        IntCode {
            memory,
            inputs: inputs.iter().cloned().collect(),
            outputs: Vec::new(),
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
    
    fn step(&mut self) -> Option<Vec<isize>> {
        let mut instruction = self.memory[self.pc];
        let opcode = instruction % 100; instruction /= 100;
        let mode1 = instruction % 10; instruction /= 10;
        let mode2 = instruction % 10; instruction /= 10;
        let mode3 = instruction % 10; instruction /= 10;
        assert_eq!(0, instruction);

        match opcode {
            99 => {
                Some(self.outputs.clone())
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
                None
            },
            3 => {
                let input = self.inputs.pop_front().expect("needed an input.");
                let out = self.memory[self.pc+1];
                let out = self.get_mut_ref(mode1, out);

                *out = input;

                self.pc += 2;
                None
            },
            4 => {
                let in1 = self.memory[self.pc+1];
                let in1 = self.get_value(mode1, in1);
                assert_eq!(0, mode2);
                assert_eq!(0, mode3);
                self.outputs.push(in1);

                self.pc += 2;
                None
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
                None
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
                None
            },
            9 => {
                let in1 = self.memory[self.pc+1];
                let in1 = self.get_value(mode1, in1);

                self.relative_base += in1;
                self.pc += 2;
                None
            }

            _ => {
                panic!("Unexpected opcode {} at {}.", opcode, self.pc);
            }
        }
    }
}

#[aoc(day9, part1)]
fn part1(program: &[isize]) -> isize {
    let mut comp = IntCode::new(program, &[1]);
    loop {
        // dbg!(comp.pc);
        if let Some(outputs) = comp.step() {
            assert_eq!(1, outputs.len());
            return outputs[0];
        }
    }
}

#[aoc(day9, part2)]
fn part2(program: &[isize]) -> isize {
    let mut comp = IntCode::new(program, &[2]);
    loop {
        // dbg!(comp.pc);
        if let Some(outputs) = comp.step() {
            assert_eq!(1, outputs.len());
            return outputs[0];
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(start: &[isize], inputs: &[isize], expected: &[isize]) {
        let mut comp = IntCode::new(start, inputs);
        loop {
            if let Some(outputs) = comp.step() {
                assert_eq!(expected, outputs.as_slice());
                break;
            }
        }
    }

    #[test]
    fn part1_example() {
        test(
            &[109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99],
            &[], 
            &[109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99]);

        test(
            &[1102,34915192,34915192,7,4,7,99,0],
            &[],
            &[1219070632396864]
        );

        test(
            &[104,1125899906842624,99],
            &[],
            &[1125899906842624]
        );
    }

    #[test]
    fn part2_example() {
        let program = &[3,9,8,9,10,9,4,9,99,-1,8];
        test(program, &[9], &[0]);
        test(program, &[8], &[1]);
        test(program, &[7], &[0]);

        let program = &[3,9,7,9,10,9,4,9,99,-1,8];
        test(program, &[8], &[0]);
        test(program, &[8], &[0]);
        test(program, &[7], &[1]);
        test(program, &[6], &[1]);

        let program = &[3,3,1108,-1,8,3,4,3,99];
        test(program, &[9], &[0]);
        test(program, &[8], &[1]);
        test(program, &[7], &[0]);

        let program = &[3,3,1107,-1,8,3,4,3,99];
        test(program, &[8], &[0]);
        test(program, &[8], &[0]);
        test(program, &[7], &[1]);
        test(program, &[6], &[1]);
    }

    #[test]
    fn part2_example_jmp() {
        let program = &[3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9];
        test(program, &[0], &[0]);
        test(program, &[1], &[1]);
        test(program, &[2], &[1]);

        let program = &[3,3,1105,-1,9,1101,0,0,12,4,12,99,1];
        test(program, &[0], &[0]);
        test(program, &[1], &[1]);
        test(program, &[2], &[1]);

        let program = &[3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,
        1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,
        999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99];
        test(program, &[7], &[999]);
        test(program, &[8], &[1000]);
        test(program, &[9], &[1001]);

    }
}
