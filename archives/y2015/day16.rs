type Sue = [u8; 10];

#[aoc_generator(day16)]
pub fn input_generator(input: &str) -> Vec<(u16, Sue)> {
    input
        .lines()
        .map(|line| {
            let mut sue = [u8::MAX; 10];
            let (sue_number, data) = line.split_once(": ").expect("Invalid line");
            let sue_number: u16 = sue_number
                .trim_start_matches("Sue ")
                .parse()
                .expect("Invalid Sue number");

            for item in data.split(", ") {
                let (key, value) = item.split_once(": ").expect("Invalid data");
                let value: u8 = value.parse().expect("Invalid value");
                match key {
                    "children" => {
                        sue[0] = value;
                    }
                    "cats" => {
                        sue[1] = value;
                    }
                    "samoyeds" => {
                        sue[2] = value;
                    }
                    "pomeranians" => {
                        sue[3] = value;
                    }
                    "akitas" => {
                        sue[4] = value;
                    }
                    "vizslas" => {
                        sue[5] = value;
                    }
                    "goldfish" => {
                        sue[6] = value;
                    }
                    "trees" => {
                        sue[7] = value;
                    }
                    "cars" => {
                        sue[8] = value;
                    }
                    "perfumes" => {
                        sue[9] = value;
                    }
                    _ => unreachable!("Invalid key"),
                }
            }

            (sue_number, sue)
        })
        .collect()
}

fn is_match(sue: Sue, target: Sue) -> bool {
    for (clue, target) in sue.into_iter().zip(target.into_iter()) {
        if clue != u8::MAX && clue != target {
            return false;
        }
    }
    true
}

fn is_real_match(sue: Sue, target: Sue) -> bool {
    for (i, clue) in sue.into_iter().enumerate() {
        if clue != u8::MAX {
            match i {
                1 | 7 => {
                    if clue <= target[i] {
                        return false;
                    }
                }
                3 | 6 => {
                    if clue >= target[i] {
                        return false;
                    }
                }
                _ => {
                    if clue != target[i] {
                        return false;
                    }
                }
            }
        }
    }
    true
}

const TARGET: Sue = [3, 7, 2, 3, 0, 0, 5, 3, 2, 1];

#[aoc(day16, part1)]
pub fn solve_part1(input: &[(u16, Sue)]) -> u16 {
    input
        .iter()
        .filter(|(_, sue)| is_match(*sue, TARGET))
        .map(|(sue_number, _)| *sue_number)
        .next()
        .expect("No match found")
}

#[aoc(day16, part2)]
pub fn solve_part2(input: &[(u16, Sue)]) -> u16 {
    input
        .iter()
        .filter(|(_, sue)| is_real_match(*sue, TARGET))
        .map(|(sue_number, _)| *sue_number)
        .next()
        .expect("No match found")
}
