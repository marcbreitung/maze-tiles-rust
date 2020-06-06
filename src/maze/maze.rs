use std::collections::HashMap;

use tile::field::Field;
use tile::position::Position;
use tile::size::Size;
use tile::tile::Tile;

#[derive(Debug, Clone, PartialEq)]
pub struct Maze {
    size: Size,
    tiles: HashMap<Position, Tile>,
}

impl Maze {
    pub fn new(width: u32, height: u32) -> Self {
        let size = Size::new(width, height);
        let tiles = HashMap::new();
        Self {
            size,
            tiles,
        }
    }

    pub fn get_tile_at_position(&mut self, position: Position) -> Option<Tile> {
        if let Some(tile) = self.tiles.get(&position) {
            return Some(tile.clone());
        }
        None
    }

    pub fn add_tile(&mut self, tile: Tile) {
        self.tiles.insert(tile.position.clone(), tile);
    }

    pub fn get_path(&self) -> Vec<Field> {
        let mut fields = vec![Field::None; self.size.len()];
        for (_, tile) in self.tiles.iter().clone() {
            let mut row = 0;
            for (index, field) in tile.fields.iter().enumerate() {
                if index > 0 && (index % (tile.size.width) as usize) == 0 {
                    row = row + 1;
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
    use maze::maze::Maze;
    use tile::field::Field;
    use tile::position::Position;
    use tile::tile::Tile;

    #[test]
    fn maze_new() {
        let maze = Maze::new(10, 10);
        assert_eq!(10, maze.size.width);
        assert_eq!(10, maze.size.width);
    }

    #[test]
    fn add_tile() {
        let mut maze = Maze::new(10, 10);
        maze.add_tile(Tile::new_path());
        assert_eq!(1, maze.tiles.len());
    }

    #[test]
    fn maze_get_index() {
        let maze = Maze::new(10, 10);
        let position = Position::new(5, 5);
        assert_eq!(55, maze.get_index(position));
    }

    #[test]
    fn get_path() {
        let mut maze = Maze::new(10, 10);
        maze.add_tile(Tile::new_path());
        let path = maze.get_path();
        assert_eq!(Field::Ground, path[0]);
        assert_eq!(Field::Path, path[1]);
    }

    #[test]
    fn get_tile_at_position() {
        let mut maze = Maze::new(10, 10);
        maze.add_tile(Tile::new_path());
        if let Some(tile) = maze.get_tile_at_position(Position::new(0,0)) {
            assert_eq!(Field::Ground, tile.fields[0]);
        };
    }
}
