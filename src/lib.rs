pub mod template;

pub struct Grid {
    grid: Vec<Vec<char>>,
}

impl Grid {
    pub fn new(input: &str) -> Self {
        let grid = input
            .lines()
            .map(|line| line.chars().collect())
            .collect();
        Self { grid }
    }

    pub fn get(&self, x: usize, y: usize) -> char {
        self.grid[y][x]
    }

    /** find the positions of a character in the grid */
    pub fn find(&self, c: char) -> Vec<(usize, usize)> {
        let mut positions = Vec::new();
        for (y, row) in self.grid.iter().enumerate() {
            for (x, &cell) in row.iter().enumerate() {
                if cell == c {
                    positions.push((x, y));
                }
            }
        }
        positions
    }

    /** is_valid checks if a position is within the grid */
    pub fn is_valid(&self, x: i32, y: i32) -> bool {
        x >= 0 && y >= 0 && y < (self.grid.len() as i32) && x < (self.grid[y as usize].len() as i32)
    }

    /** given an origin, a length, and a direction, returns the characters on this path */
    pub fn path(&self, origin: (i32, i32), length: i32, direction: (i32, i32)) -> Vec<char> {
        let mut path = Vec::new();
        let (dx, dy) = direction;
        let (mut x, mut y) = origin;
        for _ in 0..length {
            if !self.is_valid(x, y) {
                break;
            }
            path.push(self.grid[y as usize][x as usize]);
            x += dx;
            y += dy;
        }
        path
    }
}

pub fn add_tuples(t1: (i32, i32), t2: (i32, i32)) -> (i32, i32) {
    (t1.0 + t2.0, t1.1 + t2.1)
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grid_structure() {
        let input = "ABC\nDEF\nGHI";
        let grid = Grid::new(input);
        assert_eq!(grid.grid, vec![vec!['A', 'B', 'C'], vec!['D', 'E', 'F'], vec!['G', 'H', 'I']]);
    }

    #[test]
    fn test_grid_find() {
        let input = "TCGAG\nAGTAC\nGACGT";
        let grid = Grid::new(input);
        assert_eq!(grid.find('A').len(), 4);
    }

    #[test]
    fn test_grid_is_valid() {
        let input = "123\n456\n789";
        let grid = Grid::new(input);
        assert_eq!(grid.is_valid(0, 0), true);
        assert_eq!(grid.is_valid(3, -1), false);
    }

    #[test]
    fn test_grid_path() {
        let input = "123\n456\n789";
        let grid = Grid::new(input);
        assert_eq!(grid.path((0, 0), 3, (1, 0)), vec!['1', '2', '3']);
        assert_eq!(grid.path((0, 0), 3, (0, 1)), vec!['1', '4', '7']);
        assert_eq!(grid.path((0, 0), 3, (1, 1)), vec!['1', '5', '9']);
    }
}
