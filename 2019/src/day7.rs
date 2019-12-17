use aoc_runner_derive::{aoc, aoc_generator};
use std::num::ParseIntError;

use permute::permutations_of;

#[aoc_generator(day7)]
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

#[aoc(day7, part1)]
fn part1(input: &[isize]) -> isize {
    let mut best_thrust = -1;
    let phases : &[isize] = &[0, 1, 2, 3, 4];
    for permutation in permutations_of(phases) {
        let permutation : Vec<isize> = permutation.cloned().collect();
        let mut last_output = 0;

        for amp in 0..=4 {
            let mut memory : Vec<isize> = input.to_vec();
            let outputs = run(memory.as_mut_slice(), &[permutation[amp], last_output]);
            last_output = outputs[0];
        }

        best_thrust = std::cmp::max(best_thrust, last_output);
    }

    best_thrust
}

use std::sync::mpsc::{sync_channel, Receiver, SyncSender};

fn run2(memory: &mut [isize], input_chan: Receiver<isize>, output_chan: SyncSender<isize>) -> Vec<isize> {
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
                // output_chan. close??
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
                memory[out as usize] = input_chan.recv().expect("needed an input.");

                i += 2;
            },
            4 => {
                let in1 = memory[i+1];
                let in1 = if mode1 == 0 { memory[in1 as usize] } else { in1 };
                assert_eq!(0, mode2);
                assert_eq!(0, mode3);
                outputs.push(in1);
                let _ = output_chan.send(in1);//.expect("Couldn't send.");

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

use crossbeam_utils::thread;

#[aoc(day7, part2)]
fn part2(input: &[isize]) -> isize {
    let mut best_thrust = -1;
    let phases : &[isize] = &[5, 6, 7, 8, 9];
    for permutation in permutations_of(phases) {
        let permutation : Vec<isize> = permutation.cloned().collect();

        let mut sends = Vec::new();
        let mut recvs = Vec::new();
        for amp in 0..=4 {
            let (send, recv) = sync_channel(100);

            send.send(permutation[amp]).unwrap();

            if amp == 0 {
                send.send(0).unwrap();
            }

            sends.push(Some(send));
            recvs.push(Some(recv));
        }

        let thrust = thread::scope(|s| {
            let mut handles = Vec::new();
            for amp in 0..=4 {
                let mut recv_me = None;
                std::mem::swap(&mut recvs[amp], &mut recv_me);
                let recv_me = recv_me.unwrap();

                let mut send_next = None;
                std::mem::swap(&mut sends[(amp+1)%5], &mut send_next);
                let send_next = send_next.unwrap();

                handles.push(s.spawn(|_| {
                    let mut memory : Vec<isize> = input.to_vec();
                    let outputs = run2(memory.as_mut_slice(), recv_me, send_next);
                    *outputs.last().unwrap()
                }));
            }

            let mut final_outputs = Vec::new();
            for h in handles {
                final_outputs.push(h.join().unwrap());
            }
            *final_outputs.last().unwrap()
        }).unwrap();

        best_thrust = std::cmp::max(best_thrust, thrust);
    }

    best_thrust
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(43210, part1(&[3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0]));
        assert_eq!(54321, part1(&[3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0]));
        assert_eq!(65210, part1(&[3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0]));
    }

    #[test]
    fn part2_example() {
        assert_eq!(139629729, part2(&[3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5]));
        assert_eq!(18216, part2(&[3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,
            -5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,
            53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10]));
    }
}
