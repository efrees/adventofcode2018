#[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Copy, Hash, Debug)]
pub struct Point {
    pub x: i64,
    pub y: i64,
}

impl Point {
    pub fn new(x: i64, y: i64) -> Point {
        Point { x: x, y: y }
    }
}
