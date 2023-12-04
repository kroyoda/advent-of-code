#[aoc(day1, part1)]
pub fn solve_part1(input: &[u8]) -> i32 {
    input.iter().fold(0, |sum, c| match c {
        b'(' => sum + 1,
        b')' => sum - 1,
        _ => unreachable!("Invalid input")
    })
}

#[aoc(day1, part2)]
pub fn solve_part2(input: &str) -> usize {
    let mut sum: u32 = 0;

    for (i, c) in input.as_bytes().iter().enumerate() {
        match c {
            b'(' => sum += 1,
            b')' => if let Some(s) = sum.checked_sub(1) {
                sum = s;
            } else {
                return i + 1;
            },
            _ => unreachable!("Invalid input")
        }
    }

    unreachable!("Invalid input")
}