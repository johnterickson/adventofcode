use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day7)]
fn parse_input(input: &str) -> Vec<i64> {
    let line = input.lines().next().unwrap();
    line.trim().split(',').map(|n| i64::from_str_radix(n.trim(), 10).unwrap()).collect()
}


#[aoc(day7, part1)]
fn part1(positions: &Vec<i64>) -> i64 { 
    let mut positions = positions.clone();
    positions.sort();
    let target = positions[positions.len()/2];

    positions.iter().map(|p| (p-target).abs()).sum()
}

#[aoc(day7, part2)]
fn part2(positions: &Vec<i64>) -> i64 {
    todo!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let input = r#"16,1,2,0,4,2,7,1,2,14"#;
        let input = parse_input(input);
        assert_eq!(part1(&input), 37);
    }
}