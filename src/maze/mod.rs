use tile::Tile;
use position::Position;

/// The maze contains the tiles and calculates the walkable path based on the given tiles
pub struct Maze {
    pub tiles: Vec<Tile>
}

impl Maze {
    /// Returns a new maze
    pub fn new() -> Self {
        Maze { tiles: Vec::new() }
    }

    /// Adds a tile to the maze
    pub fn add_tile(&mut self, tile: Tile) {
        if !self.tiles.iter().any(|t| t.position == tile.position) {
            self.tiles.push(tile);
        }
    }

    /// Adds a tile to the maze at the given position
    pub fn add_tile_at_position(&mut self, mut tile: Tile, position: Position) {
        tile.position = position;
        self.update_tile(&mut tile);
        self.add_tile(tile);
    }

    /// Checks if tile at given position already exists
    pub fn has_tile_at_position(&self, position: &Position) -> bool {
        self.tiles.iter().any(|t| t.position == *position)
    }

    /// Updates the neighbours
    fn update_tile(&mut self, tile: &mut Tile) {
        let positions = tile.position.get_neighbours_positions();
        for position in positions.iter() {
            if self.has_tile_at_position(&position) {
                tile.neighbours.push(*position);
            }
        }
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
        let position_true = Position { column: 1, row: 1 };
        let position_false = Position { column: 2, row: 2 };

        maze.add_tile_at_position(Tile::new(), position_true);

        assert_eq!(true, maze.has_tile_at_position(&position_true));
        assert_eq!(false, maze.has_tile_at_position(&position_false));
    }

    #[test]
    fn update_tile() {
        let mut maze = Maze::new();
        maze.add_tile_at_position(Tile::with_walkable([false, true, false, false, true, true, false, false, false]), Position { column: 0, row: 0 });
        maze.add_tile_at_position(Tile::with_walkable([false, false, false, true, true, true, false, false, false]), Position { column: 1, row: 0 });
        maze.add_tile_at_position(Tile::with_walkable([false, false, false, true, true, true, false, false, false]), Position { column: 2, row: 0 });
    }
}
