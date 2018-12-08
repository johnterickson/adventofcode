use std::cmp;
use std::iter;
use std::collections::HashSet;

pub struct Claim {
    pub id : usize,
    pub x : usize,
    pub y : usize,
    pub w : usize,
    pub h : usize,
}

impl Claim {
    pub fn parse(line: &str) -> Option<Claim> {
        let nums : Vec<&str> = line.trim().split(|c| !char::is_numeric(c)).collect();

        if nums.len() != 9 {
            return None;
        }

        Some(Claim {
            id: nums[1].parse().unwrap(),
            x: nums[4].parse().unwrap(),
            y: nums[5].parse().unwrap(),
            w: nums[7].parse().unwrap(),
            h: nums[8].parse().unwrap(),
        })
    }
}

#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> Vec<Claim> {
    input
        .lines()
        .filter_map(|s| Claim::parse(s))
        .collect()
}

#[aoc(day3, part1)]
pub fn solve_part1(input: &[Claim]) -> i32 {
    let mut width : usize = 0;
    let mut height : usize = 0;
    for c in input {
        width = cmp::max(width, c.x + c.w + 1);
        height = cmp::max(height, c.y + c.h + 1);
    }
    
    let mut cells : Vec<Vec<u32>> = iter::repeat(iter::repeat(0u32).take(height).collect()).take(width).collect();
    for c in input {
        for x in 0..c.w {
            for y in 0..c.h {
                cells[c.x + x][c.y + y] += 1;
            }
        }
    }
    
    let mut multi_claimed = 0;
    for col in cells {
        for c in col {
            if c > 1 {
                multi_claimed += 1;
            }
        }
    }

    multi_claimed
}

#[aoc(day3, part2)]
pub fn solve_part2(input: &[Claim]) -> usize {
    let mut width : usize = 0;
    let mut height : usize = 0;
    for c in input {
        width = cmp::max(width, c.x + c.w + 1);
        height = cmp::max(height, c.y + c.h + 1);
    }
    
    let mut touches : Vec<Vec<HashSet<usize>>> = iter::repeat(iter::repeat(HashSet::new()).take(height).collect()).take(width).collect();
    let mut candidates : HashSet<usize> = input.iter().map(|c| c.id).collect();

    for c in input {
        for x in 0..c.w {
            for y in 0..c.h {
                touches[c.x + x][c.y + y].insert(c.id);
            }
        }
    }

    for x in 0..width {
        for y in 0..height {
            let s = &touches[x][y];
            if s.len() > 1 {
                for  x in s {
                    candidates.remove(x);
                }
            }
        }
    }

    assert_eq!(candidates.len(), 1 as usize);
    *candidates.iter().next().unwrap()
}

// #[cfg(test)]
// mod tests {
//     use super::{solve_part1 as part1, 
//     //solve_part2 as part2
//     };

//     #[test]
//     fn sample1() {
//         assert_eq!(part1(&["abcdef".to_owned()]), 0);
//     }

//     #[test]
//     fn sample2() {
//         assert_eq!(
//             part2(&[
//                 "abc".to_owned(),
//                 "abd".to_owned()]),
//              "ab".to_owned());
//     }
// }