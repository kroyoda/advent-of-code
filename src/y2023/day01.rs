#[aoc(day1, part1)]
pub fn solve_part1(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let mut digits = Vec::<usize>::new();

            line.chars().for_each(|c| {
                if c.is_ascii_digit() {
                    digits.push(c.to_digit(10).unwrap() as usize);
                }
            });

            let (first, last) = (digits.first(), digits.last());

            if first.is_none() || last.is_none() {
                return 0;
            }
            let merge = format!("{}{}", first.unwrap(), last.unwrap());
            merge.parse::<usize>().unwrap()
        })
        .sum()
}

#[aoc(day1, part2)]
pub fn solve_part2(input: &str) -> usize {
    const DIGITS_TO_LETTERS: [(&str, usize); 10] = [
        ("zero", 0),
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
    input
        .lines()
        .map(|line| {
            let mut digits = Vec::<usize>::new();
            let mut word = String::new();

            let chars = line.chars().collect::<Vec<char>>();

            for c in chars {
                if c.is_ascii_digit() {
                    digits.push(c.to_digit(10).unwrap() as usize);
                    word.clear();
                } else if c.is_ascii_alphabetic() {
                    for (w, d) in DIGITS_TO_LETTERS.iter() {
                        word.push(c);
                        if word.contains(w) {
                            digits.push(*d);
                            word.clear();
                            break;
                        }
                    }
                } else {
                    continue;
                }
            }

            println!("{:?}", digits);

            let (first, last) = (digits.first(), digits.last());
            if first.is_none() || last.is_none() {
                return 0;
            }

            let merge = format!("{}{}", first.unwrap(), last.unwrap());
            merge.parse::<usize>().unwrap()
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

        assert_eq!(solve_part1(input), 142);
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

        assert_eq!(solve_part2(input), 281);
    }
}
