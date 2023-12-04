const VOWELS: [char; 5] = ['a', 'e', 'i', 'o', 'u'];

#[aoc(day5, part1)]
pub fn solve_part1(input: &str) -> usize {
    input
        .lines()
        .filter(|s: &&str| is_nice(s))
        .count()
}

// It contains at least three vowels (aeiou only), like aei, xazegov, or aeiouaeiouaeiou.
fn three_vowels(string: &str) -> bool {
    string
        .chars()
        .filter(|c| VOWELS.contains(c))
        .count() >= 3
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
        .all(|ab| { !matches!(ab, ('a', 'b') | ('c', 'd') | ('p', 'q') | ('x', 'y')) })
}

fn is_nice(string: &str) -> bool {
    three_vowels(string) && twice_in_a_row(string) && no_forbidden_strings(string)
}

#[aoc(day5, part2)]
pub fn solve_part2(input: &str) -> usize {
    input
        .lines()
        .filter(|s: &&str| is_really_nice(s))
        .count()
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nice_strings() {
        assert!(is_nice("ugknbfddgicrmopn"));
        assert!(is_nice("aaa"));
        assert!(!is_nice("jchzalrnumimnmhp"));
        assert!(!is_nice("haegwjzuvuyypxyu"));
        assert!(!is_nice("dvszwmarrgswjxmb"));
    }

    #[test]
    fn test_really_nice_strings() {
        assert!(is_really_nice("qjhvhtzxzqqjkmpb"));
        assert!(is_really_nice("xxyxx"));
        assert!(!is_really_nice("uurcxstgmygtbstg"));
        assert!(!is_really_nice("ieodomkazucvgmuy"));
    }
}
