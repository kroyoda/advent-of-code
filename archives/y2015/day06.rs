#[derive(Clone, PartialEq, Debug)]
struct Position(usize, usize);

impl std::str::FromStr for Position {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(',');
        let x = parts
            .next()
            .ok_or("Missing x")?
            .parse()
            .map_err(|_| "Cannot parse x")?;
        let y = parts
            .next()
            .ok_or("Missing y")?
            .parse()
            .map_err(|_| "Cannot parse y")?;

        Ok(Position(x, y))
    }
}

#[derive(Clone, PartialEq, Debug)]
enum Instruction {
    TurnOn(Position, Position),
    TurnOff(Position, Position),
    Toggle(Position, Position),
}

#[derive(Clone)]
pub struct Light {
    pub brightness: usize,
    pub is_on: bool,
}

impl Light {
    fn new() -> Light {
        Light {
            brightness: 0,
            is_on: false,
        }
    }

    fn turn_on(&mut self) {
        self.brightness += 1;
        self.is_on = true;
    }

    fn turn_off(&mut self) {
        if self.brightness > 0 {
            self.brightness -= 1;
        }
        self.is_on = false;
    }

    fn toggle(&mut self) {
        self.brightness += 2;
        self.is_on = !self.is_on;
    }
}

#[derive(Clone)]
pub struct Grid {
    instructions: Vec<Instruction>,
    lights: Vec<Light>,
}

impl Grid {
    const WIDTH: usize = 1000;
    const HEIGHT: usize = 1000;

    fn new(instructions: Vec<Instruction>) -> Grid {
        let mut lights = Vec::new();
        for _ in 0..Grid::WIDTH * Grid::HEIGHT {
            lights.push(Light::new());
        }
        Grid {
            instructions,
            lights,
        }
    }

    fn run_instructions(&mut self) {
        let instructions = self.instructions.clone();

        for instruction in instructions {
            match instruction {
                Instruction::TurnOn(Position(x1, y1), Position(x2, y2)) => {
                    for x in x1..=x2 {
                        for y in y1..=y2 {
                            self[(x, y)].turn_on();
                        }
                    }
                }
                Instruction::TurnOff(Position(x1, y1), Position(x2, y2)) => {
                    for x in x1..=x2 {
                        for y in y1..=y2 {
                            self[(x, y)].turn_off();
                        }
                    }
                }
                Instruction::Toggle(Position(x1, y1), Position(x2, y2)) => {
                    for x in x1..=x2 {
                        for y in y1..=y2 {
                            self[(x, y)].toggle();
                        }
                    }
                }
            }
        }
    }
}

impl std::ops::Index<(usize, usize)> for Grid {
    type Output = Light;

    fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
        &self.lights[y * Grid::WIDTH + x]
    }
}

impl std::ops::IndexMut<(usize, usize)> for Grid {
    fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut Self::Output {
        &mut self.lights[y * Grid::WIDTH + x]
    }
}

#[aoc_generator(day6)]
pub fn parse_input(input: &str) -> Result<Grid, String> {
    // Depending on the first word of line, the second word may be
    // the first coordinate or a subcommand "on" or "off".
    let mut instructions = Vec::new();
    let lines = input.lines();

    for line in lines {
        let mut words = line.split_whitespace();
        let first_word = words.next().ok_or("Empty line")?;
        let second_word = words.next().ok_or("Missing second word")?;

        let instruction = match first_word {
            "turn" =>
                match second_word {
                    "on" =>
                        Instruction::TurnOn(
                            words.next().ok_or("Missing third word")?.parse()?,
                            words.nth(1).ok_or("Missing fifth word")?.parse()?
                        ),
                    "off" =>
                        Instruction::TurnOff(
                            words.next().ok_or("Missing third word")?.parse()?,
                            words.nth(1).ok_or("Missing fifth word")?.parse()?
                        ),
                    _ => {
                        return Err(format!("Unexpected second word: {}", second_word));
                    }
                }
            "toggle" =>
                Instruction::Toggle(
                    second_word.parse()?,
                    words.nth(1).ok_or("Missing fourth word")?.parse()?
                ),
            _ => {
                return Err(format!("Unexpected first word: {}", first_word));
            }
        };

        instructions.push(instruction);
    }

    Ok(Grid::new(instructions))
}

#[aoc(day6, part1)]
pub fn solve_part1(grid: &Grid) -> usize {
    let mut grid = grid.clone();
    grid.run_instructions();

    grid.lights
        .iter()
        .filter(|light| light.is_on)
        .count()
}

#[aoc(day6, part2)]
pub fn solve_part2(grid: &Grid) -> usize {
    let mut grid = grid.clone();
    grid.run_instructions();

    grid.lights
        .iter()
        .map(|light| light.brightness)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        let input =
            "turn on 0,0 through 999,999\n\
                     toggle 0,0 through 999,0\n\
                     turn off 499,499 through 500,500";
        let grid = parse_input(input).unwrap();

        assert_eq!(grid.instructions.len(), 3);
        assert_eq!(grid.instructions[0], Instruction::TurnOn(Position(0, 0), Position(999, 999)));
        assert_eq!(grid.instructions[1], Instruction::Toggle(Position(0, 0), Position(999, 0)));
        assert_eq!(
            grid.instructions[2],
            Instruction::TurnOff(Position(499, 499), Position(500, 500))
        );
    }

    #[test]
    fn test_solve_part1() {
        let input =
            "turn on 0,0 through 999,999\n\
                     toggle 0,0 through 999,0\n\
                     turn off 499,499 through 500,500";
        let grid = parse_input(input).unwrap();

        assert_eq!(solve_part1(&grid), 998_996);
    }

    #[test]
    fn test_solve_part2() {
        let input = "turn on 0,0 through 0,0\n\
                     toggle 0,0 through 999,999";
        let mut grid = parse_input(input).unwrap();

        assert_eq!(
            &grid.lights
                .iter()
                .map(|light| light.brightness)
                .sum::<usize>(),
            &0
        );

        grid.run_instructions();

        assert_eq!(
            &grid.lights
                .iter()
                .map(|light| light.brightness)
                .sum::<usize>(),
            &2000001
        );
    }
}
