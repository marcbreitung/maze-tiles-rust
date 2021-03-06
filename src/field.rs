/// A ``Field`` defines the type of a ``Tile``
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Field {
    None = 0,
    Ground = 1,
    Path = 2,
}
