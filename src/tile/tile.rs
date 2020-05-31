use tile::field::Field;
use tile::position::Position;
use tile::size::Size;
use tile::field::Field::Path;

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
            position: Position::new(0, 0),
            size: Size::new(3, 3),
            fields: vec![
                Field::Ground, Field::Path, Field::Ground,
                Field::Ground, Field::Path, Field::Ground,
                Field::Ground, Field::Path, Field::Ground,
            ],
        }
    }

    pub fn rotate(&mut self) {
        let mut rotated = vec![vec![Field::None; self.size.width as usize]; self.size.height as usize];
        for (row_index, row) in self.fields.chunks(self.size.width as usize).enumerate() {
            for (column_index, column) in row.iter().enumerate() {
                rotated[column_index][row.len() - 1 - row_index] = *column;
            }
        }
        for (index, field) in rotated.iter().flatten().enumerate() {
            self.fields[index] = *field;
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

    #[test]
    fn tile_rotate() {
        let fields = vec![
            Field::Ground, Field::Path, Field::Ground,
            Field::Ground, Field::Path, Field::Ground,
            Field::Ground, Field::Path, Field::Ground,
        ];

        let rotated_fields = vec![
            Field::Ground, Field::Ground, Field::Ground,
            Field::Path,   Field::Path,   Field::Path,
            Field::Ground, Field::Ground, Field::Ground,
        ];

        let mut tile = Tile::new_path();
        assert_eq!(fields, tile.fields);

        tile.rotate();
        assert_eq!(rotated_fields, tile.fields);
    }
}