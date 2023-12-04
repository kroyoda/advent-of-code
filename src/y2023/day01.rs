#[aoc(day1, part1)]
pub fn solve_part1(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let nums: Vec<u8> = line
                .chars()
                .filter(|c| c.is_ascii_digit())
                .map(|c| (c as u8) - b'0')
                .collect();
            let (first, last) = (nums.first(), nums.last());
            match (first, last) {
                (Some(first), Some(last)) => (*first as u32) * 10 + (*last as u32),
                _ => 0,
            }
        })
        .sum()
}

#[aoc(day1, part2)]
pub fn solve_part2(input: &str) -> u32 {
    let nums = [
        "one",
        "1",
        "two",
        "2",
        "three",
        "3",
        "four",
        "4",
        "five",
        "5",
        "six",
        "6",
        "seven",
        "7",
        "eight",
        "8",
        "nine",
        "9",
    ];

    let ac = aho_corasick::AhoCorasick::new(nums).expect("Failed to build AhoCorasick");
    input
        .lines()
        .map(|line| {
            let matches = ac.find_overlapping_iter(line).collect::<Vec<_>>();
            let first = matches.first().unwrap().pattern().as_u32() / 2 + 1;
            let last = matches.last().unwrap().pattern().as_u32() / 2 + 1;

            10 * first + last
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = r#"1abc2
            pqr3stu8vwx
            a1b2c3d4e5f
            treb7uchet"#;
        assert_eq!(solve_part1(input), 142)
    }

    #[test]
    fn test_part2() {
        let input =
            r#"two1nine
            eightwothree
            abcone2threexyz
            xtwone3four
            4nineeightseven2
            zoneight234
            7pqrstsixteen"#;
        assert_eq!(solve_part2(input), 281)
    }
}
