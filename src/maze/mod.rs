use ::tile;

pub struct Maze<'a> {
    pub tiles: Vec<tile::Tile<'a>>
}

impl<'a> Maze<'a> {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Maze { tiles: Vec::new() }
    }
}

#[cfg(test)]
mod tests {
    use super::Maze;

    #[test]
    fn maze_new() {
        let maze = Maze::new();
        assert_eq!(0, maze.tiles.len());
    }
}