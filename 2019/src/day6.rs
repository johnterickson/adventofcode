use aoc_runner_derive::{aoc, aoc_generator};
use std::num::ParseIntError;

use std::collections::{BTreeSet, BTreeMap};

#[aoc_generator(day6)]
fn parse_input(input: &str) -> Result<Vec<(String,String)>, ParseIntError> {
    Ok(input.lines().map(|l| {
        let mut l = l.split(')');
        let inner = l.next().unwrap();
        let outer = l.next().unwrap();
        (inner.to_owned(), outer.to_owned())
    }).collect())
}

#[aoc(day6, part1)]
fn part1(pairs: &[(String,String)]) -> u32 {
    let mut satellites_of : BTreeMap<&str, BTreeSet<&str>> = BTreeMap::new();
    for (planet, satellite) in pairs {
        satellites_of.entry(planet).or_insert_with(|| BTreeSet::new()).insert(satellite);
    }

    let mut total = 0;
    let mut to_visit = Vec::new();
    to_visit.push(("COM", 0));
    while let Some((planet,steps)) = to_visit.pop() {
        if let Some(satellites) = satellites_of.get(planet) {
            for satellite in satellites {
                total += steps + 1;
                to_visit.push((satellite, steps + 1));
            }
        }
    }

    total
}

#[aoc(day6, part2)]
fn part2(pairs: &[(String,String)]) -> u32 {
    let mut satellites_of : BTreeMap<&str, BTreeSet<&str>> = BTreeMap::new();
    let mut planet_is : BTreeMap<&str, &str> = BTreeMap::new();
    for (planet, satellite) in pairs {
        satellites_of.entry(planet).or_insert_with(|| BTreeSet::new()).insert(satellite);
        planet_is.insert(satellite, planet);
    }

    let mut depth = 1;
    let mut visited = BTreeSet::new();
    let mut to_visit : Vec<&str> = Vec::new();
    let mut to_visit_next : Vec<&str> = Vec::new();
    to_visit.push("YOU");
    loop
    {
        for x in to_visit.drain(..) {
            if x == "SAN" {
                return depth - 3;
            }
            
            if let Some(planet) = planet_is.get(x) {
                if visited.insert(planet) {
                    to_visit_next.push(planet);
                }
            }

            if let Some(satellites) = satellites_of.get(x) {
                for satellite in satellites {
                    if visited.insert(satellite) {
                        to_visit_next.push(satellite)
                    }
                }
            }
        }

        assert!(to_visit.is_empty());
        std::mem::swap(&mut to_visit, &mut to_visit_next);

        depth += 1;
    }
    
    // unreachable!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        {
            let input =
"COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L";
            let input = parse_input(input).unwrap();
            assert_eq!(42, part1(&input));
        }
    }

    #[test]
    fn part2_example() {
        let input =
"COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L
K)YOU
I)SAN";
        let input = parse_input(input).unwrap();
        assert_eq!(4, part2(&input));
    }
}
