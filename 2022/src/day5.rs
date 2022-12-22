use std::collections::VecDeque;

use aoc_runner_derive::{aoc, aoc_generator};

type CrateStack = VecDeque<char>;

#[derive(Debug)]
struct Move {
    count: usize,
    from: usize,
    to: usize,
}

type Input = (Vec<CrateStack>, Vec<Move>);

#[aoc_generator(day5)]
fn parse_input(input: &str) -> Input {
    let mut lines = input.lines().into_iter().peekable();
    if lines.peek().unwrap().trim().is_empty() {
        let _ = lines.next().unwrap();
    }
    let mut stacks = Vec::new();
    while let Some(line) = lines.next() {
        // let line = line.trim();
        if line.trim().is_empty() {
            break;
        }
        for c in line.char_indices() {
            if c.1.is_alphabetic() {
                let stack_index = (c.0 - 1) / 4;
                while stack_index >= stacks.len() {
                    stacks.push(CrateStack::new());
                }

                stacks[stack_index].push_front(c.1);
            }
        }
    }

    // dbg!(&stacks);

    let mut moves = Vec::new();
    for line in lines {
        let line = line.trim();
        if line.trim().is_empty() {
            break;
        }
        let mut tokens = line.split_whitespace();
        assert_eq!(Some("move"), tokens.next());
        let count = tokens.next().unwrap().parse().unwrap();
        assert_eq!(Some("from"), tokens.next());
        let from = tokens.next().unwrap().parse().unwrap();
        assert_eq!(Some("to"), tokens.next());
        let to = tokens.next().unwrap().parse().unwrap();
        assert_eq!(None, tokens.next());
        moves.push(Move { count, from, to });
    }

    (stacks, moves)
}

#[aoc(day5, part1)]
fn part1(pairs: &Input) -> String {
    let mut stacks = pairs.0.clone();

    for m in &pairs.1 {
        // dbg!(&stacks, &m);
        for _ in 0..m.count {
            let c = stacks[m.from - 1].pop_back().unwrap();
            stacks[m.to - 1].push_back(c);
        }
    }

    stacks.iter().map(|s| s.back().unwrap()).collect()
}

#[aoc(day5, part2)]
fn part2(pairs: &Input) -> String {
    let mut stacks = pairs.0.clone();

    for m in &pairs.1 {
        let mut q = VecDeque::new();
        for _ in 0..m.count {
            let c = stacks[m.from - 1].pop_back().unwrap();
            q.push_back(c);
        }
        while let Some(c) = q.pop_back() {
            stacks[m.to - 1].push_back(c);
        }
    }

    stacks.iter().map(|s| s.back().unwrap()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let input = parse_input(
            r#"
    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
        "#
        );
        assert_eq!(part1(&input), "CMZ");
    }

    #[test]
    fn part2_example() {
        let input = parse_input(
            r#"
    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
        "#
        );
        assert_eq!(part2(&input), "MCD");
    }
}
