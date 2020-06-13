use field::Field;
use position::Position;

#[derive(Debug)]
pub struct Tile {
    pub position: Position,
    pub field: Field,
}

impl Tile {
    pub fn new(position: Position, field: Field) -> Self {
        Self {
            position,
            field,
        }
    }
}

#[cfg(test)]
mod test {
    use tile::Tile;
    use position::Position;
    use field::Field;

    #[test]
    fn new_test() {
        let tile = Tile::new(Position::new(1, 2), Field::Ground);
        assert_eq!(1, tile.position.x);
        assert_eq!(2, tile.position.y);
        assert_eq!(Field::Ground, tile.field);
    }
}