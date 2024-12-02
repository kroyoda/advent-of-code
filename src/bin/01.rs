use std::collections::HashMap;

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let mut left_list: Vec<u32> = Vec::new();
    let mut right_list: Vec<u32> = Vec::new();

    let mut list_diff: u32 = 0;

    for line in input.lines() {
        // each line is two numbers separated by three spaces
        let numbers: Vec<u32> = line
            .split_whitespace()
            .map(|n| n.parse().unwrap())
            .collect();
        left_list.push(numbers[0]);
        right_list.push(numbers[1]);
    }

    // sort list in ascending order
    left_list.sort();
    right_list.sort();

    for (left, right) in left_list.iter().zip(right_list.iter()) {
        list_diff += left.abs_diff(*right);
    }

    Some(list_diff)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut left_list: Vec<u32> = Vec::new();
    let mut right_list: Vec<u32> = Vec::new();

    let mut list_similarity: u32 = 0;

    for line in input.lines() {
        // each line is two numbers separated by three spaces
        let numbers: Vec<u32> = line
            .split_whitespace()
            .map(|n| n.parse().unwrap())
            .collect();
        left_list.push(numbers[0]);
        right_list.push(numbers[1]);
    }

    // hashmap to count occurrences of each number in right_list
    let mut count_map: HashMap<u32, usize> = HashMap::new();
    for number in right_list.iter() {
        *count_map.entry(*number).or_insert(0) += 1;
    }

    for left in left_list.iter() {
        if let Some(count) = count_map.get_mut(left) {
            list_similarity += (*count as u32) * left;
        }
    }

    Some(list_similarity)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
