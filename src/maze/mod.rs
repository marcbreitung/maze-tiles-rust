use tile::Tile;
use position::Position;

/// The maze contains the tiles and calculates the walkable path based on the given tiles
pub struct Maze<'a> {
    pub tiles: Vec<Tile<'a>>
}

impl<'a> Maze<'a> {
    /// Returns a new maze
    pub fn new() -> Self {
        Maze { tiles: Vec::new() }
    }

    /// Adds a tile to the maze
    pub fn add_tile(&mut self, tile: Tile<'a>) {
        if !self.tiles.iter().any(|t| t.position == tile.position) {
            self.tiles.push(tile);
        }
    }

    /// Adds a tile to the maze at the given position
    pub fn add_tile_at_position(&mut self, mut tile: Tile<'a>, position: Position) {
        tile.position = position;
        self.add_tile(tile);
    }

    /// Checks if tile at given position already exists
    pub fn has_tile_at_position(&mut self, position: Position) -> bool {
        self.tiles.iter().any(|t| t.position == position)
    }

    /// Returns a reference to the tile at the given position
    pub fn get_tile_at_position(&mut self, position: Position) -> Option<&Tile> {
        self.tiles.iter().find(|t| t.position == position)
    }
}

#[cfg(test)]
mod tests {
    use maze::Maze;
    use tile::Tile;
    use position::Position;

    #[test]
    fn maze_new() {
        let maze = Maze::new();
        assert_eq!(0, maze.tiles.len());
    }

    #[test]
    fn maze_add_tile() {
        let mut maze = Maze::new();
        assert_eq!(0, maze.tiles.len());

        maze.add_tile(Tile::new());
        maze.add_tile(Tile::new());

        assert_eq!(1, maze.tiles.len());
    }

    #[test]
    fn add_tile_at_position() {
        let mut maze = Maze::new();

        maze.add_tile_at_position(Tile::new(), Position { column: 1, row: 1 });
        maze.add_tile_at_position(Tile::new(), Position { column: 2, row: 1 });
        maze.add_tile_at_position(Tile::new(), Position { column: 1, row: 1 });

        assert_eq!(2, maze.tiles.len());
    }

    #[test]
    fn has_tile_at_position() {
        let mut maze = Maze::new();

        maze.add_tile_at_position(Tile::new(), Position { column: 1, row: 1 });

        assert_eq!(true, maze.has_tile_at_position(Position { column: 1, row: 1 }));
        assert_eq!(false, maze.has_tile_at_position(Position { column: 5, row: 5 }));
    }

    #[test]
    fn get_tile_at_position() {
        let mut maze = Maze::new();
        let tile = Tile::with_position(1, 1);

        maze.add_tile_at_position(Tile::new(), Position { column: 1, row: 1 });

        assert_eq!(Some(&tile), maze.get_tile_at_position(Position { column: 1, row: 1 }));
        assert_eq!(None, maze.get_tile_at_position(Position { column: 2, row: 1 }));
    }
}
