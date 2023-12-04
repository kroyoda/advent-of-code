#[aoc(day1, part1)]
pub fn solve_part1(input: &str) -> u32 {
    // For each line, we want to find the first and last digit.
    // Map each line to a 2 digit number, where the first digit is first digit of the line,
    // and the second digit is the last digit of the line.
    // Then sum all the 2 digit numbers.

    input
        .lines()
        .map(|l| {
            let digits: Vec<char> = l
                .chars()
                .filter(|c| c.is_ascii_digit())
                .collect();

            if digits.is_empty() {
                return 0;
            }

            let first = digits.first().unwrap();
            let last = digits.last().unwrap();

            let first_digit = first.to_digit(10).unwrap();
            let last_digit = last.to_digit(10).unwrap();

            let result = first_digit * 10 + last_digit;
            println!("{} -> {}", l, result);

            result
        })
        .sum()
}

#[aoc(day1, part2)]
pub fn solve_part2(input: &str) -> u32 {
    input
        .lines()
        .map(|l| {
            let mut line = l;
            let mut word: Vec<char> = vec![];
            let mut digits: Vec<u32> = vec![];

            const NUMS_IN_WORD: [(&str, u32); 9] = [
                ("one", 1),
                ("two", 2),
                ("three", 3),
                ("four", 4),
                ("five", 5),
                ("six", 6),
                ("seven", 7),
                ("eight", 8),
                ("nine", 9),
            ];

            // If char is digit, push it and continue
            // If char is not digit, it may be a written number
            // In that case, continue with the next char until the word is complete or we encounter a digit
            if line.chars().count() == 0 {
                return 0;
            }

            while let Some(c) = line.chars().next() {
                if c.is_ascii_digit() {
                    digits.push(c.to_digit(10).unwrap());
                    if !word.is_empty() {
                        let word_str: String = word.iter().collect();
                        for (word, num) in NUMS_IN_WORD.iter() {
                            if word_str.contains(word) {
                                digits.push(*num);
                            }
                        }
                    }
                } else if c.is_ascii_alphabetic() {
                    word.push(c);
                }

                if line.chars().count() == 0 {
                    break;
                }

                line = &line[1..];
            }

            if digits.is_empty() {
                return 0;
            }

            let result = digits.first().unwrap() * 10 + digits.last().unwrap();
            println!("{} -> {}", l, result);

            result
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input =
            r#"
            1abc2
            pqr3stu8vwx
            a1b2c3d4e5f
            treb7uchet"#;
        assert_eq!(solve_part1(input), 142)
    }

    #[test]
    fn test_part2() {
        let input =
            r#"
            two1nine
            eightwothree
            abcone2threexyz
            xtwone3four
            4nineeightseven2
            zoneight234
            7pqrstsixteen
            pbkprbzvs819threeonekjpk7brkmbqbkgroneightb"#;
        assert_eq!(solve_part2(input), 369)
    }
}
