#[aoc(day8, part1)]
pub fn solve_part1(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let mem_len = line.as_bytes().len();

            let mut char_len = 0;
            let mut string = &line.as_bytes()[1..mem_len - 1];
            while !string.is_empty() {
                let (ch, rest) = string.split_first().expect("empty string");
                if let b'\\' = ch {
                    match rest.split_first().expect("invalid string") {
                        (b'"' | b'\\', _) => {
                            string = &rest[1..];
                        }
                        (b'x', _) => {
                            string = &rest[3..];
                        }
                        _ => unreachable!("unexpected string"),
                    }
                } else {
                    string = rest;
                }
                char_len += 1;
            }
            (mem_len, char_len)
        })
        .map(|(m, c)| m - c)
        .sum()
}

#[aoc(day8, part2)]
pub fn solve_part2(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let mem_len = line.as_bytes().len();
            let enc_len =
                mem_len +
                2 +
                line
                    .as_bytes()
                    .iter()
                    .filter(|&&b| (b == b'\\' || b == b'"'))
                    .count();

            (mem_len, enc_len)
        })
        .map(|(m, e)| e - m)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = r#"""
"abc"
"aaa\"aaa"
"\x27""#;

    #[test]
    fn test_part1() {
        assert_eq!(solve_part1(TEST_INPUT), 12);
    }

    #[test]
    fn test_part2() {
        assert_eq!(solve_part2(TEST_INPUT), 19);
    }
}
