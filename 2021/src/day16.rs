use std::collections::VecDeque;

use aoc_runner_derive::{aoc, aoc_generator};

struct Packet {
    version: u8,
}

impl Packet {
    fn read(q: &mut VecDeque<u8>) -> Packet {
        let mut version = 0;
        for _ in 0..3 {
            version <<= 1;
            version |= q.pop_front().unwrap();
        }

        Packet { version }
    }
}

enum Type {
    Literal = 4,
}

impl Type {
    fn read(q: &mut VecDeque<u8>) -> Type {
        let mut bits = 0;
        for _ in 0..3 {
            bits <<= 1;
            bits |= q.pop_front().unwrap();
        }

        match bits {
            4 => Type::Literal,
            _ => panic!(),
        }
    }
}

#[aoc_generator(day16)]
fn parse_input(input: &str) -> VecDeque<u8> {
    let mut bits = Vec::new();
    for line in  input.trim().lines() {
        let line = line.trim();
        let line: Vec<_> = line.chars().collect();
        for nibbles in line.chunks(2) {
            let byte = nibbles[0].to_digit(16).unwrap();
            let byte = 16 * byte;
            let byte = byte + nibbles[1].to_digit(16).unwrap();
            let byte: u8 = byte.try_into().unwrap();
            for i in (0..8).rev() {
                bits.push((byte >> i) & 0x1);
            }
        }
    }

    bits
}

#[aoc(day16, part1)]
fn part1(bits: &VecDeque<u8>) -> u64 { 
    todo!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example1() {
        todo!();
    }
}