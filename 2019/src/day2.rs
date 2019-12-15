use aoc_runner_derive::{aoc, aoc_generator};
use std::num::ParseIntError;

#[aoc_generator(day2)]
fn parse_input(input: &str) -> Result<Vec<usize>, ParseIntError> {
    input.split(",").map(|l| l.parse()).collect()
}

fn run(memory: &mut [usize]) {
    let mut i = 0;
    while memory[i] != 99 {
        let in1 = memory[i+1];
        let in2 = memory[i+2];
        let out = memory[i+3];
        match memory[i] {
            1 => {
                memory[out] = memory[in1] + memory[in2];
            },
            2 => {
                memory[out] = memory[in1] * memory[in2];
            }
            i => {
                panic!("Unexpected opcode {}.", i);
            }
        }

        i += 4;
    }
}

#[aoc(day2, part1)]
fn part1(input: &[usize]) -> usize {
    let mut memory : Vec<usize> = input.iter().cloned().collect();
    memory[1] = 12;
    memory[2] = 2;
    run(memory.as_mut_slice());
    memory[0]
}

#[aoc(day2, part2)]
fn part2(input: &[usize]) -> usize {
    for noun in 0..=99 {
        for verb in 0..=99 {
            let mut memory : Vec<usize> = input.iter().cloned().collect();
            memory[1] = noun;
            memory[2] = verb;
            run(memory.as_mut_slice());
            if memory[0] == 19690720 {
                return 100*noun + verb;
            }
        }
    }

    unreachable!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let test = | start: &[usize], end: &[usize] | {
            let mut mem : Vec<usize> = start.to_vec();
            run(mem.as_mut_slice());
            assert_eq!(end, mem.as_slice());
        };

        test(&[1, 0, 0, 0, 99], &[2,0,0,0,99]);
        test(&[2,3,0,3,99], &[2,3,0,6,99]);
        test(&[1,1,1,4,99,5,6,0,99], &[30,1,1,4,2,5,6,0,99]);
    }

    #[test]
    fn part2_example() {
    }
}
