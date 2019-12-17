use aoc_runner_derive::{aoc, aoc_generator};
use std::num::ParseIntError;

#[aoc_generator(day8)]
fn parse_input(input: &str) -> Result<Vec<usize>, ParseIntError> {
    Ok(input.chars().map(|c| (c as usize) - ('0' as usize) ).collect())
}

#[aoc(day8, part1)]
fn part1(input: &[usize]) -> usize {
    let width = 25;
    let height = 6;
    let layers = input.len() / (width * height);
    assert_eq!(input.len(), width*height*layers);
    let layers = input.chunks(width*height);
    let (fewest_zeroes_layer, _zeroes) = layers
        .map(|layer| {
            assert_eq!(width*height, layer.len());
            let zeroes = layer.iter().filter(|pixel| **pixel == 0).count();
            (layer, zeroes)
        })
        .min_by(|(_,x), (_,y)| x.cmp(y))
        .unwrap();

    let ones = fewest_zeroes_layer.iter().filter(|pixel| **pixel == 1).count();
    let twos = fewest_zeroes_layer.iter().filter(|pixel| **pixel == 2).count();
    ones * twos
}

#[derive(Copy, Clone, PartialEq)]
enum Color {
    Black,
    White,
    Transparent,
}

impl Color {
    fn parse(c: &usize) -> Color {
        match c {
            0 => Color::Black,
            1 => Color::White,
            2 => Color::Transparent,
            _ => panic!(),
        }
    }

    fn merge(&mut self, c: Color) {
        if Color::Transparent == *self {
            *self = c;
        }
    }
}

impl std::fmt::Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "{}", match self {
            Color::Black => " ",
            Color::White => "O",
            Color::Transparent => "?",
        })
    }
}

#[aoc(day8, part2)]
fn part2(input: &[usize]) -> usize {
    let width = 25;
    let height = 6;
    let layers = input.len() / (width * height);
    assert_eq!(input.len(), width*height*layers);
    let layers = input.chunks(width*height);

    let mut picture = vec![Color::Transparent; width*height];
    for layer in layers {
        for (i, pixel) in layer.iter().enumerate() {
            picture[i].merge(Color::parse(pixel));
        }
    }

    for h in 0..height {
        for w in 0..width {
            print!("{}", picture[h*width + w]);
        }
        println!();
    }

    println!();

    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
    }

    #[test]
    fn part2_example() {
    }
}
