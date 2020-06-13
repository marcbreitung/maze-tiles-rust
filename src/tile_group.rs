use position::Position;
use size::Size;
use field::Field;

pub struct TileGroup {
    pub origin: Position,
    pub size: Size,
    pub fields: Vec<Field>,
}

impl TileGroup {
    pub fn new(origin: Position, size: Size, fields: Vec<Field>) -> Self {
        Self {
            origin,
            size,
            fields,
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
mod test {
    use tile_group::TileGroup;
    use position::Position;
    use size::Size;
    use field::Field;

    #[test]
    fn new_test() {
        let fields = vec![
            Field::Ground, Field::Path, Field::Ground,
            Field::Ground, Field::Path, Field::Ground,
            Field::Ground, Field::Path, Field::Ground,
        ];
        let tile_group = TileGroup::new(Position::new(0, 1), Size::new(3, 3), fields);
        assert_eq!(0, tile_group.origin.x);
        assert_eq!(1, tile_group.origin.y);
        assert_eq!(3, tile_group.size.width);
        assert_eq!(3, tile_group.size.height);
    }

    #[test]
    fn rotated_test() {
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
        let mut tile_group = TileGroup::new(Position::new(0, 1), Size::new(3, 3), fields);
        tile_group.rotate();
        assert_eq!(rotated_fields, tile_group.fields);
    }
}
