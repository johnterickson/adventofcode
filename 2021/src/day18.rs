use std::{str::Chars, iter::Peekable, fmt::Display, collections::VecDeque};

use aoc_runner_derive::{aoc, aoc_generator};

struct NumPair(Option<Rc<NumPair>>, Num, Num);

impl NumPair {
    fn parse(mut chars: &mut Peekable<Chars>) -> NumPair {
        assert_eq!(Some('['), chars.next());
        let left = Num::parse(&mut chars);
        assert_eq!(Some(','), chars.next());
        let right = Num::parse(&mut chars);
        assert_eq!(Some(']'), chars.next());
        NumPair(None, left,right)
    }

    fn try_add_to_leftmost(&mut self, add: u64) -> bool {
        match self.0 {
            Num::Literal(ref mut n) => {
                *n += add;
                true
            }
            Num::Pair(ref mut inner) => inner.try_add_to_leftmost(add),
        }
    }

    fn explode(&mut self) -> bool {

        let mut parents = VecDeque::new();
        parents.push_back(self);

        loop {
            if parents.len() > 4 {
                
            }
        }

        false
    }
}

impl Display for NumPair {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"[{},{}]", self.0, self.1)
    }
}

enum Num {
    Literal(u64),
    Pair(Box<NumPair>)
}

impl Num {
    fn parse(chars: &mut Peekable<Chars>) -> Num {
        if chars.peek().unwrap().is_digit(10) {
            let mut n = 0u64;
            loop {
                let digit = if let Some(c) = chars.peek() {
                    c.to_digit(10)
                } else {
                    None
                };

                if let Some(digit) = digit {
                    let digit: u64 = digit.into();
                    let _ = chars.next().unwrap();
                    n *= 10;
                    n = n + digit;
                    continue;
                }

                break Num::Literal(n);
            }
        } else {
            Num::Pair(Box::new(NumPair::parse(chars)))
        }
    }

    fn explode_inner(&mut self, depth: usize) -> (Option<u64>, Option<u64>) {
        if let Num::Literal(_) = self {
            return (None, None);
        }

        if depth == 4 {
            let exploded = std::mem::replace(self, Num::Literal(0));
            let exploded = match exploded {
                Num::Pair(pair) => *pair,
                _ => panic!(),
            };
            let left = match exploded.0 {
                Num::Literal(n) => n,
                _ => panic!(),
            };
            let right = match exploded.0 {
                Num::Literal(n) => n,
                _ => panic!(),
            };
            return (Some(left), Some(right));
        }

        match self {
            Num::Literal(_) => panic!(),
            Num::Pair(ref mut pair) => {
                pair.explode_inner(depth+1)
            },
        }
    }
}

impl Display for Num {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Num::Literal(n) => write!(f,"{}", n),
            Num::Pair(pair) => write!(f,"{}", pair),
        }
        
    }
}

#[aoc_generator(day18)]
fn parse_input(input: &str) -> Vec<NumPair> {
    input.trim().lines().map(|line| {
        let mut chars = line.trim().chars().peekable();
        NumPair::parse(&mut chars)
    }).collect()
}

#[aoc(day18, part1)]
fn part2(pairs: &Vec<NumPair>) -> usize { 
    todo!();
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fmt::Write;

    #[test]
    fn part1_examples() {
       let nums =
           "[1,2]
           [[1,2],3]
           [9,[8,7]]
           [[1,9],[8,5]]
           [[[[1,2],[3,4]],[[5,6],[7,8]]],9]
           [[[9,[3,8]],[[0,9],6]],[[[3,7],[4,9]],3]]
           [[[[1,3],[5,3]],[[1,3],[8,7]]],[[[4,9],[6,9]],[[8,2],[7,3]]]]";
        let parsed = parse_input(nums);
        for (parsed, expected) in parsed.iter().zip(nums.lines()) {
            let mut formatted = String::new();
            write!(&mut formatted, "{}", parsed).unwrap();
            assert_eq!(formatted.as_str(), expected.trim());
        }
    }
}