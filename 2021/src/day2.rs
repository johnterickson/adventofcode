use aoc_runner_derive::{aoc, aoc_generator};

enum Direction {
    Forward,
    Down,
    Up
}

#[aoc_generator(day2)]
fn parse_input(input: &str) -> Vec<(Direction,u32)> {
    input.lines().filter_map(|l| {
        let mut tokens = l.trim().split(' ');
        if let (Some(dir),Some(dist)) = (tokens.next(), tokens.next()) {
            let dir = match dir {
                "forward" => Direction::Forward,
                "down" => Direction::Down,
                "up" => Direction::Up,
                _ => panic!(),
            };
            Some((dir, dist.parse().unwrap()))
        } else {
            None
        }
    }).collect()
}

#[aoc(day2, part1)]
fn part1(steps: &[(Direction,u32)]) -> u32 {
    let mut horiz = 0;
    let mut depth = 0;
    for step in steps {
        match step.0 {
            Direction::Forward => horiz += step.1,
            Direction::Down => depth += step.1,
            Direction::Up => depth -= step.1,
        }
    }
    horiz * depth
}

#[aoc(day2, part2)]
fn part2(steps: &[(Direction,u32)]) -> u32 {
    let mut horiz = 0;
    let mut depth = 0;
    let mut aim = 0;
    for step in steps {
        match step.0 {
            Direction::Forward => {
                horiz += step.1;
                depth += aim * step.1;
            }
            Direction::Down => aim += step.1,
            Direction::Up => aim -= step.1,
        }
    }
    horiz * depth
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let input = r#"forward 5
        down 5
        forward 8
        up 3
        down 8
        forward 2"#;
        let input = parse_input(input);
        assert_eq!(part1(&input), 150);
    }

    #[test]
    fn part2_example() {
        let input = r#"forward 5
        down 5
        forward 8
        up 3
        down 8
        forward 2"#;
        let input = parse_input(input);
        assert_eq!(part2(&input), 900);
    }
}