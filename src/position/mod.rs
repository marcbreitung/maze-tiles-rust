use direction::Direction;

/// Defines the tiles position inside the maze
#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Position {
    pub column: u32,
    pub row: u32,
}

impl Position {
    /// Returns new Position at column 0 and row 0
    pub fn new() -> Self {
        Position { column: 0, row: 0 }
    }
    /// Returns a vector with positions from direct neighbours as tuples
    pub fn get_neighbours_positions(&self) -> Vec<Position> {
        let mut result = Vec::new();
        let directions = Direction::get_directions();
        let columns = vec![self.column as i32, self.column as i32 + 1, self.column as i32, self.column as i32 - 1];
        let rows = vec![self.row as i32 - 1, self.row as i32, self.row as i32 + 1, self.row as i32];

        let directions_positions = directions.iter().zip(columns.iter().zip(rows.iter()));

        for (direction, position) in directions_positions {
            let (column, row) = position;
            if *column >= 0 && *row >= 0 {
                match direction {
                    Direction::Top => { result.push(Position { column: *column as u32, row: *row as u32 }); }
                    Direction::Right => { result.push(Position { column: *column as u32, row: *row as u32 }); }
                    Direction::Bottom => { result.push(Position { column: *column as u32, row: *row as u32 }); }
                    Direction::Left => { result.push(Position { column: *column as u32, row: *row as u32 }); }
                }
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use position::Position;

    #[test]
    fn position_new() {
        let position = Position::new();
        assert_eq!(0, position.column);
        assert_eq!(0, position.row);
    }

    #[test]
    fn get_neighbours_positions() {
        let position = Position { column: 2, row: 2 };
        assert_eq!(vec![
            Position { column: 2, row: 1 },
            Position { column: 3, row: 2 },
            Position { column: 2, row: 3 },
            Position { column: 1, row: 2 }
        ], position.get_neighbours_positions());
    }
}
