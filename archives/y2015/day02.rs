type Gift = (u32, u32, u32);

#[aoc_generator(day2)]
pub fn parse_input(input: &str) -> Vec<Gift> {
    input
        .lines()
        .map(|l| {
            let mut gift = l
                .trim()
                .split('x')
                .map(|d| d.parse::<u32>().unwrap());
            (gift.next().unwrap(), gift.next().unwrap(), gift.next().unwrap())
        })
        .collect()
}

pub fn solve_part1(input: &[Gift]) -> u32 {
    input
        .iter()
        .map(|&(l, w, h)| {
            let (s1, s2) = smallest_side((l, w, h));

            2 * l * w + 2 * w * h + 2 * h * l + s1 * s2
        })
        .sum()
}

#[aoc(day2, part2)]
pub fn solve_part2(input: &[Gift]) -> u32 {
    input
        .iter()
        .map(|&(l, w, h)| {
            let (s1, s2) = smallest_side((l, w, h));

            (s1 + s2) * 2 + l * w * h
        })
        .sum()
}

fn smallest_side((l, w, h): Gift) -> (u32, u32) {
    let mut vec = [l, w, h];
    vec.sort();

    (vec[0], vec[1])
}
