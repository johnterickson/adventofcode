use aoc_runner_derive::{aoc, aoc_generator};
use std::num::ParseIntError;

use std::collections::{BTreeSet, BTreeMap, VecDeque};

use num_integer::gcd;

#[aoc_generator(day10)]
fn parse_input(input: &str) -> Result<BTreeSet<(isize,isize)>, ParseIntError> {
    Ok(input.lines().enumerate().map(|(y, line)| {
        line.trim().chars().enumerate().filter(|(_,pixel)| *pixel == '#').map(move |(x, _)| (x as isize, y as isize))
    }).flatten().collect())
}

fn count_visible(home: &(isize,isize), asteroids: &BTreeSet<(isize,isize)>) -> usize {
    let mut blocked = BTreeSet::new();
    let (home_x, home_y) = home;
    let (max_x, max_y) = (
        *asteroids.iter().map(|(x,_y)| x).max().unwrap(), 
        *asteroids.iter().map(|(_x,y)| y).max().unwrap()
    );
    for a in asteroids {
        if a == home {
            continue;
        }

        let (blocker_x, blocker_y) = a;
        let (dx, dy) = (blocker_x - home_x, blocker_y - home_y);
        let gcd = gcd(dx,dy);
        let (dx, dy) = (dx/gcd, dy/gcd);
        let mut multiple = gcd + 1;
        loop {
            let x = home_x + multiple*dx;
            let y = home_y + multiple*dy;

            if x < 0 || y < 0 || x > max_x || y > max_y {
                break;
            }

            if asteroids.contains(&(x,y)) {
                // println!("from {:?}, {:?} blocks {:?}", &home, &a, &(x,y));
                blocked.insert((x,y));
            }

            multiple += 1;
        }
    }

    asteroids.len() - blocked.len() - 1
}

fn find_best(asteroids: &BTreeSet<(isize,isize)>) -> ((isize,isize), usize) {
    let candidates = asteroids.iter().map(|a| (*a, count_visible(a, asteroids)));
    candidates.max_by_key(|(_a, count)| *count).unwrap()
}

#[aoc(day10, part1)]
fn part1(asteroids: &BTreeSet<(isize,isize)>) -> usize {
    let (_asteroid, count) = find_best(asteroids);
    count
}

// fn get_sector(x: isize, y: isize) -> usize {
//     match (x, y) {
//         (0, 0) => panic!(),
//         (0, y) => if y > 0 { 4 } else { 0 },
//         (x, 0) => if x > 0 { 2 } else { 6 },
//         (x, y) if x > 0 => if y > 0 { 3 } else { 1 }
//         (x, y) if x <= 0 => if y > 0 { 5 } else { 7 }
//         (_, _) => unreachable!()
//     }
// }

fn angle(y: isize, x: isize) -> isize {
    let gcd = gcd(x,y);
    let (y,x) = (y/gcd,x/gcd);
    let a = (y as f64).atan2(x as f64) - std::f64::consts::FRAC_PI_2;
    let a = if a < -1.0*std::f64::consts::PI {
        a + 2.0*std::f64::consts::PI
    } else {
        a
    };
    (a * 1_000_000_000.0).round() as isize
}

#[aoc(day10, part2)]
fn part2(asteroids: &BTreeSet<(isize,isize)>) -> isize {
    let ((home_x,home_y), _) = find_best(asteroids);
    // dbg!((home_x,home_y));
    let mut directions : BTreeMap<isize, Vec<(usize,(isize,isize))>> = BTreeMap::new();

    for (x,y) in asteroids {
        if (*x,*y) == (home_x, home_y) {
            continue;
        }
        let (dx, dy) = (x - home_x, y - home_y);
        let angle = angle(dy, dx);
        let distance = (dx*dx + dy*dy) as usize;
        directions.entry(angle).or_insert_with(|| Vec::new()).push((distance,(*x,*y)));
    }

    for (_angle, asteroids) in &mut directions {
        asteroids.sort();
    }

    let mut directions : Vec<_> = directions.iter().collect();
    directions.sort();
    let mut directions : Vec<VecDeque<(isize,isize)>> = directions.iter().map(|(_angle, asteroids)| {
        asteroids.iter().map(|(_dist,(x,y))| (*x,*y)).collect()
    }).collect();

    let mut count = 0;
    loop {
        for direction in &mut directions {
            if let Some((x,y)) = direction.pop_front() {
                // println!("{},{}", x, y);
                count += 1;
                if count == 200 {
                    return 100*x + y
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let input = parse_input(
            "##.
             .#.
             .#.
             .#.
             .#."
        ).unwrap();
        assert_eq!(5, count_visible(&(0,0),&input));

        let input = parse_input(
       ".#..#
        .....
        #####
        ....#
        ...##"
        ).unwrap();
        assert_eq!(((3,4),8), find_best(&input));

        let input = parse_input(
           "......#.#.
            #..#.#....
            ..#######.
            .#.#.###..
            .#..#.....
            ..#....#.#
            #..#....#.
            .##.#..###
            ##...#..#.
            .#....####"
        ).unwrap();
        assert_eq!(((5,8),33), find_best(&input));

        let input = parse_input(
            "#.#...#.#.
            .###....#.
            .#....#...
            ##.#.#.#.#
            ....#.#.#.
            .##..###.#
            ..#...##..
            ..##....##
            ......#...
            .####.###."
         ).unwrap();
         assert_eq!(((1,2),35), find_best(&input));

         let input = parse_input(
            ".#..#..###
            ####.###.#
            ....###.#.
            ..###.##.#
            ##.##.#.#.
            ....###..#
            ..#.#..#.#
            #..#.#.###
            .##...##.#
            .....#.#.."
         ).unwrap();
         assert_eq!(((6,3),41), find_best(&input));

         let input = parse_input(
            ".#..##.###...#######
            ##.############..##.
            .#.######.########.#
            .###.#######.####.#.
            #####.##.#.##.###.##
            ..#####..#.#########
            ####################
            #.####....###.#.#.##
            ##.#################
            #####.##.###..####..
            ..######..##.#######
            ####.##.####...##..#
            .#####..#.######.###
            ##...#.##########...
            #.##########.#######
            .####.#.###.###.#.##
            ....##.##.###..#####
            .#.#.###########.###
            #.#.#.#####.####.###
            ###.##.####.##.#..##"
         ).unwrap();
         assert_eq!(((11,13),210), find_best(&input));
    }

    


    #[test]
    fn part2_example() {
        // println!("{}", angle(-100, 000));
        // println!("{}", angle(-100, 001));
        // println!("{}", angle(-100, 100));
        // println!("{}", angle( 000, 100));
        // println!("{}", angle( 100, 100));
        // println!("{}", angle( 100, 000));
        // println!("{}", angle( 100,-100));
        // println!("{}", angle( 000,-100));
        // println!("{}", angle(-100,-100));
        // println!("{}", angle(-100,-001));

        let input = parse_input(
           ".#....#####...#..
            ##...##.#####..##
            ##...#...#.#####.
            ..#.....#...###..
            ..#.#.....#....##"
        ).unwrap();
        assert_eq!(((8,3),30), find_best(&input));


        let input = parse_input(
            ".#..##.###...#######
            ##.############..##.
            .#.######.########.#
            .###.#######.####.#.
            #####.##.#.##.###.##
            ..#####..#.#########
            ####################
            #.####....###.#.#.##
            ##.#################
            #####.##.###..####..
            ..######..##.#######
            ####.##.####...##..#
            .#####..#.######.###
            ##...#.##########...
            #.##########.#######
            .####.#.###.###.#.##
            ....##.##.###..#####
            .#.#.###########.###
            #.#.#.#####.####.###
            ###.##.####.##.#..##"
         ).unwrap();
         assert_eq!(802, part2(&input));
    }
}
