use aoc_runner_derive::{aoc, aoc_generator};
use strum_macros::EnumIter;

#[derive(EnumIter)]
#[derive(PartialEq, Eq, Debug, Clone, Copy)]
enum RPC {
    Rock,
    Paper,
    Scissors
}

impl RPC {
    fn parse(s: &str) -> Option<RPC> {
        match s.trim().chars().next().map(|c| c.to_ascii_lowercase()) {
            Some('a') | Some('x') => Some(RPC::Rock),
            Some('b') | Some('y') => Some(RPC::Paper),
            Some('c') | Some('z') => Some(RPC::Scissors),
            _ => None,
        }
    }

    fn hand_score(&self) -> u32 {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }

    fn score(&self, other: &Self) -> u32 {
        self.hand_score() + self.cmp(other).score()
    }

    fn cmp(&self, other: &Self) -> Outcome {
        match (self, other) {
            (x, y) if x == y => Outcome::Draw,
            (Self::Rock, Self::Paper) |
            (Self::Paper, Self::Scissors) |
            (Self::Scissors, Self::Rock)
                => Outcome::Lose,
            _ => Outcome::Win,
        }
    }

    fn in_order_to(&self, o: &Outcome) -> Self {
        match (self, o) {
            (_, Outcome::Draw) => *self,
            (RPC::Rock, Outcome::Lose) => Self::Scissors,
            (RPC::Rock, Outcome::Win) => Self::Paper,
            (RPC::Paper, Outcome::Lose) => Self::Rock,
            (RPC::Paper, Outcome::Win) => Self::Scissors,
            (RPC::Scissors, Outcome::Lose) => Self::Paper,
            (RPC::Scissors, Outcome::Win) => Self::Rock,
        }
    }
}

#[derive(EnumIter)]
#[derive(PartialEq, Eq, Debug, Clone, Copy)]
enum Outcome {
    Lose,
    Draw,
    Win
}

impl Outcome {
    fn parse(s: &str) -> Option<Self> {
        match s.trim().chars().next().map(|c| c.to_ascii_lowercase()) {
            Some('x') => Some(Self::Lose),
            Some('y') => Some(Self::Draw),
            Some('z') => Some(Self::Win),
            _ => None,
        }
    }

    fn score(&self) -> u32 {
        match self {
            Self::Lose => 0,
            Self::Draw => 3,
            Self::Win => 6,
        }
    }
}

#[aoc_generator(day2, part1)]
fn parse_input1(input: &str) -> Vec<(RPC, RPC)> {
    input
        .trim()
        .lines()
        .map(|l| {
            let mut tokens = l.trim().split(' ');
            let other = RPC::parse(tokens.next().unwrap()).unwrap();
            (other, RPC::parse(tokens.next().unwrap()).unwrap())
        })
        .collect()
}

#[aoc_generator(day2, part2)]
fn parse_input2(input: &str) -> Vec<(RPC, Outcome)> {
    input
        .trim()
        .lines()
        .map(|l| {
            let mut tokens = l.trim().split(' ');
            let other = RPC::parse(tokens.next().unwrap()).unwrap();
            (other, Outcome::parse(tokens.next().unwrap()).unwrap())
        })
        .collect()
}

#[aoc(day2, part1)]
fn part1(pairs: &[(RPC, RPC)]) -> u32 {
    pairs
        .iter()
        .map(|(other, me)| {
            let score = me.hand_score() + me.cmp(other).score();
            assert_eq!(score, me.score(other));
            // println!("{:?} vs {:?} = {}", me, other, score);
            score
        })
        .sum()
}

#[aoc(day2, part2)]
fn part2(pairs: &[(RPC, Outcome)]) -> u32 {
    pairs
    .iter()
    .map(|(other, goal)| {
        let me = other.in_order_to(goal);
        goal.score() + me.hand_score()
    })
    .sum()
}


#[cfg(test)]
mod tests {
    use strum::IntoEnumIterator;

    use super::*;

    #[test]
    fn combos() {
        for other in RPC::iter() {
            for me in RPC::iter() {
                assert_eq!(
                    me.score(&other),
                    match (me, other) {
                        (RPC::Rock, RPC::Rock) => 1 + 3,
                        (RPC::Rock, RPC::Paper) => 1 + 0,
                        (RPC::Rock, RPC::Scissors) => 1 + 6,
                        (RPC::Paper, RPC::Rock) => 2 + 6,
                        (RPC::Paper, RPC::Paper) => 2 + 3,
                        (RPC::Paper, RPC::Scissors) => 2 + 0,
                        (RPC::Scissors, RPC::Rock) => 3 + 0,
                        (RPC::Scissors, RPC::Paper) => 3 + 6,
                        (RPC::Scissors, RPC::Scissors) => 3 + 3,
                    });

                let s = format!("{} {}",
                    (other.hand_score() as u8 - 1 + ('A' as u8)) as char,
                    (me.hand_score() as u8 - 1 + ('X' as u8)) as char,
                );
                assert_eq!(
                    me.score(&other),
                    part1(&parse_input1(&s)));
            }
        }
    }

    #[test]
    fn part1_example() {
        let input = parse_input1(r#"
        A Y
        B X
        C Z
        "#.trim());
        assert_eq!(part1(&input), 15);
    }

    #[test]
    fn part2_example() {
        let input = parse_input2(r#"
        A Y
        B X
        C Z
        "#.trim());
        assert_eq!(part2(&input), 12);
    }
}