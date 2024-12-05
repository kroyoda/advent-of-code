use advent_of_code::{ printer::Printer, take_middle_value };

advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u32> {
    let printer = Printer::new(input);
    let result: u32 = printer
        .get_valid_jobs()
        .iter()
        .map(|job| take_middle_value(job))
        .sum();

    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let printer = Printer::new(input);
    let result = printer
        .get_invalid_jobs()
        .iter()
        .map(|job| printer.fix_job(job))
        .map(|job| take_middle_value(&job))
        .sum();
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
