use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day6)]
fn parse_input(input: &str) -> Vec<u64> {
    let line = input.lines().next().unwrap();
    let mut counts = vec![0u64; 9];

    for n in line.trim().split(',') {
        counts[usize::from_str_radix(n.trim(), 10).unwrap()] += 1;
    }

    counts
}


fn simulate(fishes: &mut Vec<u64>, mut days: u32) -> u64{
    while days > 0 {
        let mut new_fish = vec![0; 9];
        for (age, count) in fishes.iter().enumerate() {
            match age {
                0 => {
                    new_fish[8] += count;
                    new_fish[6] += count;
                }
                i => {
                    new_fish[i-1] += count;
                }
            }
        }
        *fishes = new_fish;
        // dbg!(fishes.iter().enumerate());

        days -= 1;
    }

    fishes.iter().cloned().sum()
}

#[aoc(day6, part1)]
fn part1(fishes: &Vec<u64>) -> u64 {   
    let mut fishes = fishes.clone();
    simulate(&mut fishes, 80)
}

#[aoc(day6, part2)]
fn part2(fishes: &Vec<u64>) -> u64 {
    let mut fishes = fishes.clone();
    simulate(&mut fishes, 256)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let input = r#"3,4,3,1,2"#;
        let mut fishes = parse_input(input);
        assert_eq!(simulate(&mut fishes, 18), 26);
        assert_eq!(simulate(&mut fishes, 80-18), 5934);
    }
}