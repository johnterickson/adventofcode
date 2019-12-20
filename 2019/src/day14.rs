use aoc_runner_derive::{aoc, aoc_generator};
use std::num::ParseIntError;

use std::collections::{BTreeMap, VecDeque};

#[derive(Clone,Debug)]
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

#[derive(Clone,Debug)]
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

struct Graph {
    pub ore_remaining: usize,
    pub recipes: BTreeMap<String, Formula>,
    pub available: BTreeMap<String, usize>,
}

impl Graph {
    fn new(formulas: &Vec<Formula>) -> Graph {
        let mut recipes: BTreeMap<String, Formula> = BTreeMap::new();
        for f in formulas {
            recipes.insert(f.output.chemical.to_owned(), f.clone());
        }

        Graph {
            recipes,
            available: BTreeMap::new(),
            ore_remaining: 1000000000000
        }
    }

    fn take(&mut self, chemical: &str) {
        *self.available.get_mut(chemical).unwrap() -= 1;
    }


    fn ensure_available(&mut self, chemical_to_create: &str, count_needed: usize) -> Option<usize> {

        // println!("Making available {} {} ", chemical_to_create, &count_needed);

        if chemical_to_create == ORE {
            if self.ore_remaining < count_needed {
                return None;
            }
            self.ore_remaining -= count_needed;
            Some(count_needed)
        } else {
            let mut count_to_create = count_needed;
            if let Some(available) = self.available.get_mut(chemical_to_create) {
                let available = std::cmp::min(count_to_create, *available);
                count_to_create -= available;
                // println!("\talready availble {} {} ", available, chemical_to_create,);
            }

            if count_to_create == 0 {
                return Some(0);
            }

            let recipe = &self.recipes[chemical_to_create];
            let recipe_count = div_round_up(&count_to_create, &recipe.output.count);
            // println!("\tusing {}x recipe {:?} ", recipe_count, &recipe);

            let outputs_created = recipe_count * recipe.output.count;
            let mut ingredients_to_create = Vec::new();

            for ingredient in &recipe.inputs {
                let mut ingredient_count_needed = ingredient.count * recipe_count;
                // println!("\t\tneed {} {}", ingredient_count_needed, ingredient.chemical.as_str());
                if let Some(available) = self.available.get_mut(ingredient.chemical.as_str()) {
                    let available_to_take = std::cmp::min(ingredient_count_needed, *available);
                    ingredient_count_needed -= available_to_take;
                    *available -= available_to_take;
                }

                if ingredient_count_needed > 0 {
                    ingredients_to_create.push((ingredient.chemical.clone(), ingredient_count_needed));
                }
            }

            for (chemical,ingredient_count_needed) in ingredients_to_create {
                match self.ensure_available(chemical.as_str(), ingredient_count_needed) {
                    None => return None,
                    Some(outs) => {
                        // println!("\t\tcreated {} {}", outs, chemical);
                    }
                }

                if chemical != ORE {
                    *self.available.get_mut(&chemical).unwrap() -= ingredient_count_needed;
                }
            }

            let value = {
                if None == self.available.get(chemical_to_create) {
                    self.available.entry(chemical_to_create.to_owned()).or_insert(0)
                } else {
                    self.available.get_mut(chemical_to_create).unwrap()
                }
            };

            
            *value += outputs_created;

            Some(outputs_created)
        }
    }
}

#[aoc(day14, part1)]
fn part1(input: &Vec<Formula>) -> usize {
    let mut g = Graph::new(input);
    let start = g.ore_remaining;
    g.ensure_available(FUEL, 1).unwrap();
    let end = g.ore_remaining;
    (start - end)
}

#[aoc(day14, part2)]
fn part2(input: &Vec<Formula>) -> usize {

    let mut hi = 1;
    loop {
        let mut g = Graph::new(input);
        if None == g.ensure_available(FUEL, hi) {
            break;
        }

        println!("Success at {}", hi);

        hi *= 2;
    }

    let mut lo = hi / 2;
    while lo + 1 < hi {
        let mid = (lo + hi)/2;
        let mut g = Graph::new(input);
        if None == g.ensure_available(FUEL, mid) {
            hi = mid;
        } else {
            lo = mid;
        }
        println!("{} {} {}", lo, mid, hi);
    }

    lo
}

fn part2_try1(input: &Vec<Formula>) -> usize {

    let mut g = Graph::new(input);
    let intial_ore_remaining = g.ore_remaining;
    let mut states = BTreeMap::new();
    let mut fuel = 0;

    let mut estimates = VecDeque::new();

    loop {

        // println!("{:?}", &g.available);
        if g.recipes.len() < 20 {
            if let Some((previous_fuel,previous_ore_remaining)) = states.insert(g.available.clone(), (fuel, g.ore_remaining)) {
                println!("Loop from {} {} to {} {}", previous_fuel, previous_ore_remaining, fuel, g.ore_remaining);
                println!("{:?}", &g.available);
                let fuel_created = fuel - previous_fuel;
                let ore_used = previous_ore_remaining - g.ore_remaining;
                let loops_to_simulate = g.ore_remaining / ore_used;
                fuel += fuel_created * loops_to_simulate;
                g.ore_remaining -= ore_used * loops_to_simulate;
                break;
            }
        }

        if None == g.ensure_available(FUEL, 1) {
            break;
        }

        g.take(FUEL);

        fuel += 1;

        if fuel % 1000 == 0 {
            let ore_per_fuel_estimate = ((intial_ore_remaining - g.ore_remaining) as f64)/(fuel as f64);
            let fuel_estimate = ((intial_ore_remaining as f64) / ore_per_fuel_estimate).round() as usize;
            
            estimates.push_front(fuel_estimate);
            if estimates.len() > 20 {
                println!("1 fuel <-> {} ore as of {} fuel for total of {}: {:?}", ore_per_fuel_estimate, fuel, fuel_estimate, estimates);

                estimates.pop_back();

                if g.recipes.len() >= 20 {
                    if estimates.iter().all(|e| *e == fuel_estimate) {
                        return fuel_estimate;
                    }
                }
            }
        }
    }

    loop {
        if None == g.ensure_available(FUEL, 1) {
            break;
        }

        g.take(FUEL);

        fuel += 1;
    }

    // 2804563 is too low
    // 2804879
    // 2805196

    // 3281821 is too high

    // 3281820 is correcct

    fuel
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
    fn part2_example1() {
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
        assert_eq!(82892753, part2(&input));
    }

    #[test]
    fn part2_example2() {
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
        assert_eq!(5586022, part2(&input));
    }

    #[test]
    fn part2_example3() {
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
        assert_eq!(460664, part2(&input));
    }
}
