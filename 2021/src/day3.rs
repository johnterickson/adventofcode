use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day3)]
fn parse_input(input: &str) -> (usize, Vec<u32>) {
    let mut bits = None;
    let vals = input.lines().map(|l| {
        let l = l.trim();
        bits = bits.or_else(|| Some(l.len()));
        assert_eq!(l.len(), bits.unwrap());
        u32::from_str_radix(l.trim(), 2).unwrap()
    }).collect();
    (bits.unwrap(), vals)
}

#[aoc(day3, part1)]
fn part1(inputs: &(usize, Vec<u32>)) -> u32 {
    let (bits, lines) = inputs;
    let mut counts = [[0u32; 2]; 32];
    for line in lines {
        for i in 0..*bits {
            let bit = (line >> i) & 0x1;
            counts[i][bit as usize] += 1;
        }
    }

    let mut gamma = 0u32;
    let mut epsilon  = 0u32;
    for (i, counts) in counts.iter().take(*bits).enumerate() {
        if counts[0] < counts[1] {
            gamma |= 1 << i;
        }
        if counts[0] > counts[1] {
            epsilon  |= 1 << i;
        }
    }

    gamma * epsilon
}

#[aoc(day3, part2)]
fn part2(inputs: &(usize, Vec<u32>)) -> u32 {
    let (bits, lines) = inputs;

    let mut oxy = lines.clone();
    for bit in (0..*bits).rev() {

        let mut counts = [0u32; 2];
        for line in &oxy {
            let bit = (line >> bit) & 0x1;
            counts[bit as usize] += 1;
        }

        let to_match = if counts[0] > counts[1] { 0 } else { 1 };
        // dbg!((bit, counts[0], counts[1], to_match, &oxy));
        oxy = oxy.iter().filter(|l| (**l >> bit) & 0x1 == to_match).cloned().collect();
        // dbg!(&oxy);
        if oxy.len() == 1 {
            break;
        }
    }
    assert_eq!(oxy.len(), 1);
    let oxy = oxy[0];
    // dbg!(oxy);

    let mut co2 = lines.clone();
    for bit in (0..*bits).rev() {

        let mut counts = [0u32; 2];
        for line in &co2 {
            let bit = (line >> bit) & 0x1;
            counts[bit as usize] += 1;
        }

        let to_match = if counts[0] > counts[1] { 1 } else { 0 };
        // dbg!((bit, counts[0], counts[1], to_match, &oxy));
        co2 = co2.iter().filter(|l| (**l >> bit) & 0x1 == to_match).cloned().collect();
        // dbg!(&oxy);
        if co2.len() == 1 {
            break;
        }
    }
    assert_eq!(co2.len(), 1);
    let co2 = co2[0];


    oxy * co2
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let input = 
    r#"00100
    11110
    10110
    10111
    10101
    01111
    00111
    11100
    10000
    11001
    00010
    01010"#;
        let input = parse_input(input);
        assert_eq!(part1(&input), 198);
    }

    #[test]
    fn part2_example() {
        let input = 
    r#"00100
    11110
    10110
    10111
    10101
    01111
    00111
    11100
    10000
    11001
    00010
    01010"#;
        let input = parse_input(input);
        assert_eq!(part2(&input), 230);
    }
}