#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Color {
    Black,
    Cyan,
    Yellow,
    Purple,
    Green,
    Red,
    Blue,
    Orange,
}

#[derive(PartialEq, Copy, Clone)]
pub enum Direction {
    Left,
    Right
}
