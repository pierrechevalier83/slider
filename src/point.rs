/// Point represents a 2d point. It can give us its neighbours
/// and let us now if it is contained within given bounds.
#[derive(Copy, Clone)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn new(x_: i32, y_: i32) -> Point {
        Point { x: x_, y: y_ }
    }
    pub fn right(&self) -> Point {
        Point::new(self.x + 1, self.y)
    }
    pub fn left(&self) -> Point {
        Point::new(self.x - 1, self.y)
    }
    pub fn up(&self) -> Point {
        Point::new(self.x, self.y - 1)
    }
    pub fn down(&self) -> Point {
        Point::new(self.x, self.y + 1)
    }
    pub fn within_bounds(&self, low: Point, high: Point) -> bool {
        self.x >= low.x && self.x < high.x && self.y >= low.y && self.y < high.y
    }
}
