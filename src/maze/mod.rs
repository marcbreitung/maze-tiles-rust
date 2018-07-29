use tile::Tile;

/// The maze contains the tiles and calculates the walkable path based on the given tiles
pub struct Maze<'a> {
    pub tiles: Vec<Tile<'a>>
}

impl<'a> Maze<'a> {
    /// Returns a maze
    #[allow(dead_code)]
    pub fn new() -> Self {
        Maze { tiles: Vec::new() }
    }

    /// Adds a tile to the maze
    #[allow(dead_code)]
    pub fn add_tile(&mut self, tile: Tile<'a>) {
        if !self.tiles.iter().any(|t| t.position == tile.position) {
            self.tiles.push(tile);
        }
    }
}

#[cfg(test)]
mod tests {
    use ::maze::Maze;
    use ::tile::Tile;

    #[test]
    fn maze_new() {
        let maze = Maze::new();
        assert_eq!(0, maze.tiles.len());
    }

    #[test]
    fn maze_add_tile() {
        let mut maze = Maze::new();
        assert_eq!(0, maze.tiles.len());

        let tile_a = Tile::new();
        let tile_b = Tile::new();
        maze.add_tile(tile_a);
        maze.add_tile(tile_b);

        assert_eq!(1, maze.tiles.len());
    }
}
