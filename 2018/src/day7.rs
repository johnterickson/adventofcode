use std::collections::{HashSet,HashMap, VecDeque};
use bytes::complete::{tag, take};
use combinator::{map, map_res};
use nom::*;
use sequence::tuple;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, PartialOrd, Ord)]
pub struct StepName(pub char);

fn from_hex(input: &str) -> Result<StepName, std::num::ParseIntError> {
  Ok(StepName(input.chars().next().unwrap()))
}

fn step_name(input: &str) -> IResult<&str, StepName> {
  map_res(take(1u32), from_hex)(input)
}

pub struct Requirement {
    pub name: StepName,
    pub dep: StepName,
}

fn requirement(input: &str) -> IResult<&str, Requirement> {
  map(
   tuple((tag("Step "), step_name, tag(" must be finished before step "), step_name, tag(" can begin."))),
    |(_, name, _, dep, _)| Requirement { name, dep }
  )(input)
}

#[aoc_generator(day7)]
pub fn input_generator(input: &str) -> Vec<Requirement> {
    input
        .lines()
        .map(|s| requirement(s).unwrap().1)
        .collect()
}

#[derive(Debug)]
pub struct Step {
    pub done : bool,
    pub deps : HashSet<StepName>,
}

#[aoc(day7, part1)]
pub fn solve_part1(reqs: &Vec<Requirement>) -> String {
    let mut steps : HashMap<StepName, Step> = HashMap::new();

    for req in reqs {
        steps.entry(req.dep).or_insert(Step { done: false, deps: HashSet::new()});
        let step = steps.entry(req.name).or_insert(Step { done: false, deps: HashSet::new()});
        step.deps.insert(req.dep);
    }

    // for c in ('A' as u8) ..= ('Z' as u8) {
    //     let s = StepName(c as char);
    //     steps.entry(s).or_insert(Step { done: false, deps: HashSet::new()});
    // }

    let mut order = String::new();

    while steps.iter().any(|s| !s.1.done)
    {
        for s in &steps {
            println!("{:?}", &s);
        }

        let mut ready : HashSet<StepName> = HashSet::new();
        for s in &steps {
            if !s.1.done && s.1.deps.iter().all(|d| steps[d].done) {
                ready.insert(*s.0);
            }
        }

        let mut ready_sorted : Vec<StepName> = ready.iter().map(|s| *s).collect();
        ready_sorted.sort();

        steps.get_mut(&ready_sorted[0]).unwrap().done = true;
        order.push(ready_sorted[0].0);
        println!("{}", &order);
    }

    order
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Worker(pub usize);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StepState {
    Waiting,
    Running(Worker, usize),
    Complete,
}

#[derive(Debug)]
pub struct ParallelStep {
    pub state : StepState,
    pub deps : HashSet<StepName>,
}

#[aoc(day7, part2)]
pub fn solve_part2(reqs: &Vec<Requirement>) -> usize {
    let mut steps : HashMap<StepName, ParallelStep> = HashMap::new();

    for req in reqs {
        steps.entry(req.dep).or_insert(ParallelStep { state: StepState::Waiting, deps: HashSet::new()});
        let step = steps.entry(req.name).or_insert(ParallelStep { state: StepState::Waiting, deps: HashSet::new()});
        step.deps.insert(req.dep);
    }

    let mut workers : HashMap<Worker, Option<StepName>> = HashMap::new();
    let worker_count = 5; //2
    for i in 0..worker_count {
        workers.insert(Worker(i), None);
    }

    let mut step_count = 0;
    while steps.iter().any(|s| s.1.state != StepState::Complete) {
        for s in &mut steps{
            match s.1.state {
                StepState::Running(w, remaining) => {
                    if remaining == 1 {
                        s.1.state = StepState::Complete;
                        workers.entry(w).and_modify(|v| *v = None);
                    } else {
                        s.1.state = StepState::Running(w, remaining - 1);
                    }
                },
                _ => {}
            }
        }

        let mut ready_sorted : VecDeque<StepName> = {
            let mut ready : HashSet<StepName> = HashSet::new();
            for s in &steps{
                match s.1.state {
                    StepState::Waiting => {
                        if s.1.deps.iter().all(|d| steps[d].state == StepState::Complete) {
                            ready.insert(*s.0);
                        }
                    },
                    _ => {}
                }
            }

            let mut ready_sorted : Vec<StepName> = ready.iter().map(|s| *s).collect();
            ready_sorted.sort();
            ready_sorted.iter().map(|s| *s).collect()
        };

        for w in &mut workers {
            if w.1.is_none() {
                if let Some(next) = ready_sorted.pop_front() {
                    *w.1 = Some(next);
                    let cost = 60 + 1 + next.0 as usize - 'A' as usize;
                    //let cost = 1 + next.0 as usize - 'A' as usize;
                    steps.entry(next).and_modify(|s| s.state = StepState::Running(*w.0, cost));
                }
            }
        }

        step_count += 1;
        println!("step_count: {}", step_count);
        for s in &steps {
            println!("{:?}", &s);
        }
        println!();
    }

    step_count - 1
}