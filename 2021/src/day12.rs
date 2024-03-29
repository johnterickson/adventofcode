
use std::collections::{BTreeMap, BTreeSet};

use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum CaveName {
    Start,
    End,
    Big(String),
    Small(String),
}

impl CaveName {
    fn can_revisit(&self) -> bool {
        match self {
            &CaveName::Big(_) => true,
            _ => false
        }
    }

    fn parse(s: &str) -> Self {
        match s {
            "start" => Self::Start,
            "end" => Self::End,
            s => {
                let c = s.chars().next().unwrap();
                if c.is_lowercase() {
                    Self::Small(s.to_owned())
                } else {
                    Self::Big(s.to_owned())
                }
            },
        }
    }
}

#[aoc_generator(day12)]
fn parse_input<'a>(input: &str) -> BTreeMap<CaveName, BTreeSet<CaveName>> {
    let mut caves = BTreeMap::new();

    for line in input.lines() {
        let line = line.trim();
        let mut caves_tokens = line.split("-");

        let from = CaveName::parse(caves_tokens.next().unwrap());
        let to =  CaveName::parse(caves_tokens.next().unwrap());

        caves.entry(from.clone()).or_insert_with(|| BTreeSet::new()).insert(to.clone());
        caves.entry(to).or_insert_with(|| BTreeSet::new()).insert(from);
    }
    
    caves
}

#[aoc(day12, part1)]
fn part1(caves: &BTreeMap<CaveName, BTreeSet<CaveName>>) -> usize { 

    let mut paths = BTreeSet::new();
    let mut path = Vec::new();

    fn dfs<'a>(caves: &'a BTreeMap<CaveName, BTreeSet<CaveName>>, this: &'a CaveName, path: &mut Vec<&'a CaveName>, complete_paths: &mut BTreeSet<Vec<&'a CaveName>>) {
        if !this.can_revisit() && path.contains(&this) {
            return;
        }

        path.push(this);

        if this == &CaveName::End {
            complete_paths.insert(path.clone());
        } else {
            for n in caves[&this].iter() {
                dfs(caves, n, path, complete_paths);
            }
        }
        
        path.pop();
    }

    dfs(caves, &CaveName::Start, &mut path, &mut paths);

    paths.len()
}

#[aoc(day12, part2)]
fn part2(caves: &BTreeMap<CaveName, BTreeSet<CaveName>>) -> usize { 

    let mut paths = BTreeSet::new();
    let mut path = Vec::new();

    fn dfs<'a>(caves: &'a BTreeMap<CaveName, BTreeSet<CaveName>>, this: &'a CaveName, can_visit_twice: Option<&'a CaveName>, path: &mut Vec<&'a CaveName>, complete_paths: &mut BTreeSet<Vec<&'a CaveName>>) {
        let mut this_is_second_visit = false;

        if path.contains(&this) {
            if Some(this) == can_visit_twice {
                this_is_second_visit = true;
            } else if !this.can_revisit() {
                return;
            }
        }

        path.push(this);
        // println!("{:?} {:?}", can_visit_twice, &path);

        if this == &CaveName::End {
            complete_paths.insert(path.clone());
        } else {
            for n in caves[&this].iter() {
                let can_visit_twice = if this_is_second_visit { None } else { can_visit_twice };
                dfs(caves, n, can_visit_twice, path, complete_paths);
            }
        }
        
        path.pop();
    }

    for cave in caves.keys() {
        match cave {
            CaveName::Small(_) => {
                dfs(caves, &CaveName::Start, Some(cave), &mut path, &mut paths);
                assert_eq!(path.len(), 0);
            }
            _ => {}
        }
    }

    paths.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example1() {
       let input = parse_input(
r#"start-A
start-b
A-c
A-b
b-d
A-end
b-end"#);
       assert_eq!(part1(&input), 10);
    }

    #[test]
    fn part1_example2() {
       let input = parse_input(
r#"dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc"#);
       assert_eq!(part1(&input), 19);
    }

    #[test]
    fn part1_example3() {
       let input = parse_input(
r#"fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW"#);
       assert_eq!(part1(&input), 226);
    }

    #[test]
    fn part2_example() {
       let input = parse_input(
r#"start-A
start-b
A-c
A-b
b-d
A-end
b-end"#);
       assert_eq!(part2(&input), 36);
    }

    #[test]
    fn part2_example2() {
       let input = parse_input(
r#"dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc"#);
       assert_eq!(part2(&input), 103);
    }

    #[test]
    fn part2_example3() {
       let input = parse_input(
r#"fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW"#);
       assert_eq!(part2(&input), 3509);
    }
}