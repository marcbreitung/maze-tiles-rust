use std::collections::HashMap;

use position::Position;
use tile::Tile;
use tile_group::TileGroup;
use size::Size;

/// A maze is a group of ``Tiles``
pub struct Maze {
    pub tiles: HashMap<Position, Tile>,
    size: Size,
}

impl Maze {
    /// Returns a ``Maze`` with the given width and height
    ///
    /// # Example
    /// ```
    /// use maze_tiles_rust::maze::Maze;
    ///
    /// let maze = Maze::new(20, 10);
    /// assert!(maze.tiles.capacity() >= 200);
    /// ```
    pub fn new(columns: u32, rows: u32) -> Self {
        let tiles = HashMap::with_capacity((columns * rows) as usize);
        let size = Size::new(columns, rows);
        Self {
            tiles,
            size,
        }
    }

    /// Adds a ``Tile`` if currently no tile exists at the given ``Position``
    ///
    /// # Example
    /// ```
    /// use maze_tiles_rust::maze::Maze;
    /// use maze_tiles_rust::tile::Tile;
    /// use maze_tiles_rust::position::Position;
    /// use maze_tiles_rust::field::Field;
    ///
    /// let mut maze = Maze::new(6, 6);
    /// let tile = Tile::new(Position::new(0, 0), Field::Ground);
    /// maze.add_tile(tile);
    /// assert_eq!(maze.tiles.contains_key(&Position::new(0, 0)), true);
    /// ```
    pub fn add_tile(&mut self, tile: Tile) {
        if tile.position.x < self.size.columns && tile.position.y < self.size.rows {
            if self.tiles.contains_key(&tile.position) == false {
                self.tiles.insert(tile.position.clone(), tile);
            }
        }
    }

    /// Adds a ``TileGroup``
    /// ```
    /// use maze_tiles_rust::maze::Maze;
    /// use maze_tiles_rust::tile_group::TileGroup;
    /// use maze_tiles_rust::position::Position;
    /// use maze_tiles_rust::size::Size;
    /// use maze_tiles_rust::field::Field;
    ///
    /// let mut maze = Maze::new(6, 6);
    /// let tile_group = TileGroup::new(
    /// Position::new(3, 3),
    /// Size::new(3, 3),
    /// vec![
    ///     Field::Ground, Field::Path, Field::Ground,
    ///     Field::Ground, Field::Path, Field::Ground,
    ///     Field::Ground, Field::Path, Field::Ground,
    /// ],
    /// );
    /// maze.add_tile_group(tile_group);
    /// ```
    pub fn add_tile_group(&mut self, tile_group: TileGroup) {
        for (y, fields) in tile_group.fields.chunks(tile_group.size.columns as usize).enumerate() {
            for (x, field) in fields.iter().enumerate() {
                let position = Position::new(x as u32, y as u32) + tile_group.origin.clone();
                let tile = Tile::new(position, *field);
                self.add_tile(tile);
            }
        }
    }
}

#[cfg(test)]
mod test {
    use field::Field;
    use maze::Maze;
    use position::Position;
    use size::Size;
    use tile::Tile;
    use tile_group::TileGroup;

    #[test]
    fn new_test() {
        let maze = Maze::new(20, 10);
        assert!(maze.tiles.capacity() >= 200);
    }

    #[test]
    fn add_tile_test() {
        let mut maze = Maze::new(6, 6);
        let tile = Tile::new(Position::new(0, 0), Field::Ground);
        maze.add_tile(tile);
        assert_eq!(maze.tiles.contains_key(&Position::new(0, 0)), true);
    }

    #[test]
    fn add_tile_group() {
        let mut maze = Maze::new(6, 6);
        let tile_group = TileGroup::new(
            Position::new(3, 3),
            Size::new(3, 3),
            vec![
                Field::Ground, Field::Path, Field::Ground,
                Field::Ground, Field::Path, Field::Ground,
                Field::Ground, Field::Path, Field::Ground,
            ],
        );
        maze.add_tile_group(tile_group);
        assert_eq!(false, maze.tiles.contains_key(&Position::new(0, 0)));
        assert_eq!(true, maze.tiles.contains_key(&Position::new(3, 3)));
    }
}
