type Gift = (u32, u32, u32);

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<Gift> {
    input
        .lines()
        .map(|l| {
            let mut gift = l.trim().split('x').map(|d| d.parse().unwrap());
            (
                gift.next().unwrap(),
                gift.next().unwrap(),
                gift.next().unwrap(),
            )
        }).collect()
}

#[aoc(day2, part1)]
pub fn solve_part1(input: &[Gift]) -> u32 {
    input
        .iter()
        .map(|&(l, w, h)| {
            let (s1, s2) = smallest_side((l, w, h));

            2 * l * w + 2 * w * h + 2 * h * l + s1 * s2
        }).sum()
}

#[aoc(day2, part1, for_loop)]
pub fn solve_part1_for(input: &[Gift]) -> u32 {
    let mut sum = 0;

    for &(l, w, h) in input {
        let (s1, s2) = smallest_side((l, w, h));

        sum += 2 * l * w + 2 * w * h + 2 * h * l + s1 * s2;
    }

    sum
}

#[aoc(day2, part2)]
pub fn solve_part2(input: &[Gift]) -> u32 {
    input
        .iter()
        .map(|&(l, w, h)| {
            let (s1, s2) = smallest_side((l, w, h));

            (s1 + s2) * 2 + l * w * h
        }).sum()
}

fn smallest_side((l, w, h): Gift) -> (u32, u32) {
    let mut vec = vec![l, w, h];
    vec.sort();

    (vec[0], vec[1])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    // A present with dimensions 2x3x4 requires 2*6 + 2*12 + 2*8 = 52 square feet of wrapping paper plus 6 square feet of slack, for a total of 58 square feet.
    fn example1() {
        assert_eq!(solve_part1(&input_generator("2x3x4")), 58);
    }

    #[test]
    // A present with dimensions 1x1x10 requires 2*1 + 2*10 + 2*10 = 42 square feet of wrapping paper plus 1 square foot of slack, for a total of 43 square feet.
    fn example2() {
        assert_eq!(solve_part1(&input_generator("1x1x10")), 43);
    }

    #[test]
    // A present with dimensions 2x3x4 requires 2+2+3+3 = 10 feet of ribbon to wrap the present plus 2*3*4 = 24 feet of ribbon for the bow, for a total of 34 feet.
    fn example3() {
        assert_eq!(solve_part2(&input_generator("2x3x4")), 34);
    }

    #[test]
    // A present with dimensions 1x1x10 requires 1+1+1+1 = 4 feet of ribbon to wrap the present plus 1*1*10 = 10 feet of ribbon for the bow, for a total of 14 feet.
    fn example4() {
        assert_eq!(solve_part2(&input_generator("1x1x10")), 14);
    }
}
