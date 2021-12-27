use std::collections::VecDeque;

use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Clone)]
struct BitStream(VecDeque<bool>);

impl BitStream {
    fn read_bits(&mut self, n: u8) -> u64 {
        assert!(n <= 64);
        let mut bits = 0;
        for _ in 0..n {
            bits <<= 1;
            bits |= if self.0.pop_front().unwrap() { 1 } else { 0 }
        }
        // dbg!(n,bits);
        bits
    }

    fn discard_to_nibble(&mut self) {
        while self.0.len() % 4 != 0 {
            let _ = self.0.pop_front();
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Packet {
    version: u8,
    content: PacketContent
}

#[derive(Debug, PartialEq, Eq)]
enum PacketContent {
    Literal(Vec<u8>),
    Operator(u8, Vec<Packet>),
}

impl Packet {
    fn read(bs: &mut BitStream) -> Packet {
        let version = bs.read_bits(3).try_into().unwrap();
        let t = bs.read_bits(3);
        let content = match t {
            4 => {
                let mut bytes = Vec::new();
                loop {
                    let byte = bs.read_bits(5) as u8;
                    bytes.push(byte & 0xF);
                    if byte & 0x10 == 0 {
                        break;
                    }
                }
                PacketContent::Literal(bytes)
            }
            op => {
                let mut subpackets = Vec::new();

                let legth_type = bs.read_bits(1);
                match legth_type {
                    0 => {
                        let bits_to_read = bs.read_bits(15) as usize;
                        let end_len = bs.0.len() - bits_to_read;
                        while bs.0.len() > end_len {
                            subpackets.push(Packet::read(bs));
                        }
        
                        assert_eq!(bs.0.len(), end_len);
                    }
                    1 => {
                        let subpackets_to_read = bs.read_bits(11);
                        for _ in 0..subpackets_to_read {
                            subpackets.push(Packet::read(bs));
                        }

                    }
                    _ => panic!(),
                }

                PacketContent::Operator(op.try_into().unwrap(), subpackets)
            }
        };
        Packet { version, content}
    }
}


#[aoc_generator(day16)]
fn parse_input(input: &str) -> BitStream {
    let mut bits = VecDeque::new();
    for line in  input.trim().lines() {
        let line = line.trim();
        let line: Vec<_> = line.chars().collect();
        for nibbles in line.chunks(2) {
            let byte = nibbles[0].to_digit(16).unwrap();
            let byte = 16 * byte;
            let byte = byte + nibbles[1].to_digit(16).unwrap();
            let byte: u8 = byte.try_into().unwrap();
            for i in (0..8).rev() {
                bits.push_back(if (byte >> i) & 0x1 == 1 { true } else { false });
            }
        }
    }

    BitStream(bits)
}

fn sum_versions(p: &Packet) -> u64 {
    p.version as u64 + match &p.content {
        PacketContent::Operator(_, subpackets) => {
            subpackets.iter().map(|p| sum_versions(p)).sum()
        }
        _ => 0
    }
}

#[aoc(day16, part1)]
fn part1(bits: &BitStream) -> u64 { 
    let mut bits: BitStream = bits.clone();
    let p = Packet::read(&mut bits);
    sum_versions(&p)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example1() {
        let mut bs = parse_input("D2FE28");
        let p = Packet::read(&mut bs);
        assert_eq!(p, Packet { 
            version: 6, 
            content: PacketContent::Literal(vec![0b0111, 0b1110, 0b0101])});
    }

    #[test]
    fn part1_example2() {
        let mut bs = parse_input("38006F45291200");
        let p = Packet::read(&mut bs);
        assert_eq!(p, Packet { 
            version: 1, 
            content: PacketContent::Operator(
                6,
                vec![
                    Packet { version: 6, content: PacketContent::Literal(vec![0b1010])},
                    Packet { version: 2, content: PacketContent::Literal(vec![0b0001, 0b0100])},
                ]
            )
        });
    }

    #[test]
    fn part1_example3() {
        let mut bs = parse_input("EE00D40C823060");
        let p = Packet::read(&mut bs);
        assert_eq!(p, Packet { 
            version: 7, 
            content: PacketContent::Operator(
                3,
                vec![
                    Packet { version: 2, content: PacketContent::Literal(vec![0b0001])},
                    Packet { version: 4, content: PacketContent::Literal(vec![0b0010])},
                    Packet { version: 1, content: PacketContent::Literal(vec![0b0011])},
                ]
            )
        });
    }

    #[test]
    fn part1_example4() {
        let bs = parse_input("8A004A801A8002F478");
        assert_eq!(16, part1(&bs));
    }

    #[test]
    fn part1_example5() {
        let bs = parse_input("620080001611562C8802118E34");
        assert_eq!(12, part1(&bs));
    }

    #[test]
    fn part1_example6() {
        let bs = parse_input("C0015000016115A2E0802F182340");
        assert_eq!(23, part1(&bs));
    }

    #[test]
    fn part1_example7() {
        let bs = parse_input("A0016C880162017C3686B18A3D4780");
        assert_eq!(31, part1(&bs));
    }
}