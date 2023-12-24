use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day1)]
fn parse_input(input: &str) -> Vec<String> {
    input.trim().lines().map(|l| l.trim().to_string()).collect()
}

#[aoc(day1, part1)]
fn part1(lines: &[String]) -> u32 {
    let mut sum = 0;
    for line in lines {
        let first = line.chars().filter(|c| c.is_ascii_digit()).next().unwrap();
        let last = line.chars().rev().filter(|c| c.is_ascii_digit()).next().unwrap();
        println!("{} -> {} {}", line, first, last);
        sum += 10*first.to_digit(10).unwrap() + last.to_digit(10).unwrap();
    }
    sum
}

#[aoc(day1, part2)]
fn part2(lines: &[String]) -> u32 {
    let mut sum = 0;

    let numbers = [
        "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"
    ];

    for line in lines {
        let mut first_numbers = Vec::with_capacity(10);
        let mut last_numbers = Vec::with_capacity(10);

        for n in numbers {
            first_numbers.push(line.find(n));
            last_numbers.push(line.rfind(n));
        }

        let first_digit = line.chars().enumerate().filter(|(_,c)| c.is_ascii_digit()).next().map(|(i,d)| (i,d.to_digit(10).unwrap()));
        let last_digit = line.chars().rev().enumerate().filter(|(_,c)| c.is_ascii_digit()).next().map(|(i,d)| (line.len() - 1 - i, d.to_digit(10).unwrap()));

        let first_number = first_numbers.iter().enumerate()
            .filter_map(|(n, &i)| i.map(|i| (n as u32,i)))
            .min_by(|a,b| a.1.cmp(&b.1));
        let last_number = last_numbers.iter().enumerate()
            .filter_map(|(n, &i)| i.map(|i| (n as u32,i)))
            .max_by(|a,b| a.1.cmp(&b.1));

        let first_any = match (first_digit, first_number) {
            (Some(d), Some(n)) => if d.0 < n.1 { d.1 } else { n.0 },
            (Some(d), None) => d.1,
            (None, Some(n)) => n.0,
            _ => unreachable!("No numbers found in line {}", line)
        };

        let last_any = match (last_digit, last_number) {
            (Some(d), Some(n)) => if d.0 > n.1 { d.1 } else { n.0 },
            (Some(d), None) => d.1,
            (None, Some(n)) => n.0,
            _ => unreachable!("No numbers found in line {}", line)
        };

        println!("{} -> ({:?} {:?})={} ({:?} {:?})={}", line,
            first_digit, first_number, first_any,
            last_digit, last_number, last_any);

        sum += 10*first_any + last_any;
    }

    sum
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let input = parse_input(r#"
            1abc2
            pqr3stu8vwx
            a1b2c3d4e5f
            treb7uchet
        "#.trim());
        assert_eq!(part1(&input), 142);
    }

    #[test]
    fn part2_example() {
        let input = parse_input(r#"
            two1nine
            eightwothree
            abcone2threexyz
            xtwone3four
            4nineeightseven2
            zoneight234
            7pqrstsixteen
        "#.trim());
        assert_eq!(part2(&input), 281);
    }
}