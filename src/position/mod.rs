use direction::Direction;

/// Defines the tiles position inside the maze
#[derive(PartialEq, Debug)]
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
    pub fn get_neighbours_positions(&self) -> Vec<(Direction, Position)> {
        let mut result = Vec::new();
        let directions = Direction::get_directions();
        let columns = vec![self.column as i32, self.column as i32 + 1, self.column as i32, self.column as i32 - 1];
        let rows = vec![self.row as i32 - 1, self.row as i32, self.row as i32 + 1, self.row as i32];

        let directions_positions = directions.iter().zip(columns.iter().zip(rows.iter()));

        for direction_position in directions_positions {
            let positions = direction_position.1;
            if *positions.1 >= 0 && *positions.0 >= 0 {
                match direction_position.0 {
                    Direction::Top => { result.push((Direction::Top, Position { column: *positions.0 as u32, row: *positions.1 as u32 })); }
                    Direction::Right => { result.push((Direction::Right, Position { column: *positions.0 as u32, row: *positions.1 as u32 })); }
                    Direction::Bottom => { result.push((Direction::Bottom, Position { column: *positions.0 as u32, row: *positions.1 as u32 })); }
                    Direction::Left => { result.push((Direction::Left, Position { column: *positions.0 as u32, row: *positions.1 as u32 })); }
                }
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use position::Position;
    use direction::Direction;

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
            (Direction::Top, Position { column: 2, row: 1 }),
            (Direction::Right, Position { column: 3, row: 2 }),
            (Direction::Bottom, Position { column: 2, row: 3 }),
            (Direction::Left, Position { column: 1, row: 2 })
        ], position.get_neighbours_positions());
    }
}
