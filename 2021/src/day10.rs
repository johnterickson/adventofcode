
use std::collections::{VecDeque, BTreeMap};

use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day10)]
fn parse_input(input: &str) -> Vec<String> {
    input.lines().map(|l| l.trim().to_owned()).collect()
}


#[aoc(day10, part1)]
fn part1(lines: &Vec<String>) -> u64 { 
    let mut score = 0;
    
    let mut matching = BTreeMap::new();
    matching.insert(')','(');
    matching.insert(']','[');
    matching.insert('}','{');
    matching.insert('>','<');

    let mut tokens = VecDeque::new();
    for line in lines {
        for c in line.chars() {
            match c {
                '(' | '[' | '{' | '<' => tokens.push_back(c),
                c => {
                    let expect = Some(matching[&c]);
                    let actual = tokens.pop_back();
                    if expect != actual {
                        score += match c {
                            ')' => 3,
                            ']' => 57,
                            '}' => 1197,
                            '>' => 25137,
                            _ => panic!(),
                        };
                        break;
                    }
                }
            }
        }
    }

    score
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
       let input = parse_input(
r#"[({(<(())[]>[[{[]{<()<>>
    [(()[<>])]({[<{<<[]>>(
    {([(<{}[<>[]}>{[]{[(<()>
    (((({<>}<{<{<>}{[]{[]{}
    [[<[([]))<([[{}[[()]]]
    [{[{({}]{}}([{[{{{}}([]
    {<[[]]>}<{[{[{[]{()[[[]
    [<(<(<(<{}))><([]([]()
    <{([([[(<>()){}]>(<<{{
    <{([{{}}[<[[[<>{}]]]>[]]"#);
       assert_eq!(part1(&input), 26397);
    }
}