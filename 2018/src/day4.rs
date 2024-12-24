use std::iter;
use std::collections::HashMap;
use std::str::FromStr;
use branch::alt;
use bytes::complete::*;
use combinator::{map, map_res};
use nom::*;
use sequence::{delimited, tuple};

#[derive(Debug,PartialEq, Eq, PartialOrd, Ord)]
pub enum SleepRecordAction {
    BeginShift(usize),
    FallsAsleep,
    Wakes
}

fn guard_id(input: &str) -> IResult<&str, usize> {
  delimited(tag("Guard #"), map_res(take_while(|c: char| c.is_digit(10)), usize::from_str), tag(" begins shift"))(input)
}

fn sleep_record_action(input: &str) -> IResult<&str, SleepRecordAction> {
    alt((
    map(tag("wakes up"),|_| SleepRecordAction::Wakes),
    map(tag("falls asleep"), |_| SleepRecordAction::FallsAsleep),
    map(guard_id, |id: usize| SleepRecordAction::BeginShift(id))
    ))(input)
}

#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct Date {
    pub y : usize,
    pub m : usize,
    pub d : usize,
}

fn year(input: &str) -> IResult<&str, usize> {
  map_res(take_while_m_n(1, 4, |c: char| c.is_digit(10)), usize::from_str)(input)
}

fn month(input: &str) -> IResult<&str, usize> {
  map_res(take_while_m_n(1, 2, |c: char| c.is_digit(10)), usize::from_str)(input)
}

fn day(input: &str) -> IResult<&str, usize> {
  map_res(take_while_m_n(1, 2, |c: char| c.is_digit(10)), usize::from_str)(input)
}

fn date(input: &str) -> IResult<&str, Date> {
  map(
    tuple((year, tag("-"), month, tag("-"), day)),
    |(y, _, m, _, d)| Date { y, m, d }
  )(input)
}

fn hour(input: &str) -> IResult<&str, usize> {
  map_res(take_while_m_n(2, 2, |c: char| c.is_digit(10)), usize::from_str)(input)
}

fn minute(input: &str) -> IResult<&str, usize> {
  map_res(take_while_m_n(2, 2, |c: char| c.is_digit(10)), usize::from_str)(input)
}

// [1518-11-01 00:00] Guard #10 begins shift
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct SleepRecord {
    pub d : Date,
    pub h : usize,
    pub m : usize,
    pub a : SleepRecordAction,
}

fn sleep_record(input: &str) -> IResult<&str, SleepRecord> {
  map(
    tuple((tag("["), date, tag(" "), hour, tag(":"), minute, tag("] "), sleep_record_action)),
    |(_, d, _, h, _, m, _, a)| SleepRecord { d, h, m, a }
  )(input)
}

#[aoc_generator(day4)]
pub fn input_generator(input: &str) -> Vec<SleepRecord> {
    input
        .lines()
        .map(|s| sleep_record(s).unwrap().1)
        .collect()
}

#[aoc(day4, part1)]
pub fn solve_part1(input: &[SleepRecord]) -> usize {
    let mut guards : HashMap<usize,(usize, Vec<usize>)> = HashMap::new();
    let mut id = None;
    let mut start = None;

    let mut sorted : Vec<&SleepRecord> = input.iter().collect();
    sorted.sort();

    for r in sorted {
        println!("{:?}", r);
        match r.a {
            SleepRecordAction::BeginShift(i) => {id = Some(i);},
            SleepRecordAction::FallsAsleep => {start = Some(r.m)},
            SleepRecordAction::Wakes => {
                let end = r.m;

                let entry = guards.entry(id.unwrap()).or_insert((0, iter::repeat(0).take(60).collect()));
                for m in start.unwrap()..end {
                    entry.0 += 1;
                    entry.1[m] += 1;
                }

                start = None;
            },
        };
    }

    let (guard_id, sleep_values) = guards.iter().max_by_key(|e| (e.1).0).unwrap();
    let (index, _count) = sleep_values.1.iter().enumerate().max_by_key(|e| e.1).unwrap();
    guard_id * index
}

#[aoc(day4, part2)]
pub fn solve_part2(input: &[SleepRecord]) -> usize {
    let mut guards : HashMap<usize,(usize, Vec<usize>)> = HashMap::new();
    let mut id = None;
    let mut start = None;

    let mut sorted : Vec<&SleepRecord> = input.iter().collect();
    sorted.sort();

    for r in sorted {
        println!("{:?}", r);
        match r.a {
            SleepRecordAction::BeginShift(i) => {id = Some(i);},
            SleepRecordAction::FallsAsleep => {start = Some(r.m)},
            SleepRecordAction::Wakes => {
                let end = r.m;

                let entry = guards.entry(id.unwrap()).or_insert((0, iter::repeat(0).take(60).collect()));
                for m in start.unwrap()..end {
                    entry.1[m] += 1;
                }

                start = None;
            },
        };
    }

    for guard in guards.values_mut() {
        guard.0 = *guard.1.iter().max().unwrap();
    }

    let (guard_id, sleep_values) = guards.iter().max_by_key(|e| (e.1).0).unwrap();
    let (index, _count) = sleep_values.1.iter().enumerate().max_by_key(|e| e.1).unwrap();
    guard_id * index
}