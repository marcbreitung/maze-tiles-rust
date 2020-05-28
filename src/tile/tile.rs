use tile::field::Field;
use tile::position::Position;
use tile::size::Size;

#[derive(Debug, Clone, PartialEq)]
pub struct Tile {
    pub position: Position,
    pub size: Size,
    pub fields: Vec<Field>,
}

impl Tile {
    pub fn new(position: Position, size: Size, fields: Vec<Field>) -> Self {
        Self {
            position,
            size,
            fields,
        }
    }

    pub fn new_path() -> Self {
        Self {
            position: Position::new(0,0),
            size: Size::new(3,3),
            fields: vec![
                Field::Ground, Field::Path, Field::Ground,
                Field::Ground, Field::Path, Field::Ground,
                Field::Ground, Field::Path, Field::Ground,
            ],
        }
    }
}

#[cfg(test)]
mod tests {
    use tile::field::Field;
    use tile::position::Position;
    use tile::size::Size;
    use tile::tile::Tile;

    #[test]
    fn tile_new() {
        let position = Position::new(0, 0);
        let size = Size::new(3, 3);
        let fields = vec![
            Field::Ground, Field::Path, Field::Ground,
            Field::Path, Field::Path, Field::Path,
            Field::Ground, Field::Path, Field::Ground,
        ];
        let tile = Tile::new(position, size, fields);
        assert_eq!(0, tile.position.x);
        assert_eq!(0, tile.position.y);
        assert_eq!(3, tile.size.width);
        assert_eq!(3, tile.size.height);
    }
}