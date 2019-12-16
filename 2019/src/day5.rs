use aoc_runner_derive::{aoc, aoc_generator};
use std::num::ParseIntError;

#[aoc_generator(day5)]
fn parse_input(input: &str) -> Result<Vec<isize>, ParseIntError> {
    input.split(",").map(|l| l.parse()).collect()
}

fn run(memory: &mut [isize], inputs: &[isize]) -> Vec<isize> {
    let mut inputs = inputs.iter();
    let mut outputs = Vec::new();
    let mut i = 0;
    loop {
        let mut instruction = memory[i];
        let opcode = instruction % 100; instruction /= 100;
        let mode1 = instruction % 10; instruction /= 10;
        let mode2 = instruction % 10; instruction /= 10;
        let mode3 = instruction % 10; instruction /= 10;
        assert_eq!(0, instruction);

        match opcode {
            99 => {
                return outputs;
            },
            1 | 2 | 7 | 8=> {
                let in1 = memory[i+1];
                let in1 = if mode1 == 0 { memory[in1 as usize] } else { in1 };
                let in2 = memory[i+2];
                let in2 = if mode2 == 0 { memory[in2 as usize] } else { in2 };
                let out = memory[i+3];
                assert_eq!(0, mode3);

                match opcode {
                    1 => { memory[out as usize] = in1 + in2; }
                    2 => { memory[out as usize] = in1 * in2; }
                    7 => { memory[out as usize] = if in1 < in2 { 1 } else { 0 } }
                    8 => { memory[out as usize] = if in1 == in2 { 1 } else { 0 } }
                    _ => unreachable!(),
                }

                i += 4;
            },
            3 => {
                assert_eq!(0, mode1);
                assert_eq!(0, mode2);
                assert_eq!(0, mode3);

                let out = memory[i+1];
                memory[out as usize] = *inputs.next().expect("needed an input.");

                i += 2;
            },
            4 => {
                let in1 = memory[i+1];
                let in1 = if mode1 == 0 { memory[in1 as usize] } else { in1 };
                assert_eq!(0, mode2);
                assert_eq!(0, mode3);
                outputs.push(in1);

                i += 2;
            },
            5 => {
                let in1 = memory[i+1];
                let in1 = if mode1 == 0 { memory[in1 as usize] } else { in1 };
                let in2 = memory[i+2];
                let in2 = if mode2 == 0 { memory[in2 as usize] } else { in2 };

                if in1 != 0 {
                    i = in2 as usize;
                } else {
                    i += 3;
                }
            },
            6 => {
                let in1 = memory[i+1];
                let in1 = if mode1 == 0 { memory[in1 as usize] } else { in1 };
                let in2 = memory[i+2];
                let in2 = if mode2 == 0 { memory[in2 as usize] } else { in2 };

                if in1 == 0 {
                    i = in2 as usize;
                } else {
                    i += 3;
                }
            },

            _ => {
                panic!("Unexpected opcode {} at {}.", opcode, i);
            }
        }
    }
}

#[aoc(day5, part1)]
fn part1(input: &[isize]) -> isize {
    let mut memory : Vec<isize> = input.iter().cloned().collect();
    let outputs = run(memory.as_mut_slice(), &[1]);
    for i in 0..(outputs.len()-2) {
        assert_eq!(0, outputs[i]);
    }
    *outputs.last().unwrap()
}

#[aoc(day5, part2)]
fn part2(input: &[isize]) -> isize {
    let mut memory : Vec<isize> = input.iter().cloned().collect();
    let outputs = run(memory.as_mut_slice(), &[5]);
    assert_eq!(1, outputs.len());
    outputs[0]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(start: &[isize], inputs: &[isize], end: Option<&[isize]>, expected: &[isize]) {
        let mut mem : Vec<isize> = start.to_vec();
        let outputs = run(mem.as_mut_slice(), inputs);
        if let Some(end) = end {
            assert_eq!(end, mem.as_slice());
        }
        assert_eq!(expected, outputs.as_slice());
    }
    #[test]
    fn part1_example() {
        test(&[1002,4,3,4,33], &[], Some(&[1002,4,3,4,99]), &[]);
    }

    #[test]
    fn part2_example() {
        let program = &[3,9,8,9,10,9,4,9,99,-1,8];
        test(program, &[9], None, &[0]);
        test(program, &[8], None, &[1]);
        test(program, &[7], None, &[0]);

        let program = &[3,9,7,9,10,9,4,9,99,-1,8];
        test(program, &[8], None, &[0]);
        test(program, &[8], None, &[0]);
        test(program, &[7], None, &[1]);
        test(program, &[6], None, &[1]);

        let program = &[3,3,1108,-1,8,3,4,3,99];
        test(program, &[9], None, &[0]);
        test(program, &[8], None, &[1]);
        test(program, &[7], None, &[0]);

        let program = &[3,3,1107,-1,8,3,4,3,99];
        test(program, &[8], None, &[0]);
        test(program, &[8], None, &[0]);
        test(program, &[7], None, &[1]);
        test(program, &[6], None, &[1]);
    }

    #[test]
    fn part2_example_jmp() {
        let program = &[3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9];
        test(program, &[0], None, &[0]);
        test(program, &[1], None, &[1]);
        test(program, &[2], None, &[1]);

        let program = &[3,3,1105,-1,9,1101,0,0,12,4,12,99,1];
        test(program, &[0], None, &[0]);
        test(program, &[1], None, &[1]);
        test(program, &[2], None, &[1]);

        let program = &[3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,
        1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,
        999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99];
        test(program, &[7], None, &[999]);
        test(program, &[8], None, &[1000]);
        test(program, &[9], None, &[1001]);

    }
}
