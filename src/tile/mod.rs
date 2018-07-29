/// A tile defines the walkable parts of the maze
#[derive(PartialEq, Debug)]
pub struct Tile<'a> {
    pub position: Position,
    pub walkable: [bool; 9],
    pub neighbour_top: Option<&'a Tile<'a>>,
    pub neighbour_right: Option<&'a Tile<'a>>,
    pub neighbour_bottom: Option<&'a Tile<'a>>,
    pub neighbour_left: Option<&'a Tile<'a>>,
}

impl<'a> Tile<'a> {
    /// Returns a tile at position (row: 0, column: 0) without any neighbours
    #[allow(dead_code)]
    pub fn new() -> Self {
        let position = Position { column: 0, row: 0 };
        Tile { position, walkable: [false; 9], neighbour_top: None, neighbour_right: None, neighbour_bottom: None, neighbour_left: None }
    }

    /// Returns a tile at the given row and column without any neighbours
    #[allow(dead_code)]
    pub fn with_position(column: u32, row: u32) -> Self {
        let mut tile = Tile::new();
        tile.position.column = column;
        tile.position.row = row;
        tile
    }

    /// Returns a tile at the given row, column and walkable without any neighbours
    #[allow(dead_code)]
    pub fn with_position_and_walkable(column: u32, row: u32, walkable: [bool; 9]) -> Self {
        let mut tile = Tile::with_position(column, row);
        tile.walkable = walkable;
        tile
    }

    /// Adds a tile if the given tile is a neighbour
    #[allow(dead_code)]
    pub fn add_neighbour(&mut self, n: &'a Tile) {
        let direction = self.neighbour_at(&n);
        match direction {
            Some(d) => self.add_neighbour_at(&n, d),
            None => (),
        }
    }

    /// Adds a tile at the given position
    #[allow(dead_code)]
    pub fn add_neighbour_at(&mut self, n: &'a Tile, d: Direction) {
        match d {
            Direction::Top => { self.neighbour_top = Some(&n) }
            Direction::Right => { self.neighbour_right = Some(&n) }
            Direction::Bottom => { self.neighbour_bottom = Some(&n) }
            Direction::Left => { self.neighbour_left = Some(&n) }
        }
    }

    /// Checks if a given tile is a neighbour and returns an Option with the direction
    #[allow(dead_code)]
    pub fn neighbour_at(&self, n: &'a Tile) -> Option<Direction> {
        let column_diff = n.position.column as i32 - self.position.column as i32;
        let row_diff = n.position.row as i32 - self.position.row as i32;

        match (column_diff, row_diff) {
            (0, -1) => Some(Direction::Top),
            (-1, 0) => Some(Direction::Left),
            (0, 1) => Some(Direction::Bottom),
            (1, 0) => Some(Direction::Right),
            (_, _) => None,
        }
    }
}

/// Defines in which direction two tiles are connected
#[derive(PartialEq, Debug)]
pub enum Direction {
    Top,
    Right,
    Bottom,
    Left,
}

/// Defines the tiles position inside the maze
#[derive(PartialEq, Debug)]
pub struct Position {
    pub column: u32,
    pub row: u32,
}

#[cfg(test)]
mod tests {
    use super::Tile;
    use super::Direction;

    #[test]
    fn tile_new() {
        let tile = Tile::new();
        assert_eq!(0, tile.position.row);
        assert_eq!(0, tile.position.column);
        assert_eq!(Option::None, tile.neighbour_top);
        assert_eq!(Option::None, tile.neighbour_right);
        assert_eq!(Option::None, tile.neighbour_bottom);
        assert_eq!(Option::None, tile.neighbour_left);
    }

    #[test]
    fn tile_add_neighbour_at() {
        let tile = Tile::with_position(1, 2);
        assert_eq!(1, tile.position.column);
        assert_eq!(2, tile.position.row);
    }

    #[test]
    fn with_position_and_walkable() {
        let tile = Tile::with_position_and_walkable(1, 2, [false, true, false, true, true, true, false, true, false]);
        assert_eq!(1, tile.position.column);
        assert_eq!(2, tile.position.row);
        assert_eq!([false, true, false, true, true, true, false, true, false], tile.walkable);
    }

    #[test]
    fn tile_add_neighbour() {
        let tile_a = Tile::new();
        let mut tile_b = Tile::new();
        tile_b.add_neighbour_at(&tile_a, Direction::Top);
        tile_b.add_neighbour_at(&tile_a, Direction::Right);
        tile_b.add_neighbour_at(&tile_a, Direction::Bottom);
        tile_b.add_neighbour_at(&tile_a, Direction::Left);

        assert_eq!(Option::Some(&tile_a), tile_b.neighbour_top);
        assert_eq!(Option::Some(&tile_a), tile_b.neighbour_right);
        assert_eq!(Option::Some(&tile_a), tile_b.neighbour_bottom);
        assert_eq!(Option::Some(&tile_a), tile_b.neighbour_left);
    }

    #[test]
    fn is_neighbour_at_same_position() {
        let tile_a = Tile::new();
        let tile_b = Tile::new();
        assert_eq!(Option::None, tile_b.neighbour_at(&tile_a));
    }

    #[test]
    fn is_neighbour_at_top() {
        let tile_a = Tile::with_position(1, 1);
        let tile_b = Tile::with_position(1, 2);
        assert_eq!(Some(Direction::Top), tile_b.neighbour_at(&tile_a));
    }

    #[test]
    fn is_neighbour_at_right() {
        let tile_a = Tile::with_position(2, 1);
        let tile_b = Tile::with_position(1, 1);
        assert_eq!(Some(Direction::Right), tile_b.neighbour_at(&tile_a));
    }

    #[test]
    fn is_neighbour_at_bottom() {
        let tile_a = Tile::with_position(1, 2);
        let tile_b = Tile::with_position(1, 1);
        assert_eq!(Some(Direction::Bottom), tile_b.neighbour_at(&tile_a));
    }

    #[test]
    fn is_neighbour_at_left() {
        let tile_a = Tile::with_position(1, 1);
        let tile_b = Tile::with_position(2, 1);
        assert_eq!(Some(Direction::Left), tile_b.neighbour_at(&tile_a));
    }
}
