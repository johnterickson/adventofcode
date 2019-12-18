use aoc_runner_derive::{aoc, aoc_generator};
use std::num::ParseIntError;

type Vec3 = [isize; 3];

#[aoc_generator(day12)]
fn parse_input(input: &str) -> Result<Vec<Vec3>, ParseIntError> {
    Ok(input.lines().map(|l| {
        let l = l.trim();

        let parse_coord = |s: &str| -> isize {
            let mut tokens = s.split('=');
            tokens.next().unwrap();
            tokens.next().unwrap().trim_matches('>').parse().unwrap()
        };

        let mut coords = l.split(',').map(parse_coord);
        [
            coords.next().unwrap(), 
            coords.next().unwrap(),
            coords.next().unwrap()
        ]
    }).collect())
}

#[derive(Debug)]
struct Gravity {
    positions: Vec<Vec3>,
    velocities: Vec<Vec3>,
    time: usize,
}

impl Gravity {
    fn new(positions: &[Vec3]) -> Gravity {
        Gravity {
            positions: positions.to_vec(),
            velocities: vec![[0,0,0]; positions.len()],
            time: 0,
        }
    }

    fn energy(&self) -> usize {
        let potentials = self.positions.iter().map(|p|p.iter().map(|c| c.abs() as usize).sum());
        let kinetics = self.velocities.iter().map(|p|p.iter().map(|c| c.abs() as usize).sum());
        potentials.zip(kinetics).map::<usize,_>(|(p,k): (usize, usize) | p*k).sum()
    }

    fn step(&mut self) {
        use std::cmp::Ordering;

        // update velocities
        for i in 0..self.positions.len() {
            for j in i+1..self.positions.len() {
                let moon1 = &self.positions[i];
                let moon2 = &self.positions[j];
                for axis in 0..3 {
                    match moon1[axis].cmp(&moon2[axis]) {
                        Ordering::Equal => {},
                        Ordering::Less => {
                            self.velocities[i][axis] += 1;
                            self.velocities[j][axis] -= 1;
                        }
                        Ordering::Greater => {
                            self.velocities[i][axis] -= 1;
                            self.velocities[j][axis] += 1;
                        }
                    }
                }
            }
        }

        //update positions
        for i in 0..self.positions.len() {
            for axis in 0..3 {
                self.positions[i][axis] += self.velocities[i][axis];
            }
        }
        self.time += 1;
    }
}

#[aoc(day12, part1)]
fn part1(input: &[Vec3]) -> usize {
    let mut g = Gravity::new(input);
    for _ in 0..1000 {
        g.step();
    }

    g.energy()
}

#[aoc(day12, part2)]
fn part2(input: &[Vec3]) -> usize {
    unimplemented!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let input = parse_input(
            "<x=-1, y=0, z=2>
            <x=2, y=-10, z=-7>
            <x=4, y=-8, z=8>
            <x=3, y=5, z=-1>").unwrap();
        
        let mut g = Gravity::new(&input);
        // println!("{:?}",&g);
        g.step();
        // println!("{:?}",&g);
        assert_eq!([2,-1,1], g.positions[0]);
        for _ in 0..9 {
            g.step();
        }
        assert_eq!([2,1,-3], g.positions[0]);
        assert_eq!([-3,-2,1], g.velocities[0]);
        assert_eq!(179, g.energy());

        let input = parse_input(
            "<x=-8, y=-10, z=0>
            <x=5, y=5, z=10>
            <x=2, y=-7, z=3>
            <x=9, y=-8, z=-3>").unwrap();
        
        let mut g = Gravity::new(&input);
        for _ in 0..100 {
            g.step();
        }
        assert_eq!(1940, g.energy());

    }

    #[test]
    fn part2_example() {
       
    }
}
