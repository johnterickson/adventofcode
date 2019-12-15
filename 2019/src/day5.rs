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
            1 | 2 => {
                let in1 = memory[i+1];
                let in1 = if mode1 == 0 { memory[in1 as usize] } else { in1 };
                let in2 = memory[i+2];
                let in2 = if mode2 == 0 { memory[in2 as usize] } else { in2 };
                let out = memory[i+3];
                assert_eq!(0, mode3);

                match opcode {
                    1 => { memory[out as usize] = in1 + in2; }
                    2 => { memory[out as usize] = in1 * in2; }
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
            }

            i => {
                panic!("Unexpected opcode {}.", i);
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

    unimplemented!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let test = | start: &[isize], inputs: &[isize], end: &[isize], expected: &[isize] | {
            let mut mem : Vec<isize> = start.to_vec();
            let outputs = run(mem.as_mut_slice(), inputs);
            assert_eq!(end, mem.as_slice());
            assert_eq!(expected, outputs.as_slice());
        };

        test(&[1002,4,3,4,33], &[], &[1002,4,3,4,99], &[]);
    }

    #[test]
    fn part2_example() {
    }
}
