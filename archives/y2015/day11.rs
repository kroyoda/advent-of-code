fn increment_password(password: &str) -> String {
    // Available characters: a-z
    let mut result = String::new();
    password
        .chars()
        .rev()
        .fold(true, |carry, c| {
            if carry {
                let mut c = c as u8;
                if c == b'z' {
                    c = b'a';
                    result.push(c as char);
                    true
                } else {
                    c += 1;
                    result.push(c as char);
                    false
                }
            } else {
                result.push(c);
                false
            }
        });
    result.chars().rev().collect()
}

// Passwords must include one increasing straight of at least three letters
fn rule_increasing_straight(password: &str) -> bool {
    let mut prev = 0;
    let mut count = 0;
    for c in password.chars() {
        if (c as u8) == prev + 1 {
            count += 1;
            if count == 2 {
                return true;
            }
        } else {
            count = 0;
        }
        prev = c as u8;
    }
    false
}

// Passwords may not contain the letters i, o, or l
fn rule_no_confusing_letters(password: &str) -> bool {
    password.chars().all(|c| c != 'i' && c != 'o' && c != 'l')
}

// Passwords must contain at least two different, non-overlapping pairs of letters
fn rule_no_two_pairs(password: &str) -> bool {
    let mut prev = 0;
    let mut count = 0;
    for c in password.chars() {
        if (c as u8) == prev {
            count += 1;
            if count == 2 {
                return true;
            }
            prev = 0;
        } else {
            prev = c as u8;
        }
    }
    false
}

#[aoc(day11, part1)]
pub fn solve_part1(password: &str) -> String {
    let mut password = password.to_string();
    loop {
        password = increment_password(&password);
        if
            rule_increasing_straight(&password) &&
            rule_no_confusing_letters(&password) &&
            rule_no_two_pairs(&password)
        {
            break;
        }
    }
    password
}

#[aoc(day11, part2)]
pub fn solve_part2(password: &str) -> String {
    solve_part1(&solve_part1(password))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_increment_password() {
        let password = "abzz";
        let new_password = increment_password(password);
        assert_eq!(new_password, "acaa");
    }

    #[test]
    fn test_rules() {
        let password = "hijklmmn";
        assert!(rule_increasing_straight(password));
        assert!(!rule_no_confusing_letters(password));

        let password = "abbceffg";
        assert!(!rule_increasing_straight(password));
        assert!(rule_no_two_pairs(password));

        let password = "abbcegjk";
        assert!(!rule_no_two_pairs(password));
    }

    // #[test]
    // fn test_part1() {
    //     let password = "abcdefgh";
    //     assert_eq!(solve_part1(password), "abcdffaa");

    //     let password = "ghijklmn";
    //     assert_eq!(solve_part1(password), "ghjaabcc");
    // }

    // #[test]
    // fn test_part2() {
    //     let password = "ghijklmn";
    //     assert_eq!(solve_part2(password), "ghjbbcdd");
    // }
}
