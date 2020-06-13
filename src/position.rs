use std::ops::Add;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Position {
    pub x: u32,
    pub y: u32,
}

impl Position {
    pub fn new(x: u32, y: u32) -> Self {
        Self { x, y }
    }
}

impl Add for Position {
    type Output = Position;

    fn add(self, other: Position) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

#[cfg(test)]
mod tests {
    use position::Position;

    #[test]
    fn position_new() {
        let position = Position::new(1, 2);
        assert_eq!(1, position.x);
        assert_eq!(2, position.y);
    }

    #[test]
    fn position_ass() {
        let position_a = Position::new(2, 1);
        let position_b = Position::new(1, 3);
        assert_eq!(Position::new(3, 4), position_a + position_b);
    }
}