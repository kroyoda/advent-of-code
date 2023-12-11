use std::collections::HashSet;

#[derive(Debug)]
pub struct PartNumber {
    value: i64,
    points: HashSet<(i64, i64)>,
}

impl PartNumber {
    fn new(row: i64, col: i64, ch: char) -> Self {
        let points = HashSet::from([
            (row - 1, col - 1),
            (row, col - 1),
            (row + 1, col - 1),
            (row - 1, col),
            (row + 1, col),
            (row - 1, col + 1),
            (row, col + 1),
            (row + 1, col + 1),
        ]);
        Self {
            value: ((ch as u8) - b'0') as i64,
            points,
        }
    }

    fn add_digit(&mut self, row: i64, col: i64, ch: char) {
        self.value = self.value * 10 + (((ch as u8) - b'0') as i64);
        self.points.extend([
            (row - 1, col + 1),
            (row, col + 1),
            (row + 1, col + 1),
        ])
    }

    fn extract(&self) -> i64 {
        self.value
    }

    fn next_to_symbol(&self, syms: &HashSet<(i64, i64)>) -> bool {
        self.points.intersection(syms).next().is_some()
    }
}

pub struct Schema {
    nums: Vec<PartNumber>,
    syms: HashSet<(i64, i64)>,
    gears: HashSet<(i64, i64)>,
}

impl Schema {
    pub fn new(input: &str) -> Self {
        let mut schema = Self {
            nums: Vec::new(),
            syms: HashSet::new(),
            gears: HashSet::new(),
        };
        let mut cur_number: Option<PartNumber> = None;

        for (row, line) in input.lines().enumerate() {
            for (col, ch) in line.chars().enumerate() {
                if ch.is_ascii_digit() {
                    if let Some(ref mut num) = cur_number {
                        num.add_digit(row as i64, col as i64, ch);
                    } else {
                        cur_number = Some(PartNumber::new(row as i64, col as i64, ch));
                    }
                } else {
                    if let Some(num) = cur_number.take() {
                        schema.nums.push(num);
                    }
                    if ch != '.' {
                        schema.syms.insert((row as i64, col as i64));
                        if ch == '*' {
                            schema.gears.insert((row as i64, col as i64));
                        }
                    }
                }
            }
        }

        schema
    }
}

#[aoc(day3, part1)]
pub fn solve_part1(input: &str) -> i64 {
    let schema = Schema::new(input);
    schema.nums
        .iter()
        .filter(|num| num.next_to_symbol(&schema.syms))
        .map(PartNumber::extract)
        .sum::<i64>()
}

#[aoc(day3, part2)]
pub fn solve_part2(input: &str) -> i64 {
    let mut total = 0;
    let schema = Schema::new(input);

    'next_gear: for gear in schema.gears {
        let mut matches = Vec::new();
        for num in &schema.nums {
            if num.points.contains(&gear) {
                if matches.len() == 2 {
                    continue 'next_gear;
                }
                matches.push(num.value);
            }
        }
        if matches.len() == 2 {
            total += matches[0] * matches[1];
        }
    }
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str =
        r#"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."#;

    #[test]
    fn test_part1() {
        assert_eq!(solve_part1(SAMPLE), 4361);
    }

    #[test]
    fn test_part2() {
        assert_eq!(solve_part2(SAMPLE), 467835);
    }
}
