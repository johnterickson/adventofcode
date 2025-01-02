use std::collections::HashSet;

use aoc_runner_derive::{aoc, aoc_generator};

use crate::Grid;

/*
--- Day 6: Guard Gallivant ---
The Historians use their fancy device again, this time to whisk you all away to the North Pole prototype suit manufacturing lab... in the year 1518! It turns out that having direct access to history is very convenient for a group of historians.

You still have to be careful of time paradoxes, and so it will be important to avoid anyone from 1518 while The Historians search for the Chief. Unfortunately, a single guard is patrolling this part of the lab.

Maybe you can work out where the guard will go ahead of time so that The Historians can search safely?

You start by making a map (your puzzle input) of the situation. For example:

....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
The map shows the current position of the guard with ^ (to indicate the guard is currently facing up from the perspective of the map). Any obstructions - crates, desks, alchemical reactors, etc. - are shown as #.

Lab guards in 1518 follow a very strict patrol protocol which involves repeatedly following these steps:

If there is something directly in front of you, turn right 90 degrees.
Otherwise, take a step forward.
Following the above protocol, the guard moves up several times until she reaches an obstacle (in this case, a pile of failed suit prototypes):

....#.....
....^....#
..........
..#.......
.......#..
..........
.#........
........#.
#.........
......#...
Because there is now an obstacle in front of the guard, she turns right before continuing straight in her new facing direction:

....#.....
........>#
..........
..#.......
.......#..
..........
.#........
........#.
#.........
......#...
Reaching another obstacle (a spool of several very long polymers), she turns right again and continues downward:

....#.....
.........#
..........
..#.......
.......#..
..........
.#......v.
........#.
#.........
......#...
This process continues for a while, but the guard eventually leaves the mapped area (after walking past a tank of universal solvent):

....#.....
.........#
..........
..#.......
.......#..
..........
.#........
........#.
#.........
......#v..
By predicting the guard's route, you can determine which specific positions in the lab will be in the patrol path. Including the guard's starting position, the positions visited by the guard before leaving the area are marked with an X:

....#.....
....XXXXX#
....X...X.
..#.X...X.
..XXXXX#X.
..X.X.X.X.
.#XXXXXXX.
.XXXXXXX#.
#XXXXXXX..
......#X..
In this example, the guard will visit 41 distinct positions on your map.

Predict the path of the guard. How many distinct positions will the guard visit before leaving the mapped area?
 */

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Direction { Up, Down, Left, Right }

impl Direction {
    fn turn_right(self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }

    fn deltas(&self) -> (isize, isize) {
        match self {
            Direction::Up => (-1, 0),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
            Direction::Right => (0, 1),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
 enum Cell { Empty, Obstacle, Guard(Direction) }

 struct Input(Grid<Cell>);

#[aoc_generator(day6)]
fn parse_input(input: &str) -> Input {
    let input = input.trim();
    let mut map = vec![];
    for line in input.lines() {
        let mut row = vec![];
        for c in line.chars() {
            let cell = match c {
                '.' => Cell::Empty,
                '#' => Cell::Obstacle,
                '^' => Cell::Guard(Direction::Up),
                'v' => Cell::Guard(Direction::Down),
                '<' => Cell::Guard(Direction::Left),
                '>' => Cell::Guard(Direction::Right),
                _ => panic!("Invalid cell: {}", c),
            };
            row.push(cell);
        }
        map.push(row);
    }
    Input(Grid(map))
}

#[aoc(day6, part1)]
fn part1(input: &Input) -> u32 {
    input.0.0.iter().enumerate().flat_map(|(r, row)| {
        row.iter().enumerate().filter_map(move |(c, cell)| {
            if let Cell::Guard(dir) = *cell {
                Some((r, c, dir))
            } else {
                None
            }
        })
    }).map(|(r, c, mut dir)| {
        let mut visited = HashSet::new();
        let mut r = r;
        let mut c = c;
        loop {
            // println!("Guard {:?} at ({r}, {c})", dir);
            visited.insert((r, c));
            let (dr, dc) = dir.deltas();
            let next = (r as isize + dr, c as isize + dc);
            if let Some(cell) = input.0.get(next.0 as usize, next.1 as usize) {
                if Cell::Obstacle == cell {
                    // println!("Obstacle at ({:?})", next);
                    dir = dir.turn_right();
                } else {
                    r = next.0 as usize;
                    c = next.1 as usize;
                }
            } else {
                return visited.len() as u32;
            }
        }
    }).sum()
}

/*
--- Part Two ---
While The Historians begin working around the guard's patrol route, you borrow their fancy device and step outside the lab. From the safety of a supply closet, you time travel through the last few months and record the nightly status of the lab's guard post on the walls of the closet.

Returning after what seems like only a few seconds to The Historians, they explain that the guard's patrol area is simply too large for them to safely search the lab without getting caught.

Fortunately, they are pretty sure that adding a single new obstruction won't cause a time paradox. They'd like to place the new obstruction in such a way that the guard will get stuck in a loop, making the rest of the lab safe to search.

To have the lowest chance of creating a time paradox, The Historians would like to know all of the possible positions for such an obstruction. The new obstruction can't be placed at the guard's starting position - the guard is there right now and would notice.

In the above example, there are only 6 different positions where a new obstruction would cause the guard to get stuck in a loop. The diagrams of these six situations use O to mark the new obstruction, | to show a position where the guard moves up/down, - to show a position where the guard moves left/right, and + to show a position where the guard moves both up/down and left/right.

Option one, put a printing press next to the guard's starting position:

....#.....
....+---+#
....|...|.
..#.|...|.
....|..#|.
....|...|.
.#.O^---+.
........#.
#.........
......#...
Option two, put a stack of failed suit prototypes in the bottom right quadrant of the mapped area:


....#.....
....+---+#
....|...|.
..#.|...|.
..+-+-+#|.
..|.|.|.|.
.#+-^-+-+.
......O.#.
#.........
......#...
Option three, put a crate of chimney-squeeze prototype fabric next to the standing desk in the bottom right quadrant:

....#.....
....+---+#
....|...|.
..#.|...|.
..+-+-+#|.
..|.|.|.|.
.#+-^-+-+.
.+----+O#.
#+----+...
......#...
Option four, put an alchemical retroencabulator near the bottom left corner:

....#.....
....+---+#
....|...|.
..#.|...|.
..+-+-+#|.
..|.|.|.|.
.#+-^-+-+.
..|...|.#.
#O+---+...
......#...
Option five, put the alchemical retroencabulator a bit to the right instead:

....#.....
....+---+#
....|...|.
..#.|...|.
..+-+-+#|.
..|.|.|.|.
.#+-^-+-+.
....|.|.#.
#..O+-+...
......#...
Option six, put a tank of sovereign glue right next to the tank of universal solvent:

....#.....
....+---+#
....|...|.
..#.|...|.
..+-+-+#|.
..|.|.|.|.
.#+-^-+-+.
.+----++#.
#+----++..
......#O..
It doesn't really matter what you choose to use as an obstacle so long as you and The Historians can put it into position without the guard noticing. The important thing is having enough options that you can find one that minimizes time paradoxes, and in this example, there are 6 different positions you could choose.

You need to get the guard stuck in a loop by adding a single new obstruction. How many different positions could you choose for this obstruction?
*/

#[aoc(day6, part2)]
fn part2(input: &Input) -> u32 {
    let candidates = input.0.0.iter().enumerate().map(|(obs_r, row)| {
        row.iter().enumerate().filter_map(move |(obs_c, cell)| {
            if Cell::Empty == *cell {
                Some((obs_r,obs_c))
            } else {
                None
            }
        })
    }).flatten();



    let (start_r, start_c, start_dir) = input.0.0.iter().enumerate().flat_map(|(r, row)| {
        row.iter().enumerate().filter_map(move |(c, cell)| {
            if let Cell::Guard(dir) = *cell {
                Some((r, c, dir))
            } else {
                None
            }
        })
    }).next().unwrap();

    candidates.filter(|(obs_r,obs_c)| {
        let mut map = input.0.clone();

        let prev = std::mem::replace(&mut map.0[*obs_r][*obs_c], Cell::Obstacle);
        assert_eq!(Cell::Empty, prev);

        let mut visited = HashSet::new();

        let mut r = start_r;
        let mut c = start_c;
        let mut dir = start_dir;

        // println!("Candidate {:?}", (obs_r, obs_c));

        loop {
            // println!(" at {r},{c},{:?}", dir);
            if !visited.insert((r, c, dir)) {
                // found a loop
                // println!("  Found Loop!");
                return true;
            }

            let (dr, dc) = dir.deltas();
            let next = (r as isize + dr, c as isize + dc);
            if let Some(cell) = map.get(next.0 as usize, next.1 as usize) {
                if Cell::Obstacle == cell {
                    // println!(" Obstacle at ({:?})", next);
                    dir = dir.turn_right();
                } else {
                    r = next.0 as usize;
                    c = next.1 as usize;
                }
            } else {
                // println!("  Walked off map");
                return false;
            }
        }
    }).count() as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let input = parse_input(
            r#"
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
        "#
            .trim(),
        );
        assert_eq!(part1(&input), 41);
    }

    #[test]
    fn part2_example() {
        let input = parse_input(
            r#"
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
        "#
            .trim(),
        );
        assert_eq!(part2(&input), 6);
    }
}
