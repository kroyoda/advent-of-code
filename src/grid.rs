use std::fmt::{ Debug, Display };

pub type Vec2<T> = (T, T);

pub type Position = Vec2<usize>;

pub struct Grid<T> {
    dimensions: Vec2<usize>,
    data: Vec<T>,
}

impl<T> Grid<T> {
    pub fn new(dimensions: (usize, usize), data: Vec<T>) -> Self {
        assert_eq!(dimensions.0 * dimensions.1, data.len());
        Self { dimensions, data }
    }

    fn flat_index(&self, position: Position) -> usize {
        assert!(position.0 < self.dimensions.0);
        assert!(position.1 < self.dimensions.1);

        let strides = (self.dimensions.1, 1);

        position.0 * strides.0 + position.1 * strides.1
    }

    pub fn position(&self, index: usize) -> Position {
        (index / self.dimensions.1, index % self.dimensions.1)
    }

    pub fn at(&self, position: Position) -> &T {
        &self.data[self.flat_index(position)]
    }

    pub fn next(&self, position: Position) -> Option<&T> {
        if position.1 + 1 == self.dimensions.1 {
            if position.0 + 1 == self.dimensions.0 {
                None
            } else {
                Some(self.at((position.0 + 1, 0)))
            }
        } else {
            Some(self.at((position.0, position.1 + 1)))
        }
    }

    pub fn next_with_pos(&self, position: Position) -> Option<(Position, &T)> {
        if position.1 + 1 == self.dimensions.1 {
            if position.0 + 1 == self.dimensions.0 {
                None
            } else {
                Some(((position.0 + 1, 0), self.at((position.0 + 1, 0))))
            }
        } else {
            Some(((position.0, position.1 + 1), self.at((position.0, position.1 + 1))))
        }
    }

    pub fn prev(&self, position: Position) -> Option<&T> {
        if position.1 == 0 {
            if position.0 == 0 {
                None
            } else {
                Some(self.at((position.0 - 1, self.dimensions.1 - 1)))
            }
        } else {
            Some(self.at((position.0, position.1 - 1)))
        }
    }

    pub fn prev_with_pos(&self, position: Position) -> Option<(Position, &T)> {
        if position.1 == 0 {
            if position.0 == 0 {
                None
            } else {
                Some((
                    (position.0 - 1, self.dimensions.1 - 1),
                    self.at((position.0 - 1, self.dimensions.1 - 1)),
                ))
            }
        } else {
            Some(((position.0, position.1 - 1), self.at((position.0, position.1 - 1))))
        }
    }

    pub fn neighbors(&self, position: Position) -> Vec<&T> {
        let mut out = Vec::with_capacity(8);
        self.each_neighbor_pos(position, |neighbor_pos| {
            out.push(self.at(neighbor_pos));
        });
        out
    }

    pub fn neighbors_with_pos(&self, position: Position) -> Vec<(Position, &T)> {
        let mut out = Vec::with_capacity(8);
        self.each_neighbor_pos(position, |neighbor_pos| {
            out.push((neighbor_pos, self.at(neighbor_pos)));
        });
        out
    }

    fn each_neighbor_pos(&self, position: Position, mut func: impl FnMut(Position)) {
        // Left neighbor
        if position.0 != 0 {
            func((position.0 - 1, position.1));
        }
        // Top neighbor
        if position.1 != 0 {
            func((position.0, position.1 - 1));
        }
        // Right neighbor
        if position.0 + 1 != self.dimensions.0 {
            func((position.0 + 1, position.1));
        }
        // Bottom neighbor
        if position.1 + 1 != self.dimensions.1 {
            func((position.0, position.1 + 1));
        }
        // Top left neighbor
        if position.0 != 0 && position.1 != 0 {
            func((position.0 - 1, position.1 - 1));
        }
        // Top right neighbor
        if position.0 + 1 != self.dimensions.0 && position.1 != 0 {
            func((position.0 + 1, position.1 - 1));
        }
        // Bottom left neighbor
        if position.0 != 0 && position.1 + 1 != self.dimensions.1 {
            func((position.0 - 1, position.1 + 1));
        }
        // Bottom right neighbor
        if position.0 + 1 != self.dimensions.0 && position.1 + 1 != self.dimensions.1 {
            func((position.0 + 1, position.1 + 1));
        }
    }

    pub fn width(&self) -> usize {
        self.dimensions.1
    }

    pub fn height(&self) -> usize {
        self.dimensions.0
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.data.iter()
    }

    pub fn assert_valid(&self) {
        assert_eq!(self.dimensions.0 * self.dimensions.1, self.data.len());
    }

    pub fn assert_size(&self, dimensions: (usize, usize)) {
        assert_eq!(self.dimensions, dimensions);
    }

    pub fn assert_at(&self, position: Position, value: T) where T: PartialEq + Debug {
        assert_eq!(self.at(position), &value);
    }
}

impl<T> Display for Grid<T> where T: Display {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut out = String::new();
        for row in 0..self.dimensions.0 {
            for col in 0..self.dimensions.1 {
                out.push_str(&format!("{}", self.at((row, col))));
            }
            out.push('\n');
        }
        write!(f, "{}", out)
    }
}
