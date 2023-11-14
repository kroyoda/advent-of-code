struct Position(usize, usize);

enum Instruction {
    TurnOn(Position, Position),
    TurnOff(Position, Position),
    Toggle(Position, Position),
}

struct Grid {
    instructions: Vec<Instruction>,
    lights: Vec<bool>,
}

impl Grid {
    const WIDTH: usize = 1000;
    const HEIGHT: usize = 1000;

    fn new(instructions: Vec<Instruction>) -> Grid {
        Grid {
            instructions,
            lights: vec![false; Grid::WIDTH * Grid::HEIGHT],
        }
    }
}

impl std::ops::Index<(usize, usize)> for Grid {
    type Output = bool;

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
pub fn parse_input(input: &str) -> Result<Grid, ()> {
    // Depending on the first word of line, the second word may be
    // the first coordinate or a subcommand "on" or "off".
    let mut instructions = Vec::new();
}
