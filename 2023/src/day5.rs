use std::ops::Range;
use std::{error::Error, collections::BTreeMap};
use std::str::FromStr;

use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug, Clone, PartialEq, Eq)]
struct RangeMap {
    src: Range<usize>,
    dst: Range<usize>,
}

impl FromStr for RangeMap {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut tokens = s.split_whitespace();
        let dst_start = tokens.next().unwrap_or_default().parse()?;
        let src_start = tokens.next().unwrap_or_default().parse()?;
        let len: usize = tokens.next().unwrap_or_default().parse()?;
        assert_eq!(None, tokens.next());
        Ok(RangeMap { src: src_start..src_start+len, dst: dst_start..dst_start+len })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Map {
    from: String,
    to: String,
    ranges: Vec<RangeMap>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Input {
    seeds: Vec<usize>,
    maps: BTreeMap<String, Map>,
}

#[aoc_generator(day5)]
fn parse_input(input: &str) -> Input {
    let mut lines = input.trim().lines().map(|l| l.trim());
    let seeds = lines.next().unwrap();
    let mut seeds_tokens = seeds.split(':');
    assert_eq!("seeds", seeds_tokens.next().unwrap());
    let seeds = seeds_tokens.next().unwrap().trim().split_whitespace().map(|n| n.parse().unwrap()).collect();
    assert_eq!(None, seeds_tokens.next());

    assert_eq!("", lines.next().unwrap());

    let mut maps = Vec::new();
    loop {
        let name = if let Some(name) = lines.next() {
            let mut tokens = name.split(' ');
            let name = tokens.next().unwrap();
            assert_eq!("map:", tokens.next().unwrap());
            assert_eq!(None, tokens.next());
            let mut tokens = name.split('-');
            let from = tokens.next().unwrap().to_string();
            assert_eq!("to", tokens.next().unwrap());
            let to = tokens.next().unwrap().to_string();
            assert_eq!(None, tokens.next());
            (from, to)
        } else {
            break;
        };

        let mut ranges = Vec::new();
        while let Some(Ok(range)) = lines.next().map(|l| l.parse()) {
            ranges.push(range);
        }

        maps.push(Map { from: name.0, to: name.1, ranges });
    }

    let maps = maps.into_iter().map(|m| (m.from.clone(), m)).collect();

    Input {
        seeds,
        maps
    }
}

#[aoc(day5, part1)]
fn part1(input: &Input) -> usize {
    let mut lowest_location = None;
    for seed in &input.seeds {
        let mut current_type = "seed";
        let mut current_index = *seed;
        while current_type != "location" {
            let map = input.maps.get(current_type).expect("Couldn't find map!");
            let range = map.ranges.iter().filter(|range| range.src.contains(&current_index)).next();
            let next_index = range.map(|r| r.dst.start + (current_index - r.src.start));
            current_index = next_index.unwrap_or(current_index);
            current_type = &map.to;
        }

        let location = current_index;

        if let Some(ref mut lowest) = &mut lowest_location {
            *lowest = std::cmp::min(*lowest, location);
        } else {
            lowest_location = Some(location);
        }
    }
    lowest_location.unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let input = parse_input(r#"
seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
        "#.trim());
        assert_eq!(part1(&input), 35);
    }
}