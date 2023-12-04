use std::cmp::Ordering;

use itertools::Itertools;

fn count_combinations(data: &[u8], target: u8) -> u32 {
    data.iter()
        .enumerate()
        .map(|(i, elem)| {
            match elem.cmp(&target) {
                Ordering::Greater => 0,
                Ordering::Equal => 1,
                Ordering::Less => count_combinations(&data[i + 1..], target - elem),
            }
        })
        .sum()
}

fn count_least_depth_combinations(data: &[u8], target: u8, depth: u8) -> (u32, u8) {
    data.iter()
        .enumerate()
        .map(|(i, elem)| {
            match elem.cmp(&target) {
                Ordering::Greater => (0, u8::MAX),
                Ordering::Equal => (1, depth + 1),
                Ordering::Less =>
                    count_least_depth_combinations(&data[i + 1..], target - elem, depth + 1),
            }
        })
        .min_set_by_key(|&(_, d)| d)
        .iter()
        .fold((0, u8::MAX), |(acc, _), &(c, d)| (acc + c, d))
}

fn n_combinations<const N: usize>(input: &str, target: u8) -> u32 {
    let mut values = [0_u8; N];
    for (val, line) in values.iter_mut().zip(input.lines()) {
        *val = line.parse().unwrap();
    }
    count_combinations(&values, target)
}

fn minimal_combinations<const N: usize>(input: &str, target: u8) -> u32 {
    let mut values = [0_u8; N];
    for (val, line) in values.iter_mut().zip(input.lines()) {
        *val = line.parse().unwrap();
    }
    count_least_depth_combinations(&values, target, 0).0
}

#[aoc(day17, part1)]
fn part1(input: &str) -> u32 {
    n_combinations::<20>(input, 150)
}

#[aoc(day17, part2)]
fn part2(input: &str) -> u32 {
    minimal_combinations::<20>(input, 150)
}
