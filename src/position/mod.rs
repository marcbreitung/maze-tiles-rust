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
}

#[cfg(test)]
mod tests {
    use position::Position;

    #[test]
    fn position_new() {
        let tile = Position::new();
        assert_eq!(0, tile.column);
        assert_eq!(0, tile.row);
    }
}
