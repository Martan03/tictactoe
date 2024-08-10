/// Represents cell value
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Cell {
    Cross,
    Circle,
    Empty,
}

impl Cell {
    /// Gets next cell value on play
    pub fn next(self) -> Self {
        match self {
            Cell::Cross => Cell::Circle,
            _ => Cell::Cross,
        }
    }
}
