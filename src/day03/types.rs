#[derive(Debug)]
pub enum Direction {
    R,
    D,
    L,
    U,
}

pub type Wires = Vec<(Direction, i32)>;

#[derive(Clone, Copy)]
pub enum Segment {
    Horizontal(i32, i32, i32),
    Vertical(i32, i32, i32),
}