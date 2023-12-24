use std::collections::BTreeMap;

use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day2)]
fn parse_input(input: &str) -> BTreeMap<u32,Vec<BTreeMap<String,u32>>> {
    let mut games = BTreeMap::new();
    for line in input.trim().lines() {
        let line = line.trim();
        let mut game = Vec::new();
        let mut tokens = line.split(&[':']);
        let game_number = tokens.next().unwrap();
        let game_tokens = tokens.next().unwrap();

        let mut tokens = game_number.split_whitespace();
        assert_eq!("Game", tokens.next().unwrap());
        let game_number = tokens.next().unwrap().parse().unwrap();

        let mut game_tokens = game_tokens.split(';');

        while let Some(round_tokens) = game_tokens.next() {
            // dbg!(round_tokens);
            let mut draw = BTreeMap::new();
            let mut draw_tokens = round_tokens.trim().split(',');
            while let Some(draw_token) = draw_tokens.next() {
                let draw_token = draw_token.trim();
                // dbg!(draw_token);
                let mut draw_token = draw_token.split_whitespace();
                let count = draw_token.next().unwrap().parse().unwrap();
                let color = draw_token.next().unwrap().trim().to_string();
                draw.insert(color, count);
            }
            game.push(draw);
        }

        games.insert(game_number, game);
    }

    games
}

#[aoc(day2, part1)]
fn part1(games: &BTreeMap<u32,Vec<BTreeMap<String,u32>>>) -> u32 {
    let mut sum = 0;
    for (id, rounds) in games {
        /*
        The Elf would first like to know which games would have been possible 
        if the bag contained only 
        12 red cubes, 13 green cubes, and 14 blue cubes?
         */
        if rounds.iter().any(|round| {
            round.get("red").unwrap_or(&0) > &12 ||
            round.get("green").unwrap_or(&0) > &13 ||
            round.get("blue").unwrap_or(&0) > &14
        }) {
            // not possible
        } else {
            sum += id;
        }
    }
    sum
}


#[aoc(day2, part2)]
fn part2(games: &BTreeMap<u32,Vec<BTreeMap<String,u32>>>) -> u32 {
    let mut sum = 0;
    for (_, rounds) in games {
        let max_red = rounds.iter().map(|round| round.get("red").unwrap_or(&0)).max().unwrap();
        let max_green = rounds.iter().map(|round| round.get("green").unwrap_or(&0)).max().unwrap();
        let max_blue = rounds.iter().map(|round| round.get("blue").unwrap_or(&0)).max().unwrap();
        
        sum += max_red * max_green * max_blue;
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let input = parse_input(r#"
        Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
        "#.trim());
        assert_eq!(part1(&input), 8);
    }

    #[test]
    fn part2_example() {
        let input = parse_input(r#"
        Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
        "#.trim());
        assert_eq!(part2(&input), 2286);
    }
}