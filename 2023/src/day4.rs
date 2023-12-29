use std::collections::{BTreeMap, BTreeSet};

use aoc_runner_derive::{aoc, aoc_generator};


#[aoc_generator(day4)]
fn parse_input(input: &str) -> BTreeMap<u32,(BTreeSet<u32>,BTreeSet<u32>)> {
    let mut inputs = BTreeMap::new();
    for line in input.trim().lines().map(|l| l.trim())
    {
        let mut parts = line.split(&[':','|']);

        let number = parts.next().unwrap();
        let mut tokens = number.trim().split_whitespace();
        assert_eq!(Some("Card"), tokens.next());
        let number = tokens.next().unwrap().parse().unwrap();

        let winners = parts.next().unwrap();
        let winners = winners.trim().split_whitespace().map(|n| n.parse().unwrap()).collect();

        let draws = parts.next().unwrap();
        let draws = draws.trim().split_whitespace().map(|n| n.parse().unwrap()).collect();

        assert!(parts.next().is_none());

        inputs.insert(number,(winners,draws));
    }
    inputs
}

#[aoc(day4, part1)]
fn part1(input: &BTreeMap<u32,(BTreeSet<u32>,BTreeSet<u32>)>) -> u32 {
    let mut total = 0;
    for (winners, draws) in input.values() {
        let mut score = 0;
        for draw in draws {
            if winners.contains(draw) {
                if score == 0 {
                    score = 1;
                } else {
                    score *= 2;
                }
            }
        }
        total += score;
    }
    total
}

#[aoc(day4, part2)]
fn part2(input: &BTreeMap<u32,(BTreeSet<u32>,BTreeSet<u32>)>) -> u32 {

    fn recurse(card_number: u32, input: &BTreeMap<u32,(BTreeSet<u32>,BTreeSet<u32>)>, transitive_cards_cache: &mut BTreeMap<u32, u32>) -> u32 {

        if let Some(count) = transitive_cards_cache.get(&card_number) {
            return *count;
        }

        let mut cards = 1; // this card
        let (winners, draws) = &input[&card_number];
        let winning_draws = draws.intersection(&winners).count() as u32;
        for offset in 0..winning_draws {
            cards += recurse(card_number + offset + 1, input, transitive_cards_cache);
        }

        transitive_cards_cache.insert(card_number, cards);

        cards
    }

    let mut total = 0;
    let mut transitive_cards_cache = BTreeMap::new();
    for card_number in input.keys() {
        total += recurse(*card_number, input, &mut transitive_cards_cache);
    }
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let input = parse_input(r#"
        Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
        Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
        Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
        Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
        Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
        Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
        "#.trim());
        assert_eq!(part1(&input), 13);
    }

    #[test]
    fn part2_example() {
        let input = parse_input(r#"
        Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
        Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
        Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
        Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
        Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
        Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
        "#.trim());
        assert_eq!(part2(&input), 30);
    }
}