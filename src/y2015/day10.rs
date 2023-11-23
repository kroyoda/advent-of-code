pub fn look_n_say(input: &str, n: usize) -> String {
    let mut input = input.to_string();
    for _ in 0..n {
        let mut result = String::new();
        let mut chars = input.chars();
        let mut prev = chars.next().unwrap();
        let mut count = 1;
        for c in chars {
            if c == prev {
                count += 1;
            } else {
                result.push_str(&count.to_string());
                result.push(prev);
                prev = c;
                count = 1;
            }
        }
        result.push_str(&count.to_string());
        result.push(prev);
        input = result;
    }
    input.to_string()
}

#[aoc(day10, part1)]
pub fn solve_part1(input: &str) -> usize {
    look_n_say(input, 40).len()
}

#[aoc(day10, part2)]
pub fn solve_part2(input: &str) -> usize {
    look_n_say(input, 50).len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(look_n_say("1", 1), "11");
        assert_eq!(look_n_say("11", 1), "21");
        assert_eq!(look_n_say("21", 1), "1211");
        assert_eq!(look_n_say("1211", 1), "111221");
        assert_eq!(look_n_say("111221", 1), "312211");
    }
}
