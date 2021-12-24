use std::collections::BTreeMap;

use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day14)]
fn parse_input(input: &str) -> (String,BTreeMap<(char,char),char>) {
    let mut lines = input.lines();
    let template = lines.next().unwrap().trim().to_owned();
    assert_eq!("", lines.next().unwrap().trim());

    let rules = lines.map(|line| {
        let mut tokens = line.split(" -> ");
        let pair: Vec<_> = tokens.next().unwrap().trim().chars().collect();
        assert_eq!(pair.len(), 2);
        let pair = (pair[0], pair[1]);
        let insertion = tokens.next().unwrap().chars().next().unwrap();
        (pair, insertion)
    }).collect();

    (template, rules)
}

fn step1(template: &mut String, rules: &BTreeMap<(char,char),char>) { 
    let chars: Vec<_> = template.chars().collect();
    let mut new_template = Vec::new();
    for pair in chars.windows(2) {
        if let Some(insertion) = rules.get(&(pair[0], pair[1])) {
            new_template.push(pair[0]);
            new_template.push(*insertion);
        } else {
            new_template.extend_from_slice(pair);
        }
    }
    new_template.push(template.chars().last().unwrap());
    *template = String::from_iter(&new_template);
}

#[aoc(day14, part1)]
fn part1(ins: &(String,BTreeMap<(char,char),char>)) -> usize { 
    let (template, rules) = ins;
    let mut template = template.clone();

    for _ in 0..10 {
        step1(&mut template, &rules);
    }

    let mut counts = BTreeMap::new();
    for c in template.chars() {
        *counts.entry(c).or_insert(0) += 1;
    }

    let mut counts: Vec<_> = counts.iter().collect();
    counts.sort_by(|a,b| a.1.cmp(b.1));

    counts[counts.len()-1].1 - counts[0].1
}

fn step2(template: &mut BTreeMap<(char,char),u64>, rules: &BTreeMap<(char,char),char>) { 
    let mut new_template = BTreeMap::new();
    for (pair, pair_count) in template.iter() {
        if let Some(insertion) = rules.get(&pair) {
            *new_template.entry((pair.0, *insertion)).or_insert(0) += pair_count;
            *new_template.entry((*insertion, pair.1)).or_insert(0) += pair_count;
        } else {
            *new_template.entry(*pair).or_insert(0) += pair_count;
        }
    }
    *template = new_template;
}

fn to_template_pairs(template: &str) -> BTreeMap<(char,char),u64> {
    let template: Vec<char> = template.chars().collect();
    let mut template_pairs = BTreeMap::new();
    for pair in template.windows(2) {
        *template_pairs.entry((pair[0], pair[1])).or_insert(0) += 1;
    }
    template_pairs
}

fn part2_inner(ins: &(String,BTreeMap<(char,char),char>), steps: usize) -> u64 { 
    let (template, rules) = ins;
    let mut template_pairs = to_template_pairs(template);

    for _ in 0..steps {
        step2(&mut template_pairs, &rules);
    }


    let mut char_counts = BTreeMap::new();
    for (c, count) in template_pairs.iter()
        .map(|((c1,c2), count)| [(c1, count), (c2, count)])
        .flatten()
    {
        *char_counts.entry(*c).or_insert(0) += *count;
    }
    let mut char_counts: Vec<_> = char_counts.iter().collect();
    char_counts.sort_by(|a,b| a.1.cmp(&b.1));

    dbg!(&counts);
    counts[counts.len()-1].1 - counts[0].1
}

#[aoc(day14, part2)]
fn part2(ins: &(String,BTreeMap<(char,char),char>)) -> u64 { 
    part2_inner(ins, 40)
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn part1_example1() {
       let mut input = parse_input(
r#"NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C"#);
        assert_eq!(input.0, "NNCB");
        step1(&mut input.0, &input.1);
        assert_eq!(input.0, "NCNBCHB");
        step1(&mut input.0, &input.1);
        assert_eq!(input.0, "NBCCNBBBCBHCB");
        step1(&mut input.0, &input.1);
        assert_eq!(input.0, "NBBBCNCCNBBNBNBBCHBHHBCHB");
        step1(&mut input.0, &input.1);
        assert_eq!(input.0, "NBBNBNBBCCNBCNCCNBBNBBNBBBNBBNBBCBHCBHHNHCBBCBHCB");
    }

    #[test]
    fn part1_example2() {
       let input = parse_input(
r#"NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C"#);
        assert_eq!(input.0, "NNCB");
        assert_eq!(1588, part1(&input));
    }

    #[test]
    fn part2_example1() {
       let input = parse_input(
r#"NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C"#);
        assert_eq!(input.0, "NNCB");
        let mut template_pairs = to_template_pairs(&input.0);

        step2(&mut template_pairs, &input.1);
        assert_eq!(template_pairs, to_template_pairs("NCNBCHB"));
        step2(&mut template_pairs, &input.1);
        assert_eq!(template_pairs, to_template_pairs("NBCCNBBBCBHCB"));
        step2(&mut template_pairs, &input.1);
        assert_eq!(template_pairs, to_template_pairs("NBBBCNCCNBBNBNBBCHBHHBCHB"));
        step2(&mut template_pairs, &input.1);
        assert_eq!(template_pairs, to_template_pairs("NBBNBNBBCCNBCNCCNBBNBBNBBBNBBNBBCBHCBHHNHCBBCBHCB"));
    }

    #[test]
    fn part2_example2() {
       let input = parse_input(
r#"NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C"#);
        assert_eq!(1588, part2_inner(&input, 10));
    }
}