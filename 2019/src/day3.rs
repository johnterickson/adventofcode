use std::collections::HashSet;

type Position = (i32, i32);

#[aoc(day3, part1)]
pub fn part1(input: &str) -> usize {
    let mut houses: HashSet<Position> = HashSet::new();
    let mut current_pos = (0, 0);

    houses.insert(current_pos);

    for &c in input.as_bytes() {
        match c {
            b'>' => current_pos.0 += 1,
            b'<' => current_pos.0 -= 1,
            b'^' => current_pos.1 += 1,
            b'v' => current_pos.1 -= 1,
            _ => unreachable!(),
        }

        houses.insert(current_pos);
    }

    houses.len()
}

#[aoc(day3, part2)]
pub fn part2(input: &str) -> usize {
    let mut houses: HashSet<Position> = HashSet::new();
    let mut santa = (0, 0);
    let mut robot = (0, 0);

    houses.insert(santa);

    for (i, &c) in input.as_bytes().iter().enumerate() {
        let current_pos = if i % 2 == 0 { &mut santa } else { &mut robot };

        match c {
            b'>' => current_pos.0 += 1,
            b'<' => current_pos.0 -= 1,
            b'^' => current_pos.1 += 1,
            b'v' => current_pos.1 -= 1,
            _ => unreachable!(),
        }

        houses.insert(*current_pos);
    }

    houses.len()
}

#[cfg(tests)]
mod tests {
    use super::*;

    #[test]
    // > delivers presents to 2 houses: one at the starting location, and one to the east.
    fn example1() {
        assert_eq!(part1(">"), 2);
    }

    #[test]
    // ^>v< delivers presents to 4 houses in a square, including twice to the house at his starting/ending location.
    fn example2() {
        assert_eq!(part1("^>v<"), 4);
    }

    #[test]
    // ^v^v^v^v^v delivers a bunch of presents to some very lucky children at only 2 houses.
    fn example3() {
        assert_eq!(part1("^v^v^v^v^v"), 2);
    }
}
