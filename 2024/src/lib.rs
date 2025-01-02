mod day8;
mod day7;
use std::fmt::{Debug, Display};

use aoc_runner_derive::aoc_lib;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;

aoc_lib! {
    year = 2024, extra_alternatives = ["fnv"]
}


#[derive(Clone)]
struct Grid<T>(Vec<Vec<T>>);

impl<T: Copy> Grid<T> {
    fn get(&self, r: usize, c: usize) -> Option<T> {
        self.0.get(r).and_then(|row| row.get(c)).copied()
    }

    fn get_offset(&self, r: usize, r_offset: isize, c: usize, c_offset: isize) -> Option<(usize, usize, T)> {
        let r: isize = r.try_into().ok()?;
        let r: usize = (r + r_offset).try_into().ok()?;
        let c: isize = c.try_into().ok()?;
        let c: usize = (c + c_offset).try_into().ok()?;
        self.get(r, c).map(|cell|(r,c,cell))
    }
}

impl<T: Display> Display for Grid<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f)?;
        for row in &self.0 {
            for cell in row {
                write!(f, "{}", cell)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl<T: Debug> Debug for Grid<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f)?;
        for row in &self.0 {
            for cell in row {
                write!(f, "{:?}", cell)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}