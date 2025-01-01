use aoc_runner_derive::{aoc, aoc_generator};

/*
--- Day 7: Bridge Repair ---
The Historians take you to a familiar rope bridge over a river in the middle of a jungle. The Chief isn't on this side of the bridge, though; maybe he's on the other side?

When you go to cross the bridge, you notice a group of engineers trying to repair it. (Apparently, it breaks pretty frequently.) You won't be able to cross until it's fixed.

You ask how long it'll take; the engineers tell you that it only needs final calibrations, but some young elephants were playing nearby and stole all the operators from their calibration equations! They could finish the calibrations if only someone could determine which test values could possibly be produced by placing any combination of operators into their calibration equations (your puzzle input).

For example:

190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20
Each line represents a single equation. The test value appears before the colon on each line; it is your job to determine whether the remaining numbers can be combined with operators to produce the test value.

Operators are always evaluated left-to-right, not according to precedence rules. Furthermore, numbers in the equations cannot be rearranged. Glancing into the jungle, you can see elephants holding two different types of operators: add (+) and multiply (*).

Only three of the above equations can be made true by inserting operators:

190: 10 19 has only one position that accepts an operator: between 10 and 19. Choosing + would give 29, but choosing * would give the test value (10 * 19 = 190).
3267: 81 40 27 has two positions for operators. Of the four possible configurations of the operators, two cause the right side to match the test value: 81 + 40 * 27 and 81 * 40 + 27 both equal 3267 (when evaluated left-to-right)!
292: 11 6 16 20 can be solved in exactly one way: 11 + 6 * 16 + 20.
The engineers just need the total calibration result, which is the sum of the test values from just the equations that could possibly be true. In the above example, the sum of the test values for the three equations listed above is 3749.

Determine which equations could possibly be true. What is their total calibration result?
*/

struct Equation {
    test_value: u64,
    values: Vec<u64>,
}

fn try_reach_test_value(target_value: u64, current_value: u64, values: &[u64]) -> bool {
    if values.is_empty() {
        return current_value == target_value;
    }

    let this_value = values[0];
    let remaining_values = &values[1..];
    
    try_reach_test_value(target_value, current_value + this_value, remaining_values)
    || try_reach_test_value(target_value, current_value * this_value, remaining_values)
}

fn concat_digits(a: u64, b: u64) -> u64 {
    let mut result = a;

    let mut tmp = b;
    while tmp > 0 {
        result *= 10;
        tmp /= 10;
    }

    result + b
}

fn try_reach_test_value2(target_value: u64, current_value: u64, values: &[u64]) -> bool {
    if values.is_empty() {
        return current_value == target_value;
    }

    let this_value = values[0];
    let remaining_values = &values[1..];
    
    try_reach_test_value2(target_value, current_value + this_value, remaining_values)
    || try_reach_test_value2(target_value, current_value * this_value, remaining_values)
    || try_reach_test_value2(target_value, concat_digits(current_value, this_value), remaining_values)
}

#[aoc_generator(day7)]
fn parse_input(input: &str) -> Vec<Equation> {
    input.lines().map(|line| {
        let mut parts = line.split(":");
        let test_value = parts.next().unwrap().trim().parse().unwrap();
        let values = parts.next().unwrap().trim().split(' ').map(|v| v.trim().parse().unwrap()).collect();
        Equation { test_value, values }
    }).collect()
}


#[aoc(day7, part1)]
fn part1(input: &Vec<Equation>) -> u64 {
    input.iter().filter_map(|eq| {
        let initial = eq.values[0];
        if try_reach_test_value(eq.test_value, initial, &eq.values[1..]) {
            Some (eq.test_value)
        } else {
            None
        }
    }).sum()
}

#[aoc(day7, part2)]
fn part2(input: &Vec<Equation>) -> u64 {
    input.iter().filter_map(|eq| {
        let initial = eq.values[0];
        if try_reach_test_value2(eq.test_value, initial, &eq.values[1..]) {
            Some (eq.test_value)
        } else {
            None
        }
    }).sum()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn concat() {
        assert_eq!(concat_digits(123, 456), 123456);
        assert_eq!(concat_digits(123, 0), 123);
        assert_eq!(concat_digits(0, 456), 456);
    }

    #[test]
    fn part1_example() {
        let input = parse_input(
            r#"
190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20
        "#
            .trim(),
        );
        assert_eq!(part1(&input), 3749);
    }

    #[test]
    fn part2_example() {
        let input = parse_input(
            r#"
190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20
        "#
            .trim(),
        );
        assert_eq!(part2(&input), 11387);
    }
}