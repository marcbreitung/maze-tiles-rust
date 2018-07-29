/// Defines in which direction two tiles are connected
#[derive(PartialEq, Debug)]
pub enum Direction {
    Top,
    Right,
    Bottom,
    Left,
}

impl Direction {
    pub fn get_directions() -> Vec<Direction> {
        vec![
            Direction::Top,
            Direction::Right,
            Direction::Bottom,
            Direction::Left,
        ]
    }
}
