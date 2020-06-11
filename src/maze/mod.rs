use std::collections::HashMap;

use tile::field::Field;
use tile::position::Position;
use tile::size::Size;
use tile::Tile;

#[derive(Debug, Clone, PartialEq)]
pub struct Maze {
    size: Size,
    tile_size: Size,
    tiles: HashMap<Position, Tile>,
}

impl Maze {
    pub fn new(width: u32, height: u32) -> Self {
        let size = Size::new(width, height);
        let tile_size = Size::new(3, 3);
        let tiles = HashMap::new();
        Self {
            size,
            tile_size,
            tiles,
        }
    }

    /// Returns the ``tile::Tile`` at the given position
    ///
    /// # Example
    ///
    /// ```
    /// use maze_tiles_rust::maze::Maze;
    /// use maze_tiles_rust::tile::Tile;
    /// use maze_tiles_rust::tile::field::Field;
    /// use maze_tiles_rust::tile::position::Position;
    ///
    /// let mut maze = Maze::new(9, 9);
    /// maze.add_tile(Tile::new_path());
    ///
    /// if let Some(mut tile) = maze.get_tile_at_position(Position::new(0, 0)) {
    ///     assert_eq!(Field::Path, tile.fields[1]);
    /// };
    /// ```
    pub fn get_tile_at_position(&self, position: Position) -> Option<Tile> {
        if let Some(tile) = self.tiles.get(&position) {
            return Some(tile.clone());
        }
        None
    }

    /// Returns the ``tile::Tile`` at the given index
    ///
    /// # Examle
    ///
    /// ```
    /// use maze_tiles_rust::tile::Tile;
    /// use maze_tiles_rust::tile::position::Position;
    /// use maze_tiles_rust::maze::Maze;
    /// use maze_tiles_rust::tile::field::Field;
    ///
    /// let mut tile = Tile::new_path();
    /// tile.position = Position::new(1, 0);
    /// let mut maze = Maze::new(9, 9);
    /// maze.add_tile(tile);
    ///
    /// if let Some(tile) = maze.get_tile_at_index(21) {
    ///     assert_eq!(Field::Ground, tile.fields[0]);
    ///     assert_eq!(Field::Path, tile.fields[1]);
    /// };
    /// ```
    pub fn get_tile_at_index(&self, index: u32) -> Option<Tile> {
        let row = (index as f32 / self.size.height as f32).floor() as u32;
        let col = index - (row * self.size.width);
        let tile_row = row / self.tile_size.height;
        let tile_col = col / self.tile_size.width;
        let position = Position::new(tile_col, tile_row);
        self.get_tile_at_position(position)
    }

    /// Adds a ``tile::Tile`` to the maze
    ///
    /// # Example
    /// ```
    /// use maze_tiles_rust::maze::Maze;
    /// use maze_tiles_rust::tile::Tile;
    ///
    /// let mut maze = Maze::new(9, 9);
    /// maze.add_tile(Tile::new_path());
    /// ```
    pub fn add_tile(&mut self, tile: Tile) {
        self.tiles.insert(tile.position.clone(), tile);
    }

    /// Returns the ``Field`` at a given ``Position``
    ///
    /// # Examle
    ///
    /// ```
    /// use maze_tiles_rust::maze::Maze;
    /// use maze_tiles_rust::tile::Tile;
    /// use maze_tiles_rust::tile::position::Position;
    /// use maze_tiles_rust::tile::field::Field;
    ///
    /// let mut maze = Maze::new(9, 9);
    /// maze.add_tile(Tile::new_path());
    /// if let Some(field) = maze.get_field_at_position(Position::new(1,0)) {
    ///     assert_eq!(Field::Path, field);
    /// };
    /// ```
    pub fn get_field_at_position(&self, position: Position) -> Option<Field> {
        let index = self.get_index(position);
        let path = self.get_path();
        if let Some(field) = path.get(index) {
           return Some(field.clone());
        }
        None
    }

    /// Returns the maze as a flat vector
    ///
    /// # Examle
    ///
    /// ```
    /// use maze_tiles_rust::maze::Maze;
    /// use maze_tiles_rust::tile::Tile;
    ///
    /// let mut maze = Maze::new(10, 10);
    /// maze.add_tile(Tile::new_path());
    /// let path = maze.get_path();
    /// ```
    pub fn get_path(&self) -> Vec<Field> {
        let mut fields = vec![Field::None; self.size.len()];
        for (_, tile) in self.tiles.iter().clone() {
            let mut row = 0;
            for (index, field) in tile.fields.iter().enumerate() {
                if index > 0 && (index % (tile.size.width) as usize) == 0 {
                    row += 1;
                }
                let position = Position::new((index % (tile.size.width) as usize) as u32, row);
                if let Some(field_element) = fields.get_mut(self.get_index(position.clone())) {
                    *field_element = *field;
                }
            }
        }
        fields
    }

    fn get_index(&self, position: Position) -> usize {
        (position.y * self.size.width + position.x) as usize
    }
}

#[cfg(test)]
mod test {
    use maze::Maze;
    use tile::field::Field;
    use tile::position::Position;
    use tile::Tile;

    #[test]
    fn maze_new() {
        let maze = Maze::new(9, 9);
        assert_eq!(9, maze.size.width);
        assert_eq!(9, maze.size.width);
    }

    #[test]
    fn add_tile() {
        let mut maze = Maze::new(9, 9);
        maze.add_tile(Tile::new_path());
        assert_eq!(1, maze.tiles.len());
    }

    #[test]
    fn maze_get_index() {
        let maze = Maze::new(9, 9);
        let position = Position::new(5, 5);
        assert_eq!(50, maze.get_index(position));
    }

    #[test]
    fn get_path() {
        let mut maze = Maze::new(9, 9);
        maze.add_tile(Tile::new_path());
        let path = maze.get_path();
        assert_eq!(Field::Ground, path[0]);
        assert_eq!(Field::Path, path[1]);
    }

    #[test]
    fn get_tile_at_position() {
        let mut maze = Maze::new(9, 9);
        maze.add_tile(Tile::new_path());
        if let Some(tile) = maze.get_tile_at_position(Position::new(0, 0)) {
            assert_eq!(Field::Ground, tile.fields[0]);
            assert_eq!(Field::Path, tile.fields[1]);
        };
    }

    #[test]
    fn get_tile_at_position_added_rotated() {
        let mut maze = Maze::new(9, 9);
        maze.add_tile(Tile::new_path());
        if let Some(mut tile) = maze.get_tile_at_position(Position::new(0, 0)) {
            assert_eq!(Field::Path, tile.fields[1]);
            tile.rotate();
            assert_eq!(Field::Ground, tile.fields[1]);
            maze.add_tile(tile);
        };
    }

    #[test]
    fn get_tile_at_index() {
        let mut tile = Tile::new_path();
        tile.position = Position::new(1, 0);
        let mut maze = Maze::new(9, 9);
        maze.add_tile(tile);
        if let Some(tile) = maze.get_tile_at_index(21) {
            assert_eq!(Field::Ground, tile.fields[0]);
            assert_eq!(Field::Path, tile.fields[1]);
        };
    }

    #[test]
    fn get_field_at_position() {
        let mut maze = Maze::new(9, 9);
        maze.add_tile(Tile::new_path());
        if let Some(field) = maze.get_field_at_position(Position::new(1,0)) {
            assert_eq!(Field::Path, field);
        };
    }
}
