use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day1)]
fn parse_input(input: &str) -> [Vec<u32>; 2] {
    let mut cols = [Vec::new(), Vec::new()];
    for line in input.trim().lines() {
        for val in line.trim().split_whitespace().zip(cols.iter_mut()) {
            val.1.push(val.0.parse().unwrap());
        }
    }
    for col in cols.iter_mut() {
        col.sort_unstable();
    }
    cols
}

#[aoc(day1, part1)]
fn part1(cols: &[Vec<u32>; 2]) -> u32 {
    cols[0]
        .iter()
        .zip(&cols[1])
        .map(|(a, b)| a.abs_diff(*b))
        .sum()
}

/*
--- Part Two ---
Your analysis only confirmed what everyone feared: the two lists of location IDs are indeed very different.

Or are they?

The Historians can't agree on which group made the mistakes or how to read most of the Chief's handwriting, but in the commotion you notice an interesting detail: a lot of location IDs appear in both lists! Maybe the other numbers aren't location IDs at all but rather misinterpreted handwriting.

This time, you'll need to figure out exactly how often each number from the left list appears in the right list. Calculate a total similarity score by adding up each number in the left list after multiplying it by the number of times that number appears in the right list.

Here are the same example lists again:

3   4
4   3
2   5
1   3
3   9
3   3
For these example lists, here is the process of finding the similarity score:

The first number in the left list is 3. It appears in the right list three times, so the similarity score increases by 3 * 3 = 9.
The second number in the left list is 4. It appears in the right list once, so the similarity score increases by 4 * 1 = 4.
The third number in the left list is 2. It does not appear in the right list, so the similarity score does not increase (2 * 0 = 0).
The fourth number, 1, also does not appear in the right list.
The fifth number, 3, appears in the right list three times; the similarity score increases by 9.
The last number, 3, appears in the right list three times; the similarity score again increases by 9.
So, for these example lists, the similarity score at the end of this process is 31 (9 + 4 + 0 + 0 + 9 + 9).

Once again consider your left and right lists. What is their similarity score?
*/

#[aoc(day1, part2)]
fn part2(cols: &[Vec<u32>; 2]) -> u32 {
    // compute similarity score

    // functionally, this is correct, but we can be more efficient by
    // taking advantage of the fact that the lists are sorted
    // cols[0].iter().map(|a| a * cols[1].iter().filter(|b| a == *b).count() as u32).sum()

    let mut score = 0;
    let mut i = 0;
    let mut j = 0;
    while i < cols[0].len() && j < cols[1].len() {
        match cols[0][i].cmp(&cols[1][j]) {
            std::cmp::Ordering::Less => i += 1,
            std::cmp::Ordering::Equal => {
                let n = cols[0][i];
                // first count the number of times the number appears in the second list
                let right_count = cols[1][j..].iter().take_while(|&x| *x == n).count() as u32;

                let item_score = n * right_count;

                // then find how many times the number appears in the first list
                let left_count = cols[0][i..].iter().take_while(|&x| *x == n).count() as u32;

                let total_score_for_number = item_score * left_count;
                score += total_score_for_number;
                i += left_count as usize;
                j += right_count as usize;
            }
            std::cmp::Ordering::Greater => j += 1,
        }
    }
    score
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let input = parse_input(
            r#"
            3   4
4   3
2   5
1   3
3   9
3   3
        "#
            .trim(),
        );
        assert_eq!(part1(&input), 11);
    }

    #[test]
    fn part2_example() {
        let input = parse_input(
            r#"
            3   4
4   3
2   5
1   3
3   9
3   3
        "#
            .trim(),
        );
        assert_eq!(part2(&input), 31);
    }
}
