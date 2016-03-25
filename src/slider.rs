use point::Point;
use grid::Grid;

pub struct Slider {
    grid: Grid,
}
impl Default for Slider {
    fn default() -> Slider {
        Slider::new()
    }
}
impl Slider {
    pub fn new() -> Slider {
        Slider { grid: Grid::new() }
    }

    pub fn get_grid_cell(&self, pos: Point) -> i32 {
        self.grid.get_cell(pos)
    }

    pub fn left(&mut self) {
        let right = self.empty_slot().right();
        self.swap_empty_slot(right);
    }
    pub fn right(&mut self) {
        let left = self.empty_slot().left();
        self.swap_empty_slot(left);
    }
    pub fn up(&mut self) {
        let down = self.empty_slot().down();
        self.swap_empty_slot(down);
    }
    pub fn down(&mut self) {
        let up = self.empty_slot().up();
        self.swap_empty_slot(up);
    }
    pub fn shuffle(&mut self) {
        self.grid.shuffle();
    }

    // Private methods
    fn empty_slot(&self) -> Point {
        self.grid.find_empty_cell()
    }

    fn swap_empty_slot(&mut self, pos: Point) {
        let hole = self.empty_slot();
        self.grid.swap(hole, pos);
    }
}
