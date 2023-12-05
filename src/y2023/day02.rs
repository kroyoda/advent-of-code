use regex::Regex;

#[derive(Debug, PartialEq, Copy, Clone)]
enum Cube {
    Red,
    Green,
    Blue,
}

type Subset = [(Cube, usize); 3];

pub struct Game {
    id: usize,
    subsets: Vec<Subset>,
}

impl Game {
    fn max(&self) -> Subset {
        let max_red = self.subsets
            .iter()
            .filter_map(|subset| {
                if subset[0].0 == Cube::Red { Some(subset[0]) } else { None }
            })
            .max_by_key(|cube| cube.1)
            .unwrap();

        let max_green = self.subsets
            .iter()
            .filter_map(|subset| {
                if subset[1].0 == Cube::Green { Some(subset[1]) } else { None }
            })
            .max_by_key(|cube| cube.1)
            .unwrap();

        let max_blue = self.subsets
            .iter()
            .filter_map(|subset| {
                if subset[2].0 == Cube::Blue { Some(subset[2]) } else { None }
            })
            .max_by_key(|cube| cube.1)
            .unwrap();

        [max_red, max_green, max_blue]
    }
}

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<Game> {
    let pattern = Regex::new(r"Game (\d+):(.*)").expect("Failed to compile regex");
    input
        .lines()
        .map(|line| {
            let mut game = Game {
                id: 0,
                subsets: Vec::new(),
            };

            if let Some(catpure) = pattern.captures(line) {
                let game_id = catpure.get(1).unwrap().as_str().parse::<usize>().unwrap();
                let subsets_str = catpure.get(2).unwrap().as_str().trim();

                if !subsets_str.is_empty() {
                    let subsets: Vec<&str> = subsets_str.split(';').collect();

                    game.id = game_id;
                    game.subsets = subsets
                        .iter()
                        .map(|s| {
                            let mut subset = [
                                (Cube::Red, 0),
                                (Cube::Green, 0),
                                (Cube::Blue, 0),
                            ];

                            let cubes: Vec<&str> = s.split(',').collect();

                            for cube in cubes {
                                let cube = cube.trim();
                                let cube = cube.split(' ').collect::<Vec<&str>>();

                                let color = match cube[1] {
                                    "red" => Cube::Red,
                                    "green" => Cube::Green,
                                    "blue" => Cube::Blue,
                                    _ => panic!("Unknown color"),
                                };

                                let count = cube[0].parse::<usize>().unwrap();

                                match color {
                                    Cube::Red => {
                                        subset[0] = (color, count);
                                    }
                                    Cube::Green => {
                                        subset[1] = (color, count);
                                    }
                                    Cube::Blue => {
                                        subset[2] = (color, count);
                                    }
                                }
                            }

                            subset
                        })
                        .collect();
                }
            }

            game
        })
        .collect()
}

#[aoc(day2, part1)]
pub fn solve_part1(input: &[Game]) -> usize {
    // The Elf would first like to know which games would have been possible
    // if the bag contained only 12 red cubes, 13 green cubes, and 14 blue cubes?
    let want = [
        (Cube::Red, 12),
        (Cube::Green, 13),
        (Cube::Blue, 14),
    ];

    input
        .iter()
        .filter(|game| {
            let max = game.max();

            max[0].1 <= want[0].1 && max[1].1 <= want[1].1 && max[2].1 <= want[2].1
        })
        .map(|game| game.id)
        .sum()
}

#[aoc(day2, part2)]
pub fn solve_part2(input: &[Game]) -> usize {
    // Fewest number of cubes of each color that could have been in the bag to make the game possible?
    let min = [
        (Cube::Red, 0),
        (Cube::Green, 0),
        (Cube::Blue, 0),
    ];

    input
        .iter()
        .map(|game| {
            let max = game.max();

            let red = max[0].1;
            let green = max[1].1;
            let blue = max[2].1;

            let red = if red > min[0].1 { red - min[0].1 } else { 0 };
            let green = if green > min[1].1 { green - min[1].1 } else { 0 };
            let blue = if blue > min[2].1 { blue - min[2].1 } else { 0 };

            red * green * blue
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str =
        r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
    Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
    Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
    Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
    Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"#;

    #[test]
    fn test_input_generator() {
        let games = input_generator(TEST_INPUT);
        let first_game_subsets = &games[0].subsets;
        let first_game_subset1 = &first_game_subsets[0];

        assert_eq!(first_game_subset1[0].0, Cube::Red);
        assert_eq!(first_game_subset1[0].1, 4);

        assert_eq!(first_game_subset1[1].0, Cube::Green);
        assert_eq!(first_game_subset1[1].1, 0);

        assert_eq!(first_game_subset1[2].0, Cube::Blue);
        assert_eq!(first_game_subset1[2].1, 3);
    }

    #[test]
    fn test_game_impl() {
        let games = input_generator(TEST_INPUT);
        let game = &games[0];

        assert_eq!(game.max(), [
            (Cube::Red, 4),
            (Cube::Green, 2),
            (Cube::Blue, 6),
        ]);
    }

    #[test]
    fn test_part1() {
        let games = input_generator(TEST_INPUT);
        assert_eq!(solve_part1(&games), 8);
    }

    #[test]
    fn test_part2() {
        let games = input_generator(TEST_INPUT);
        assert_eq!(solve_part2(&games), 2286);
    }
}
