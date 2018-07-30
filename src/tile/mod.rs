use position::Position;
use direction::Direction;

/// A tile defines a walkable part of the maze.
/// # Examle
/// ```
/// let tile = maze_tiles_rust::tile::Tile::new();
/// ```
#[derive(PartialEq, Debug)]
pub struct Tile<'a> {
    pub position: Position,
    pub walkable: [bool; 9],
    pub neighbours: Vec<&'a Tile<'a>>,
}

impl<'a> Tile<'a> {
    /// Returns a new tile at position (row: 0, column: 0) without any neighbours
    /// ```
    /// use maze_tiles_rust::tile::Tile;
    ///
    /// let tile = Tile::new();
    /// ```
    pub fn new() -> Self {
        let position = Position::new();
        Tile { position, walkable: [false; 9], neighbours: Vec::new() }
    }

    /// Returns a new tile at the given row and column without any neighbours
    /// ```
    /// use maze_tiles_rust::tile::Tile;
    ///
    /// let tile = Tile::with_position(1, 2);
    /// ```
    pub fn with_position(column: u32, row: u32) -> Self {
        let mut tile = Tile::new();
        tile.position.column = column;
        tile.position.row = row;
        tile
    }

    /// Returns a tile at the given row, column and walkable without any neighbours
    /// ```
    /// use maze_tiles_rust::tile::Tile;
    ///
    /// let tile = Tile::with_position_and_walkable(1, 2, [false, true, false, true, true, true, false, true, false]);
    /// ```
    pub fn with_position_and_walkable(column: u32, row: u32, walkable: [bool; 9]) -> Self {
        let mut tile = Tile::with_position(column, row);
        tile.walkable = walkable;
        tile
    }

    /// Adds a tile if the given tile is a neighbour
    pub fn add_neighbour(&mut self, neighbour: &'a Tile) {
        let direction = self.neighbour_at(&neighbour);
        match direction {
            Some(_) => self.neighbours.push(&neighbour),
            None => (),
        }
    }

    /// Checks if a given tile is a neighbour and returns an Option with the direction
    pub fn neighbour_at(&self, neighbour: &'a Tile) -> Option<Direction> {
        let column_diff = neighbour.position.column as i32 - self.position.column as i32;
        let row_diff = neighbour.position.row as i32 - self.position.row as i32;

        match (column_diff, row_diff) {
            (0, -1) => Some(Direction::Top),
            (-1, 0) => Some(Direction::Left),
            (0, 1) => Some(Direction::Bottom),
            (1, 0) => Some(Direction::Right),
            (_, _) => None,
        }
    }

    /// Checks if two tiles have a walkable connection
    pub fn has_walkable_neighbour(&self, neighbour: &'a Tile) -> bool {
        let direction = self.neighbour_at(&neighbour);
        match direction {
            Some(direction) => { self.has_walkable_neighbour_at_direction(&neighbour, direction) }
            None => { false }
        }
    }

    /// Checks if two tiles have a walkable connection at a given direction
    pub fn has_walkable_neighbour_at_direction(&self, n: &'a Tile, direction: Direction) -> bool {
        match direction {
            Direction::Top => {
                self.walkable[0] == true && n.walkable[6] == true ||
                    self.walkable[1] == true && n.walkable[7] == true ||
                    self.walkable[2] == true && n.walkable[8] == true
            }
            Direction::Right => {
                self.walkable[2] == true && n.walkable[0] == true ||
                    self.walkable[5] == true && n.walkable[3] == true ||
                    self.walkable[8] == true && n.walkable[6] == true
            }
            Direction::Bottom => {
                self.walkable[6] == true && n.walkable[0] == true ||
                    self.walkable[7] == true && n.walkable[1] == true ||
                    self.walkable[8] == true && n.walkable[2] == true
            }
            Direction::Left => {
                self.walkable[0] == true && n.walkable[2] == true ||
                    self.walkable[3] == true && n.walkable[5] == true ||
                    self.walkable[6] == true && n.walkable[8] == true
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use tile::Tile;

    #[test]
    fn tile_new() {
        let tile = Tile::new();
        assert_eq!(0, tile.position.row);
        assert_eq!(0, tile.position.column);
        assert_eq!(0, tile.neighbours.len());
    }

    #[test]
    fn with_position() {
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
        let tile_a = Tile::with_position(0, 1);
        let mut tile_b = Tile::with_position(1, 1);
        tile_b.add_neighbour(&tile_a);
        assert_eq!(1, tile_b.neighbours.len());
    }

    #[test]
    fn is_neighbour_with_walkable_connection() {
        let tile_a = Tile::with_position_and_walkable(3, 3, [
            false, true, false,
            true, true, true,
            false, true, false
        ]);
        let tile_b = Tile::with_position_and_walkable(2, 3, [
            false, true, false,
            true, true, true,
            false, true, false
        ]);
        assert_eq!(true, tile_b.has_walkable_neighbour(&tile_a));

        let tile_a = Tile::with_position_and_walkable(3, 3, [
            false, true, false,
            false, true, true,
            false, true, false
        ]);
        let tile_b = Tile::with_position_and_walkable(2, 3, [
            false, true, false,
            true, true, false,
            false, true, false
        ]);
        assert_eq!(false, tile_b.has_walkable_neighbour(&tile_a));

        let tile_a = Tile::with_position_and_walkable(3, 3, [
            false, false, false,
            true, true, true,
            false, false, false
        ]);
        let tile_b = Tile::with_position_and_walkable(2, 3, [
            false, true, false,
            true, true, false,
            false, true, false
        ]);
        assert_eq!(false, tile_b.has_walkable_neighbour(&tile_a));
    }
}
