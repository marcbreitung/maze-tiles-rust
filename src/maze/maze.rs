use tile::field::Field;
use tile::position::Position;
use tile::size::Size;
use tile::tile::Tile;

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

    pub fn add_tile(&self, tile: Tile) {

    }

    pub fn get_index(&self, position: Position) -> usize {
        (position.x * self.size.width + position.x) as usize
    }
}

#[cfg(test)]
mod test {
    use maze::maze::Maze;
    use tile::position::Position;

    #[test]
    fn maze_new() {
        let maze = Maze::new(10, 10);
        assert_eq!(100, maze.fields.len());
    }

    #[test]
    fn maze_get_index() {
        let maze = Maze::new(10, 10);
        let position = Position::new(5,5);
        assert_eq!(55, maze.get_index(position));
    }
}
