use advent_of_code::{ add_tuples, grid::Grid };

advent_of_code::solution!(4);

const TOP_LEFT: (i32, i32) = (-1, -1);
const TOP: (i32, i32) = (0, -1);
const TOP_RIGHT: (i32, i32) = (1, -1);
const RIGHT: (i32, i32) = (1, 0);
const BOTTOM_RIGHT: (i32, i32) = (1, 1);
const BOTTOM: (i32, i32) = (0, 1);
const BOTTOM_LEFT: (i32, i32) = (-1, 1);
const LEFT: (i32, i32) = (-1, 0);

const DIRECTIONS: [(i32, i32); 8] = [
    TOP_LEFT,
    TOP,
    TOP_RIGHT,
    RIGHT,
    BOTTOM_RIGHT,
    BOTTOM,
    BOTTOM_LEFT,
    LEFT,
];

pub fn part_one(input: &str) -> Option<u32> {
    let grid = Grid::new(input);
    let positions = grid.find('X');
    let mut count = 0;

    let positions = positions
        .iter()
        .map(|(x, y)| (*x as i32, *y as i32))
        .collect::<Vec<(i32, i32)>>();

    let pattern = "XMAS";

    for (x, y) in positions {
        for (dx, dy) in &DIRECTIONS {
            let max_position = (x + dx * 3, y + dy * 3);
            if !grid.is_valid(max_position.0, max_position.1) {
                continue;
            }
            let path = grid.path((x, y), pattern.len() as i32, (*dx, *dy));
            if path.iter().collect::<String>() == pattern {
                count += 1;
            }
        }
    }
    Some(count)
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid = Grid::new(input);
    let positions: Vec<(i32, i32)> = grid
        .find('A')
        .iter()
        .map(|(x, y)| (*x as i32, *y as i32))
        .collect();
    let mut count = 0;

    let patterns = ["MAS", "SAM"];

    for origin in positions {
        let paths = [
            grid.path(add_tuples(origin, BOTTOM_LEFT), patterns[0].len() as i32, TOP_RIGHT),
            grid.path(add_tuples(origin, BOTTOM_RIGHT), patterns[1].len() as i32, TOP_LEFT),
        ];
        if
            paths.iter().all(|path| {
                let path = path.iter().collect::<String>();
                path == patterns[0] || path == patterns[1]
            })
        {
            count += 1;
        }
    }

    Some(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
