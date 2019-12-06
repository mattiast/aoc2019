#[derive(Debug)]
pub enum Direction {
    R,
    D,
    L,
    U,
}

pub type Wire = (Direction, i32);

pub type Point = (f64, f64);

#[derive(Clone, Copy)]
pub enum Segment {
    Horizontal(f64, f64, f64),
    Vertical(f64, f64, f64),
}
