use tile::field::Field;
use tile::position::Position;
use tile::size::Size;
use tile::tile::Tile;

#[derive(Debug, Clone, PartialEq)]
pub struct Maze {
    pub size: Size,
    pub fields: Vec<Field>,
}

impl Maze {
    pub fn new(width: u32, height: u32) -> Self {
        let size = Size::new(width, height);
        let fields = vec![Field::None; size.len()];
        Self {
            size,
            fields,
        }
    }

    pub fn add_tile(&mut self, tile: Tile) {
        let mut row = 0;
        for (index, field) in tile.fields.iter().enumerate() {
            if index > 0 && (index % (tile.size.width) as usize) == 0 {
                row = row + 1;
            }
            let position = Position::new((index % (tile.size.width) as usize) as u32, row);
            self.update_field(field.clone(), self.get_index(position.clone()));
        }
    }

    pub fn update_field(&mut self, field: Field, index: usize) {
        if let Some(field_element) = self.fields.get_mut(index) {
            *field_element = field;
        }
    }

    pub fn get_index(&self, position: Position) -> usize {
        (position.y * self.size.width + position.x) as usize
    }
}

#[cfg(test)]
mod test {
    use maze::maze::Maze;
    use tile::position::Position;
    use tile::field::Field;
    use tile::tile::Tile;
    use tile::size::Size;

    #[test]
    fn maze_new() {
        let maze = Maze::new(10, 10);
        assert_eq!(100, maze.fields.len());
    }

    #[test]
    fn maze_get_index() {
        let maze = Maze::new(10, 10);
        let position = Position::new(5, 5);
        assert_eq!(55, maze.get_index(position));
    }

    #[test]
    fn maze_update_field() {
        let mut maze = Maze::new(10, 10);
        let position = Position::new(5, 5);
        let index = maze.get_index(position);
        maze.update_field(Field::Path, index);
        assert_eq!(Field::Path, maze.fields[index]);
    }

    #[test]
    fn maze_add_tile() {
        let mut maze = Maze::new(10, 10);
        let tile = Tile::new(
            Position::new(0, 0),
            Size::new(3, 3),
            vec![
                Field::Ground, Field::Path, Field::Ground,
                Field::Ground, Field::Path, Field::Ground,
                Field::Ground, Field::Path, Field::Ground,
            ]);
        maze.add_tile(tile);
        assert_eq!(Field::Ground, maze.fields[0]);
        assert_eq!(Field::Path, maze.fields[1]);
    }
}
