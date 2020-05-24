#[derive(Debug)]
pub struct Position {
    pub x: u32,
    pub y: u32,
}

impl Position {
    pub fn new(x: u32, y: u32) -> Self {
        Self {
            x,
            y,
        }
    }
}

#[cfg(test)]
mod tests {
    use tile::position::Position;

    #[test]
    fn position_new() {
        let position = Position::new(1, 2);
        assert_eq!(1, position.x);
        assert_eq!(2, position.y);
    }
}