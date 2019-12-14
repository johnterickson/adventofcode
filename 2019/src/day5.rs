// It contains at least three vowels (aeiou only), like aei, xazegov, or aeiouaeiouaeiou.
fn three_vowels(string: &str) -> bool {
    string
        .chars()
        .filter(|&c| match c {
            'a' | 'e' | 'i' | 'o' | 'u' => true,
            _ => false,
        }).count()
        >= 3
}

// It contains at least one letter that appears twice in a row, like xx, abcdde (dd), or aabbccdd (aa, bb, cc, or dd).
fn twice_in_a_row(string: &str) -> bool {
    string
        .chars()
        .zip(string.chars().skip(1))
        .any(|(a, b)| a == b)
}

// It does not contain the strings ab, cd, pq, or xy, even if they are part of one of the other requirements.
fn no_forbidden_strings(string: &str) -> bool {
    string
        .chars()
        .zip(string.chars().skip(1))
        .all(|ab| match ab {
            ('a', 'b') | ('c', 'd') | ('p', 'q') | ('x', 'y') => false,
            _ => true,
        })
}

fn is_nice(string: &str) -> bool {
    three_vowels(string) && twice_in_a_row(string) && no_forbidden_strings(string)
}

#[aoc(day5, part1)]
pub fn part1(input: &str) -> usize {
    input.lines().filter(|l| is_nice(l.trim())).count()
}

// It contains a pair of any two letters that appears at least twice in the string without overlapping, like xyxy (xy) or aabcdefgaa (aa), but not like aaa (aa, but it overlaps).
fn two_pairs(string: &str) -> bool {
    if string.len() < 4 {
        return false;
    }

    let pair = &string[0..2];
    let remain = &string[2..];

    remain.contains(pair) || two_pairs(&string[1..])
}

// It contains at least one letter which repeats with exactly one letter between them, like xyx, abcdefeghi (efe), or even aaa.
fn repeat_separated(string: &str) -> bool {
    string
        .chars()
        .zip(string.chars().skip(2))
        .any(|(a, b)| a == b)
}

fn is_really_nice(string: &str) -> bool {
    two_pairs(string) && repeat_separated(string)
}

#[aoc(day5, part2)]
pub fn part2(input: &str) -> usize {
    input.lines().filter(|l| is_really_nice(l.trim())).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    // ugknbfddgicrmopn is nice because it has at least three vowels (u...i...o...), a double letter (...dd...), and none of the disallowed substrings.
    fn example_1() {
        assert!(is_nice("ugknbfddgicrmopn"));
    }

    #[test]
    // aaa is nice because it has at least three vowels and a double letter, even though the letters used by different rules overlap.
    fn example_2() {
        assert!(is_nice("aaa"));
    }

    #[test]
    // jchzalrnumimnmhp is naughty because it has no double letter.
    fn example_3() {
        assert!(!is_nice("jchzalrnumimnmhp"));
        assert!(!twice_in_a_row("jchzalrnumimnmhp"));
    }

    #[test]
    // haegwjzuvuyypxyu is naughty because it contains the string xy.
    fn example_4() {
        assert!(!is_nice("haegwjzuvuyypxyu"));
        assert!(!no_forbidden_strings("haegwjzuvuyypxyu"));
    }

    #[test]
    // dvszwmarrgswjxmb is naughty because it contains only one vowel.
    fn example_5() {
        assert!(!is_nice("dvszwmarrgswjxmb"));
        assert!(!three_vowels("dvszwmarrgswjxmb"));
    }

    #[test]
    // qjhvhtzxzqqjkmpb is nice because is has a pair that appears twice (qj) and a letter that repeats with exactly one letter between them (zxz).
    fn example_6() {
        assert!(is_really_nice("qjhvhtzxzqqjkmpb"));
    }

    #[test]
    // xxyxx is nice because it has a pair that appears twice and a letter that repeats with one between, even though the letters used by each rule overlap.
    fn example_7() {
        assert!(is_really_nice("xxyxx"));
    }

    #[test]
    // uurcxstgmygtbstg is naughty because it has a pair (tg) but no repeat with a single letter between them.
    fn example_8() {
        assert!(!is_really_nice("uurcxstgmygtbstg"));
        assert!(!repeat_separated("uurcxstgmygtbstg"));
    }

    #[test]
    // ieodomkazucvgmuy is naughty because it has a repeating letter with one between (odo), but no pair that appears twice.
    fn example_9() {
        assert!(!is_really_nice("ieodomkazucvgmuy"));
        assert!(!two_pairs("ieodomkazucvgmuy"));
    }

}
