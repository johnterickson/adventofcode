use std::cmp;
use std::cmp::{Ord,Ordering};
use std::iter;
use std::collections::HashMap;
use std::str::FromStr;
use nom::*;

#[derive(Debug,PartialEq, Eq, PartialOrd, Ord)]
pub enum SleepRecordAction {
    BeginShift(usize),
    FallsAsleep,
    Wakes
}

named!(guard_id<&str, usize>,
      delimited!(tag!("Guard #"), map_res!(take_while!(|c: char| c.is_digit(10)), usize::from_str), tag!(" begins shift"))
);

named!(sleep_record_action<&str, SleepRecordAction>, alt!(
    complete!(tag!("wakes up"))            => { |_| SleepRecordAction::Wakes } |
    complete!(tag!("falls asleep"))        => { |_| SleepRecordAction::FallsAsleep } |
    complete!(guard_id) => { |id: usize| SleepRecordAction::BeginShift(id) }
));

#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct Date {
    pub y : usize,
    pub m : usize,
    pub d : usize,
}

named!(year<&str, usize>,
  map_res!(take_while_m_n!(1, 4, |c: char| c.is_digit(10)), usize::from_str)
);

named!(month<&str, usize>,
  map_res!(take_while_m_n!(1, 2, |c: char| c.is_digit(10)), usize::from_str)
);

named!(day<&str, usize>,
  map_res!(take_while_m_n!(1, 2, |c: char| c.is_digit(10)), usize::from_str)
);

named!(date<&str, Date>,
  do_parse!(
    y: year >>
    tag!("-") >>
    m: month >>
    tag!("-") >>
    d: day >>
    (Date { y, m, d })
  )
);

named!(hour<&str, usize>,
  map_res!(take_while_m_n!(2, 2, |c: char| c.is_digit(10)), usize::from_str)
);

named!(minute<&str, usize>,
  map_res!(take_while_m_n!(2, 2, |c: char| c.is_digit(10)), usize::from_str)
);

// [1518-11-01 00:00] Guard #10 begins shift
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct SleepRecord {
    pub d : Date,
    pub h : usize,
    pub m : usize,
    pub a : SleepRecordAction,
}

named!(sleep_record<&str, SleepRecord>,
  do_parse!(
    tag!("[") >>
    d: date >>
    tag!(" ") >>
    h: hour >>
    tag!(":") >>
    m: minute >>
    tag!("] ") >>
    a: sleep_record_action >> 
    (SleepRecord { d, h, m, a })
  )
);

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
    let (index, count) = sleep_values.1.iter().enumerate().max_by_key(|e| e.1).unwrap();
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
                    // entry.0 += 1;
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
    let (index, count) = sleep_values.1.iter().enumerate().max_by_key(|e| e.1).unwrap();
    guard_id * index
}