use std::collections::{BTreeSet, BTreeMap};

use aoc_runner_derive::{aoc, aoc_generator};

struct Input {
    signals: Vec<BTreeSet<char>>,
    digits: [BTreeSet<char>; 4]
}

#[aoc_generator(day8)]
fn parse_input(input: &str) -> Vec<Input> {
    input.lines().map(|line| {
        let mut halves = line.split('|');
        let signals = halves.next().unwrap();
        let signals: Vec<BTreeSet<char>> = signals.trim().split(' ')
            .map(|p| p.trim().chars().collect())
            .collect();
        let outputs = halves.next().unwrap();
        let outputs: Vec<BTreeSet<char>> = outputs.trim().split(' ')
            .map(|p| p.trim().chars().collect())
            .collect();
        Input {
            signals,
            digits: outputs.try_into().unwrap()
        }
    }).collect()
}

use lazy_static::lazy_static;
lazy_static! {
    static ref NUMBER_PATTERNS: Vec<BTreeSet<char>> = {
        let mut patterns = Vec::new();
        for p in [
            "abcefg",
            "cf",
            "acdeg",
            "acdfg",
            "bcdf",
            "abdfg",
            "abdefg",
            "acf",
            "abcdefg",
            "abcdfg"
        ].iter() {
            patterns.push(p.chars().collect());
        }
        patterns
    };
}


#[aoc(day8, part1)]
fn part1(inputs: &Vec<Input>) -> i64 { 
    let mut count = 0;
    for input in inputs.iter() {
        for o in &input.digits {
            // println!("'{:?}'", o);
            match o.len() {
                2 /*1*/ |
                4 /*4*/ |
                3 /*7*/ |
                7 /*8*/ => count += 1,
                _ => {},
            }
        }
    }
    count
}

fn is_mapping_possible(map: &BTreeMap<char,char>, observed: &[BTreeSet<char>]) -> bool {
    for scrambled in observed {
        let unscrambled: BTreeSet<char> = scrambled.iter().map(|c| map[c]).collect();
        if !NUMBER_PATTERNS.contains(&unscrambled) {
            // println!("{:?} unscrambles to {:?} which is not a letter. map:{:?}", 
            //     scrambled, &unscrambled, map);
            return false;
        }
    }
    
    true
}

fn decode_output(input: &Input) -> usize {
    use itertools::Itertools;

    let mapping: Vec<_> = "abcdefg".chars().collect();
    for permutation in mapping.iter().permutations(mapping.len()) {
        let mut mapping = BTreeMap::new();
        for (i,c) in permutation.iter().enumerate() {
            mapping.insert(
                ('a' as u8 + i as u8) as char, 
                **c);
        }
        
        if is_mapping_possible(&mapping, &input.signals) {
            // println!("Found mapping: {:?}", &mapping);
            let mut n = 0;
            for scrambled in &input.digits {
                let unscrambled: BTreeSet<_> = scrambled.iter().map(|c| mapping[c]).collect();
                n *= 10;
                let digit = NUMBER_PATTERNS.iter().position(|p| p == &unscrambled)
                    .expect(&format!("Could not find number pattern for digit '{:?}'", unscrambled));
                // println!("{:?} unscrambles to {:?} which is digit '{}'", scrambled, &unscrambled, &digit);
                n += digit;
            }
            return n;
        }
    }

    panic!();
}

#[aoc(day8, part2)]
fn part2(inputs: &Vec<Input>) -> usize { 
    inputs.iter().map(|i| decode_output(i)).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
       let input = parse_input(
r#"be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce"#);
       assert_eq!(part1(&input), 26);
    }

    #[test]
    fn part2_example1() {
       let input = parse_input(
r#"acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf"#);
       assert_eq!(5353, decode_output(&input[0]));
    }

    #[test]
    fn part2_example2() {
        let input = parse_input(
            r#"be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
            edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
            fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
            fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
            aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
            fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
            dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
            bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
            egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
            gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce"#);
        assert_eq!(61229, part2(&input));
     }
}