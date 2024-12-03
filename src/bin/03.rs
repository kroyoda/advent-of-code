advent_of_code::solution!(3);

fn extract_mul(s: &str) -> Result<[u32; 2], String> {
    let pattern = "mul(";
    let mut cursor = 0;

    if s.len() < pattern.len() {
        return Err("String is too short".to_string());
    }

    if &s[..pattern.len()] != pattern {
        return Err("String does not start with 'mul('".to_string());
    }
    cursor += pattern.len();

    let digits_x = s[cursor..].bytes().take_while(u8::is_ascii_digit).count();
    if digits_x == 0 {
        return Err("No digits found".to_string());
    }
    cursor += digits_x;

    if s[cursor..].bytes().next() != Some(b',') {
        return Err("No ',' found".to_string());
    }
    cursor += 1;

    let digits_y = s[cursor..].bytes().take_while(u8::is_ascii_digit).count();
    if digits_y == 0 {
        return Err("No digits found".to_string());
    }
    cursor += digits_y;

    if s[cursor..].bytes().next() != Some(b')') {
        return Err("No ')' found".to_string());
    }

    Ok([
        s[pattern.len()..pattern.len() + digits_x].parse().unwrap(),
        s[pattern.len() + digits_x + 1..pattern.len() + digits_x + 1 + digits_y].parse().unwrap(),
    ])
}

fn extract_do(s: &str) -> bool {
    let pattern = "do()";

    if s.len() < pattern.len() {
        return false;
    }

    if &s[..pattern.len()] != pattern {
        return false;
    }

    true
}

fn extract_do_not(s: &str) -> bool {
    let pattern = "don't()";

    if s.len() < pattern.len() {
        return false;
    }

    if &s[..pattern.len()] != pattern {
        return false;
    }

    true
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut sum = 0;
    let lines = input.lines().collect::<Vec<&str>>();

    for line in &lines {
        for i in 0..line.len() {
            if let Ok([x, y]) = extract_mul(&line[i..]) {
                sum += x * y;
            }
        }
    }

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut sum = 0;
    let lines = input.lines().collect::<Vec<&str>>();

    let mut enabled = true;
    let mut cursor = 0;

    for line in &lines {
        for i in 0..line.len() {
            if let Ok([x, y]) = extract_mul(&line[i..]) {
                if enabled {
                    sum += x * y;
                }
            } else if extract_do(&line[i..]) {
                enabled = true;
            } else if extract_do_not(&line[i..]) {
                enabled = false;
            }
        }
    }

    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(48));
    }
}
