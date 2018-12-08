use std::collections::{HashMap,HashSet};

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<String> {
    input
        .lines()
        .map(|s| s.to_owned())
        .collect()
}

#[aoc(day2, part1)]
pub fn solve_part1(input: &[String]) -> i32 {
    let mut counts = HashMap::new();
    let mut doubles = 0;
    let mut triples = 0;
    for line in input {
        for c in line.chars() {
            let count_entry = counts.entry(c).or_insert(0);
            *count_entry += 1;
        }

        if counts.values().any(|c| *c == 2) {
            doubles += 1;
        }
        if counts.values().any(|c| *c == 3) {
            triples += 1;
        }

        counts.clear();
    }

    doubles * triples
}

#[aoc(day2, part2)]
pub fn solve_part2(input: &[String]) -> String {
    let words : HashSet<&String> = input.iter().collect();

    for word in input {
        let mut chars : Vec<char> = word.chars().collect();
        for index in 0..word.len() {
            for c in 'a' as u8 .. 'z' as u8 {
                if chars[index] == c as char {
                    continue;
                }

                let tmp = chars[index];
                chars[index] = c as char;
                let test_word : String = chars.iter().collect();
                if words.contains(&test_word) {
                    let matching : String = 
                        (0..word.len())
                        .filter(|i| *i != index)
                        .map(|i| chars[i]).collect();
                    return matching;
                }
                chars[index] = tmp;
            }
        }
    }
    
    panic!();
}

#[cfg(test)]
mod tests {
    use super::{solve_part1 as part1, solve_part2 as part2};

    #[test]
    fn sample1() {
        assert_eq!(part1(&["abcdef".to_owned()]), 0);
    }

    #[test]
    fn sample2() {
        assert_eq!(
            part2(&[
                "abc".to_owned(),
                "abd".to_owned()]),
             "ab".to_owned());
    }
}