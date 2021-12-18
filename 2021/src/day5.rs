use std::cmp;

use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug)]
struct Point { x: isize, y: isize }

impl Point {
    fn parse(s: &str) -> Point {
        let mut xy = s.split(',');
        Point { 
            x: isize::from_str_radix(xy.next().unwrap().trim(), 10).unwrap(),
            y: isize::from_str_radix(xy.next().unwrap().trim(), 10).unwrap(),
        }
    }
}

#[aoc_generator(day5)]
fn parse_input(input: &str) -> Vec<(Point, Point)> {
    input.lines().map(|line| {
        let mut points = line.trim().split("->");
        (Point::parse(points.next().unwrap()), Point::parse(points.next().unwrap()))
    }).collect()
}



#[aoc(day5, part1)]
fn part1(lines: &Vec<(Point, Point)>) -> usize {
    let lines: Vec<_> = lines.iter().filter(|(p1,p2)| {
        p1.x == p2.x || p1.y == p2.y
    }).collect();

    let points = lines.iter().map(|(p1,p2)| [p1,p2]).flatten();

    let width = points.clone().map(|p| p.x).max().unwrap() + 1;
    let height = points.map(|p| p.y).max().unwrap() + 1;
    
    let mut counts = vec![vec![0; width as usize]; height as usize];
    for line in lines {
        let x_lo = cmp::min(line.0.x, line.1.x);
        let x_hi = cmp::max(line.0.x, line.1.x);
        let y_lo = cmp::min(line.0.y, line.1.y);
        let y_hi = cmp::max(line.0.y, line.1.y);
        for xx in x_lo..=x_hi {
            for yy in y_lo..=y_hi {
                counts[yy as usize][xx as usize] += 1;
            }
        }
    }

    counts.iter().flatten().filter(|c| **c >= 2).count()
}

#[aoc(day5, part2)]
fn part2(lines: &Vec<(Point, Point)>) -> usize {
    let points = lines.iter().map(|(p1,p2)| [p1,p2]).flatten();

    let width = points.clone().map(|p| p.x).max().unwrap() + 1;
    let height = points.map(|p| p.y).max().unwrap() + 1;

    // dbg!(width, height);
    
    let mut counts = vec![vec![0; width as usize]; height as usize];
    for line in lines {
        let dx = line.1.x - line.0.x;
        let dy = line.1.y - line.0.y;
        let steps = cmp::max(dx.abs(), dy.abs());
        let x_step = dx/steps;
        let y_step = dy/steps;
        let steps = steps + 1;

        // dbg!(&line);

        let mut xx = line.0.x;
        let mut yy = line.0.y;
        for _ in 0..steps {
            counts[yy as usize][xx as usize] += 1;
            xx += x_step;
            yy += y_step;
        }

        // for row in &counts {
        //     for c in row {
        //         if *c == 0 {
        //             print!(".");
        //         } else {
        //             print!("{}", c);
        //         }
        //     }
        //     println!();
        // }
    }

    counts.iter().flatten().filter(|c| **c >= 2).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let input =
    r#"0,9 -> 5,9
    8,0 -> 0,8
    9,4 -> 3,4
    2,2 -> 2,1
    7,0 -> 7,4
    6,4 -> 2,0
    0,9 -> 2,9
    3,4 -> 1,4
    0,0 -> 8,8
    5,5 -> 8,2"#;
        let input = parse_input(input);
        assert_eq!(part1(&input), 5);
    }

    #[test]
    fn part2_example() {
        let input =
    r#"0,9 -> 5,9
    8,0 -> 0,8
    9,4 -> 3,4
    2,2 -> 2,1
    7,0 -> 7,4
    6,4 -> 2,0
    0,9 -> 2,9
    3,4 -> 1,4
    0,0 -> 8,8
    5,5 -> 8,2"#;
        let input = parse_input(input);
        assert_eq!(part2(&input), 12);
    }
}