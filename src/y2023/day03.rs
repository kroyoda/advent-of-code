use crate::grid::{ Grid, Position };

#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> Grid<char> {
    let lines = input.lines();
    let dimensions = (lines.clone().count(), lines.clone().next().unwrap().len());

    Grid::new(
        dimensions,
        input
            .lines()
            .flat_map(|line| line.chars())
            .collect::<Vec<_>>()
    )
}

fn is_digit(c: char) -> bool {
    c.is_ascii_digit()
}

fn is_symbol(c: char) -> bool {
    !is_digit(c) && c != '.'
}

#[aoc(day3, part1)]
pub fn solve_part1(grid: &Grid<char>) -> usize {
    let mut out: usize = 0;
    let mut number_digits: Vec<(Position, &char)> = Vec::new();

    for (i, ch) in grid.iter().enumerate() {
        if is_digit(*ch) {
            number_digits.push((grid.position(i), ch));
            continue;
        }

        // Char is either a dot or a symbol
        // In any way, we may have ended a number
        if !number_digits.is_empty() {
            // But we only care if the number is surrounded by a symbol
            // Any digit that is surrounded by a symbol makes the number valid
            let mut valid = false;
            for (pos, _) in &number_digits {
                if
                    grid
                        .neighbors(*pos)
                        .iter()
                        .any(|n| is_symbol(**n))
                {
                    valid = true;
                    break;
                }
            }

            if !valid {
                number_digits.clear();
                continue;
            }

            // We have a valid number!
            let n_string = number_digits
                .iter()
                .map(|(_, ch)| ch.to_string())
                .collect::<String>();

            let number = n_string.parse::<usize>().unwrap();
            number_digits.clear();

            out += number;
        }
    }

    out
}

#[aoc(day3, part2)]
pub fn solve_part2(grid: &Grid<char>) -> usize {
    let mut out: usize = 0;

    out
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
    fn test_generator() {
        let grid = input_generator(SAMPLE);

        grid.assert_size((10, 10));
        grid.assert_valid();

        // println!("{}", grid);

        grid.assert_at((0, 1), '6');
        grid.assert_at((1, 3), '*');
    }

    #[test]
    fn test_part1() {
        let grid = input_generator(SAMPLE);

        assert_eq!(solve_part1(&grid), 4361);
    }

    #[test]
    fn test_part2() {
        let grid = input_generator(SAMPLE);

        assert_eq!(solve_part2(&grid), 467835);
    }
}
