use std::collections::{HashSet,HashMap};
use nom::*;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct StepName(pub char);

fn from_hex(input: &str) -> Result<StepName, std::num::ParseIntError> {
  Ok(StepName(input.chars().next().unwrap()))
}

named!(step_name<&str, StepName>,
  map_res!(take!(1), from_hex)
);

pub struct Requirement {
    pub name: StepName,
    pub dep: StepName,
}

named!(requirement<&str, Requirement>,
  do_parse!(
    tag!("Step ") >>
    dep: step_name >>
    tag!(" must be finished before step ") >>
    name: step_name >>
    tag!(" can begin.") >>
    (Requirement { name, dep})
  )
);

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
        let step = steps.entry(req.name).or_insert(Step { done: false, deps: HashSet::new()});
        step.deps.insert(req.dep);
    }

    for c in ('A' as u8) ..= ('Z' as u8) {
        let s = StepName(c as char);
        steps.entry(s).or_insert(Step { done: false, deps: HashSet::new()});
    }

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

        for r in &ready {
            steps.get_mut(r).unwrap().done = true;
        }

        let mut ready_sorted : Vec<char> = ready.iter().map(|s| s.0).collect();
        
        ready_sorted.sort();
        for c in ready_sorted {
            order.push(c);
        }

        println!("{}", &order);
    }

    order
}