use std::ops::Range;
use std::{error::Error, collections::BTreeMap};
use std::str::FromStr;

use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug, Clone, PartialEq, Eq)]
struct RangeMap {
    src: Range<usize>,
    dst: Range<usize>,
}

impl PartialOrd for RangeMap {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for RangeMap {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.src.start.cmp(&other.src.start)
    }
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

fn intersect(a: &Range<usize>, b: &Range<usize>) -> Option<Range<usize>> {
    let start = std::cmp::max(a.start, b.start);
    let end = std::cmp::min(a.end, b.end);
    if end > start {
        Some(start..end)
    } else {
        None
    }
}

fn remove(whole: &Range<usize>, to_remove: &Range<usize>) -> Option<(Range<usize>, Option<Range<usize>>)> {
    let mut before = None;
    let mut after = None;
    if to_remove.start > whole.start {
        before = Some(whole.start..to_remove.start);
    }
    if to_remove.end < whole.end {
        after = Some(to_remove.end..whole.end);
    }
    if before.as_ref().is_some_and(|r| r.is_empty()) {
        before = None;
    }
    if after.as_ref().is_some_and(|r| r.is_empty()) {
        after = None;
    }

    match (before, after) {
        (Some(before), Some(after)) => Some((before, Some(after))),
        (Some(before), None) => Some((before, None)),
        (None, Some(after)) => Some((after, None)),
        (None, None) => None,
    }
}

fn find_lowest(input: &Input, in_range: &Range<usize>, current_type: &str) -> usize {

    println!("find_lowest({:?}, {:?})", in_range, current_type);
    let mut lowest_location = None;
    let mut remaining = Vec::new();
    remaining.push(RangeMap { src: in_range.clone(), dst: in_range.clone() });

    let map = input.maps.get(current_type).expect("Couldn't find map!");
    let mut range_index = 0;
    while range_index < map.ranges.len() || remaining.len() > 0 {
        let r = if range_index < map.ranges.len() {
            let r = &map.ranges[range_index];
            range_index += 1;
            r.clone()
        } else {
            let r = remaining.pop().unwrap();
            if r.src.start == r.src.end {
                continue;
            }
            r
        };

        if let Some(overlap) = intersect(&r.src, & in_range) {
            let offset = overlap.start - r.src.start;
            let dst = r.dst.start + offset .. r.dst.start + offset + overlap.len();
            let location = if map.to == "location" {
                r.dst.start
            } else {
                find_lowest(input, &dst, &map.to)
            };

            if let Some(ref mut lowest) = &mut lowest_location {
                *lowest = std::cmp::min(*lowest, location);
            } else {
                assert_ne!(0, location);
                lowest_location = Some(location);
            }

            let remaining_intial_len = remaining.len();
            for i in 0..remaining_intial_len {
                let remove_result = remove(&remaining[i].src, &overlap);
                println!("remove({:?}, {:?}) = {:?}", remaining[i].src, overlap, remove_result);
                if let Some((src_before, src_after)) = remove_result {
                    let offset = remaining[i].dst.start as isize - remaining[i].src.start as isize;
                    remaining[i].dst = (src_before.start as isize + offset) as usize .. (src_before.end as isize + offset) as usize;
                    remaining[i].src = src_before;
                    if let Some(src_after) = src_after {
                        remaining.push(RangeMap { 
                            dst: (src_after.start as isize + offset) as usize .. (src_after.end as isize + offset) as usize,
                            src: src_after,
                        });
                    }
                } else {
                    remaining[i].src = 0..0;
                    remaining[i].dst = 0..0;    
                }
            }
        }
    }

    if let Some(lowest) = lowest_location {
        lowest
    } else if map.to == "location" {
        in_range.start
    } else {
        find_lowest(input, in_range, &map.to)
    }
}

#[aoc(day5, part2)]
fn part2(input: &Input) -> usize {
    let mut lowest_location = None;
    for seed_range in input.seeds.chunks_exact(2) {
        let seed_range = seed_range[0]..seed_range[0]+seed_range[1];
        let location = find_lowest(input, &seed_range, "seed");

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
    use core::panic;

    use super::*;
    #[test]
    fn test_intersect() {
        for x1 in 0..5 {
            for x2 in x1..5 {
                for y1 in 0..5 {
                    for y2 in y1..5 {
                        let x = x1..x2;
                        let y = y1..y2;
                        let i = intersect(&x, &y);
                        for z in 0..5 {
                            let in_both = x.contains(&z) && y.contains(&z);
                            match (i.as_ref(), in_both) {
                                (Some(i), in_both) =>
                                    assert_eq!(i.contains(&z), in_both),
                                (None, false) => {},
                                _ => panic!("{:?} {:?} {} {:?} {}", x, y, z, i, in_both),
                            }                        
                        }
                    }
                }
            }
        }
    }        

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

    #[test]
    fn part2_example() {
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

        assert_eq!(46, find_lowest(&input, &(46..47), "humidity"));
        assert_eq!(46, find_lowest(&input, &(45..46), "temperature"));
        assert_eq!(46, find_lowest(&input, &(77..78), "light"));
        assert_eq!(46, find_lowest(&input, &(84..85), "water"));
        assert_eq!(46, find_lowest(&input, &(84..85), "fertilizer"));
        assert_eq!(46, find_lowest(&input, &(84..85), "soil"));
        assert_eq!(46, find_lowest(&input, &(82..83), "seed"));
        assert_eq!(46, find_lowest(&input, &(79..93), "seed"));

        // assert_eq!(part2(&input), 46);
    }
}