// A tile inside the maze
pub struct Tile<'a> {
    pub position: Position,
    pub neighbours: Vec<&'a Tile<'a>>,
}

impl<'a> Tile<'a> {
    // Returns a new tile with default position at 0 0
    #[allow(dead_code)]
    pub fn new() -> Self {
        let position = Position { row: 0, column: 0 };
        let neighbours = Vec::new();
        Tile { position, neighbours }
    }

    // Returns a new tile with the given column and row
    #[allow(dead_code)]
    pub fn with_row_column(r: u32, c: u32) -> Self {
        let mut tile = Tile::new();
        tile.position.row = r;
        tile.position.column = c;
        tile
    }

    // Adds a neighbour if tile to add has another position
    #[allow(dead_code)]
    pub fn add_neighbour(&mut self, n: &'a Tile) {
        if !self.neighbours.contains(&n) && self != n {
            self.neighbours.push(n);
        }
    }
}

impl<'a> PartialEq for Tile<'a> {
    fn eq(&self, other: &Tile) -> bool {
        self.position == other.position
    }
}

// Tiles position
pub struct Position {
    pub row: u32,
    pub column: u32,
}

impl PartialEq for Position {
    fn eq(&self, other: &Position) -> bool {
        self.row == other.row && self.column == other.column
    }
}

#[cfg(test)]
mod tests {
    use super::Position;
    use super::Tile;

    #[test]
    fn position_struct() {
        let position = Position {
            row: 1,
            column: 3,
        };
        assert_eq!(1, position.row);
        assert_eq!(3, position.column);
    }

    #[test]
    fn tile_struct() {
        let tile = Tile::new();
        assert_eq!(0, tile.position.row);
        assert_eq!(0, tile.position.column);
        assert_eq!(0, tile.neighbours.len());
    }

    #[test]
    fn tile_struct_add_neighbour() {
        let tile_a = Tile::with_row_column(1, 1);
        let tile_b = Tile::with_row_column(1, 2);
        let tile_c = Tile::with_row_column(0, 0);
        let tile_d = Tile::with_row_column(1, 1);
        let mut tile = Tile::new();

        tile.add_neighbour(&tile_a);
        tile.add_neighbour(&tile_b);
        tile.add_neighbour(&tile_c);
        tile.add_neighbour(&tile_d);

        assert_eq!(2, tile.neighbours.len());
    }
}
