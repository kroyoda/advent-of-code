#[aoc(day8, part1)]
pub fn solve_part1(input: &str) -> usize {
    let mut memory = 0;
    let mut chars = input.chars();
    while let Some(c) = chars.next() {
        if c == '"' {
            continue;
        }
        if c == '\\' {
            match chars.next() {
                Some('x') => {
                    chars.next();
                    chars.next();
                }
                Some(_) => {}
                None => panic!("Invalid input"),
            }
        }
        memory += 1;
    }
    input.len() - memory
}

#[aoc(day8, part2)]
pub fn solve_part2(input: &str) -> usize {
    let mut encoded = 0;

    encoded - input.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let lines = [r#""""#, r#""abc""#, r#""aaa\"aaa""#, r#""\x27""#];
        let literals = [2, 5, 10, 6];

        for (i, line) in lines.iter().enumerate() {
            assert_eq!(line.len(), literals[i]);
        }

        assert_eq!(solve_part1(lines.join("\n").as_str()), 12);
        assert_eq!(solve_part2(lines.join("\n").as_str()), 19);
    }
}
