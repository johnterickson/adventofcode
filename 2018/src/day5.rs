use std::cmp;

#[aoc_generator(day5)]
pub fn input_generator(input: &str) -> Vec<char> {
    input.lines().next().unwrap().chars().collect()
}

fn tails_reacts(input: &Vec<char>) -> bool {
    if input.len() < 2 {
        return false;
    }

    let ultimate = input[input.len() - 1];
    let penultimate = input[input.len() - 2];
    if ultimate == penultimate {
        return false;
    }

    ultimate.to_lowercase().next().unwrap() == penultimate.to_lowercase().next().unwrap()
}

#[aoc(day5, part1)]
pub fn solve_part1(line: &Vec<char>) -> usize {
    let mut reacted: Vec<char> = Vec::with_capacity(line.len());

    for c in line {
        reacted.push(*c);

        // println!("{}", s);
        if tails_reacts(&reacted) {
            reacted.pop();
            reacted.pop();
        }
        // let s: String = reacted.iter().collect();
        // println!("{}", s);
    }

    reacted.len()
}

#[aoc(day5, part2)]
pub fn solve_part2(line: &Vec<char>) -> usize {
    let mut smallest = usize::max_value();
    for candidate_byte in ('a' as u8)..('z' as u8) {
        let candidate = candidate_byte as char;
        let mut reacted: Vec<char> = Vec::with_capacity(line.len());

        for c in line {
            if c.to_lowercase().next().unwrap() == candidate {
                continue;
            }
            reacted.push(*c);

            // println!("{}", s);
            if tails_reacts(&reacted) {
                reacted.pop();
                reacted.pop();
            }
            // let s: String = reacted.iter().collect();
            // println!("{}", s);
        }

        smallest = cmp::min(smallest, reacted.len());
    }

    smallest
}
