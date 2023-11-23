// Process a JSON file and get all the numbers in it
fn sum_numbers(json: &serde_json::Value) -> i64 {
    match json {
        serde_json::Value::Number(n) => n.as_i64().unwrap_or_default(),
        serde_json::Value::Array(a) => a.iter().map(sum_numbers).sum(),
        serde_json::Value::Object(o) => o.values().map(sum_numbers).sum(),
        _ => 0,
    }
}

fn sum_non_reds(json: &serde_json::Value) -> i64 {
    match json {
        serde_json::Value::Number(n) => n.as_i64().unwrap_or_default(),
        serde_json::Value::Array(a) => a.iter().map(sum_non_reds).sum(),
        serde_json::Value::Object(o) => {
            if o.values().any(|v| v == "red") { 0 } else { o.values().map(sum_non_reds).sum() }
        }
        _ => 0,
    }
}

#[aoc(day12, part1)]
pub fn solve_part1(input: &str) -> i64 {
    let json = serde_json::from_str(input).expect("Invalid JSON");
    sum_numbers(&json)
}

#[aoc(day12, part2)]
pub fn solve_part2(input: &str) -> i64 {
    let json = serde_json::from_str(input).expect("Invalid JSON");
    sum_non_reds(&json)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(solve_part1("[1,2,3]"), 6);
        assert_eq!(solve_part1("{\"a\":2,\"b\":4}"), 6);
        assert_eq!(solve_part1("[[[3]]]"), 3);
        assert_eq!(solve_part1("{\"a\":{\"b\":4},\"c\":-1}"), 3);
        assert_eq!(solve_part1("{\"a\":[-1,1]}"), 0);
        assert_eq!(solve_part1("[-1,{\"a\":1}]"), 0);
        assert_eq!(solve_part1("[]"), 0);
        assert_eq!(solve_part1("{}"), 0);
    }

    #[test]
    fn test_part2() {
        assert_eq!(solve_part2("[1,2,3]"), 6);
        assert_eq!(solve_part2("[1,{\"c\":\"red\",\"b\":2},3]"), 4);
        assert_eq!(solve_part2("{\"d\":\"red\",\"e\":[1,2,3,4],\"f\":5}"), 0);
        assert_eq!(solve_part2("[1,\"red\",5]"), 6);
    }
}
