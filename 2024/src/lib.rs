mod day8;
mod day7;
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


#[derive(Debug, Clone)]
struct Grid<T>(Vec<Vec<T>>);

impl<T: Copy> Grid<T> {
    fn get(&self, r: usize, c: usize) -> Option<T> {
        self.0.get(r).and_then(|row| row.get(c)).copied()
    }

    fn get_offset(&self, r: usize, r_offset: isize, c: usize, c_offset: isize) -> Option<T> {
        let r: isize = r.try_into().ok()?;
        let r: usize = (r + r_offset).try_into().ok()?;
        let c: isize = c.try_into().ok()?;
        let c: usize = (c + c_offset).try_into().ok()?;
        self.get(r, c)
    }
}   