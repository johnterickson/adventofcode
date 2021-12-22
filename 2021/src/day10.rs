
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


#[aoc(day10, part2)]
fn part2(lines: &Vec<String>) -> u64 { 
    
    let mut matching = BTreeMap::new();
    matching.insert(')','(');
    matching.insert(']','[');
    matching.insert('}','{');
    matching.insert('>','<');

    let mut scores = Vec::new();
    
    for line in lines {
        let mut tokens = VecDeque::new();
        let mut valid = true;
        for c in line.chars() {
            match c {
                '(' | '[' | '{' | '<' => tokens.push_back(c),
                c => {
                    let expect = Some(matching[&c]);
                    let actual = tokens.pop_back();
                    if expect != actual {
                        valid = false;
                        break;
                    }
                }
            }
        }

        if valid {
            let mut score = 0;

            for c in tokens.iter().rev() {
                score *= 5;
                score += match c {
                    '(' => 1,
                    '[' => 2,
                    '{' => 3,
                    '<' => 4,
                    _ => panic!(),
                };
            }

            scores.push(score);
        }
    }

    scores.sort();
    scores[scores.len()/2]
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

    #[test]
    fn part2_example() {
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
       assert_eq!(part2(&input), 288957);
    }
}