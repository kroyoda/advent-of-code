advent_of_code::solution!(2);

fn is_decreasing(report: &[u32]) -> bool {
    for i in 0..report.len() - 1 {
        if report[i] < report[i + 1] {
            return false;
        }
    }
    true
}

fn is_increasing(report: &[u32]) -> bool {
    for i in 0..report.len() - 1 {
        if report[i] > report[i + 1] {
            return false;
        }
    }
    true
}

fn is_safe_report(report: &[u32]) -> bool {
    // The levels are either all increasing or all decreasing.
    if !is_increasing(report) && !is_decreasing(report) {
        return false;
    }
    // Any two adjacent levels differ by at least one and at most three.
    for i in 0..report.len() - 1 {
        let absolute_diff = ((report[i] as i32) - (report[i + 1] as i32)).abs();
        if !(1..=3).contains(&absolute_diff) {
            return false;
        }
    }
    true
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut safe_reports = 0;
    // Each line is a list of numbers separated by a space
    for report in input.lines() {
        let levels = report
            .split_whitespace()
            .collect::<Vec<&str>>()
            .iter()
            .map(|x| x.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();
        if is_safe_report(&levels) {
            safe_reports += 1;
        }
    }
    Some(safe_reports)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut safe_reports = 0;
    // Each line is a list of numbers separated by a space
    for report in input.lines() {
        let levels = report
            .split_whitespace()
            .collect::<Vec<&str>>()
            .iter()
            .map(|x| x.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();
        if is_safe_report(&levels) {
            safe_reports += 1;
        } else {
            // Try to get a safe report with some levels removed
            for i in 0..levels.len() {
                let mut levels_copy = levels.clone();
                levels_copy.remove(i);
                if is_safe_report(&levels_copy) {
                    safe_reports += 1;
                    break;
                }
            }
        }
    }
    Some(safe_reports)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
