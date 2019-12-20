use aoc_runner_derive::{aoc, aoc_generator};
use std::num::ParseIntError;

use std::collections::{BTreeMap, VecDeque};

#[derive(Debug)]
struct Ingredient {
    pub chemical: String,
    pub count: usize,
}

impl Ingredient {
    fn parse(s: &str) -> Ingredient {
        let s = s.trim();
        let mut parts = s.split(' ');
        let count = parts.next().unwrap().parse().unwrap();
        let chemical = parts.next().unwrap().to_owned();
        Ingredient { chemical, count }
    }
}

#[derive(Debug)]
struct Formula {
    pub inputs: Vec<Ingredient>,
    pub output: Ingredient,
}

impl Formula {
    fn parse(line: &str) -> Formula {
        let line = line.trim();
        let mut sides = line.split("=>");
        let input_side = sides.next().unwrap();
        let inputs = input_side.split(',');
        let inputs = inputs.map(Ingredient::parse).collect();

        let output = sides.next().unwrap();
        let output = Ingredient::parse(output);

        Formula { inputs, output }
    }
}

fn div_round_up(x: &usize, y: &usize) -> usize {
    (x + y - 1) / y
}

#[aoc_generator(day14)]
fn parse_input(input: &str) -> Result<Vec<Formula>, ParseIntError> {
    Ok(input.lines().map(|l| Formula::parse(l)).collect())
}

const FUEL: &'static str = "FUEL";
const ORE: &'static str = "ORE";

fn map_pop<K: Clone + Ord,V>(map: &mut BTreeMap<K,V>) -> Option<(K,V)> {
    if map.len() == 0 {
        None
    } else {
        let (k,_) = map.iter().next().unwrap();
        let k_copy = k.clone();
        let v = map.remove(&k_copy).unwrap();
        Some((k_copy, v))
    }
}

fn find_ore_needed(formulas: &Vec<Formula>) -> (usize, BTreeMap<String, usize>) {
    let mut recipes: BTreeMap<&str, &Formula> = BTreeMap::new();
    for f in formulas {
        recipes.insert(f.output.chemical.as_str(), &f);
    }

    // dbg!(&recipes);

    let mut ore_needed = 0;
    let mut extras: BTreeMap<String, usize> = BTreeMap::new();
    let mut to_walk: BTreeMap<String, usize> = BTreeMap::new();
    to_walk.insert(FUEL.to_owned(), 1);
    while let Some((chemical, count)) = map_pop(&mut to_walk) {
        if &chemical == ORE {
            ore_needed += count;
            continue;
        }
        let formula = recipes[chemical.as_str()];
        let formula_count = div_round_up(&count, &formula.output.count);
        let extra = formula_count * formula.output.count - count;
        // println!("Using formula {:?} {}x to create {} of {} with {} extra.", 
        //     &formula, &formula_count, &count, &chemical, &extra);
        *extras.entry(chemical.clone()).or_insert(0) += extra;

        for ingredient in &formula.inputs {
            let mut count_needed = ingredient.count * formula_count;
            if let Some(extra) = extras.get_mut(ingredient.chemical.as_str()) {
                let extra_to_take = std::cmp::min(count_needed, *extra);
                count_needed -= extra_to_take;
                *extra -= extra_to_take;
            }
            *to_walk.entry(ingredient.chemical.clone()).or_insert(0) += count_needed;
        }

        // println!("to_walk: {:?}", &to_walk);
        // println!("extras: {:?}", &extras);
    }

    (ore_needed, extras)
}

fn total_needed(formulas: &Vec<Formula>) -> f64 {
    let mut recipes: BTreeMap<&str, &Formula> = BTreeMap::new();
    for f in formulas {
        recipes.insert(f.output.chemical.as_str(), &f);
    }


    let mut ore_needed = 0.0;
    let mut to_walk: BTreeMap<String, f64> = BTreeMap::new();
    to_walk.insert(FUEL.to_owned(), 1.0);
    while let Some((chemical, count)) = map_pop(&mut to_walk) {
        if &chemical == ORE {
            ore_needed += count;
            continue;
        }

        let formula = recipes[chemical.as_str()];
        let formula_count = (count as f64) / (formula.output.count as f64);
        // let extra = formula_count * formula.output.count - count;
        // println!("Using formula {:?} {}x to create {} of {} with {} extra.", 
        //     &formula, &formula_count, &count, &chemical, &extra);
        // *extras.entry(chemical.clone()).or_insert(0) += extra;

        for ingredient in &formula.inputs {
            let mut count_needed = (ingredient.count as f64) * formula_count;
            // if let Some(extra) = extras.get_mut(ingredient.chemical.as_str()) {
            //     let extra_to_take = std::cmp::min(count_needed, *extra);
            //     count_needed -= extra_to_take;
            //     *extra -= extra_to_take;
            // }
            *to_walk.entry(ingredient.chemical.clone()).or_insert(0.0) += count_needed;
        }

        // println!("to_walk: {:?}", &to_walk);
        // println!("extras: {:?}", &extras);
    }

    ore_needed
}

#[aoc(day14, part1)]
fn part1(input: &Vec<Formula>) -> usize {
    let (ore, _) = find_ore_needed(input);
    ore
}

#[aoc(day14, part2)]
fn part2(formulas: &Vec<Formula>) -> usize {
    let mut ore_left : f64 = 1000000000000.0;

    let ore_needed_on_average = total_needed(formulas);


    // 2804563 is too low
    // 2804879
    // 2805196

    // 3281821 is too high

    ((ore_left as f64) / ore_needed_on_average).floor() as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let input = parse_input(
            "10 ORE => 10 A
        1 ORE => 1 B
        7 A, 1 B => 1 C
        7 A, 1 C => 1 D
        7 A, 1 D => 1 E
        7 A, 1 E => 1 FUEL",
        )
        .unwrap();
        assert_eq!(31, part1(&input));

        let input = parse_input(
            "9 ORE => 2 A
            8 ORE => 3 B
            7 ORE => 5 C
            3 A, 4 B => 1 AB
            5 B, 7 C => 1 BC
            4 C, 1 A => 1 CA
            2 AB, 3 BC, 4 CA => 1 FUEL",
        )
        .unwrap();
        assert_eq!(165, part1(&input));

        let input = parse_input(
            "157 ORE => 5 NZVS
            165 ORE => 6 DCFZ
            44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL
            12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ
            179 ORE => 7 PSHF
            177 ORE => 5 HKGWZ
            7 DCFZ, 7 PSHF => 2 XJWVT
            165 ORE => 2 GPVTF
            3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT",
        )
        .unwrap();
        assert_eq!(13312, part1(&input));

        let input = parse_input(
            "2 VPVL, 7 FWMGM, 2 CXFTF, 11 MNCFX => 1 STKFG
            17 NVRVD, 3 JNWZP => 8 VPVL
            53 STKFG, 6 MNCFX, 46 VJHF, 81 HVMC, 68 CXFTF, 25 GNMV => 1 FUEL
            22 VJHF, 37 MNCFX => 5 FWMGM
            139 ORE => 4 NVRVD
            144 ORE => 7 JNWZP
            5 MNCFX, 7 RFSQX, 2 FWMGM, 2 VPVL, 19 CXFTF => 3 HVMC
            5 VJHF, 7 MNCFX, 9 VPVL, 37 CXFTF => 6 GNMV
            145 ORE => 6 MNCFX
            1 NVRVD => 8 CXFTF
            1 VJHF, 6 MNCFX => 4 RFSQX
            176 ORE => 6 VJHF",
        )
        .unwrap();
        assert_eq!(180697, part1(&input));

        let input = parse_input(
            "171 ORE => 8 CNZTR
            7 ZLQW, 3 BMBT, 9 XCVML, 26 XMNCP, 1 WPTQ, 2 MZWV, 1 RJRHP => 4 PLWSL
            114 ORE => 4 BHXH
            14 VRPVC => 6 BMBT
            6 BHXH, 18 KTJDG, 12 WPTQ, 7 PLWSL, 31 FHTLT, 37 ZDVW => 1 FUEL
            6 WPTQ, 2 BMBT, 8 ZLQW, 18 KTJDG, 1 XMNCP, 6 MZWV, 1 RJRHP => 6 FHTLT
            15 XDBXC, 2 LTCX, 1 VRPVC => 6 ZLQW
            13 WPTQ, 10 LTCX, 3 RJRHP, 14 XMNCP, 2 MZWV, 1 ZLQW => 1 ZDVW
            5 BMBT => 4 WPTQ
            189 ORE => 9 KTJDG
            1 MZWV, 17 XDBXC, 3 XCVML => 2 XMNCP
            12 VRPVC, 27 CNZTR => 2 XDBXC
            15 KTJDG, 12 BHXH => 5 XCVML
            3 BHXH, 2 VRPVC => 7 MZWV
            121 ORE => 7 VRPVC
            7 XCVML => 6 RJRHP
            5 BHXH, 4 VRPVC => 5 LTCX",
        )
        .unwrap();
        assert_eq!(2210736, part1(&input));
    }

    #[test]
    fn part2_example() {}
}
