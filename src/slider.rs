extern crate rand;
use rand::distributions::Range;
use rand::distributions::IndependentSample;
use point::Point;
use grid::Grid;

pub struct Slider {
    grid: Grid,
    rng: rand::ThreadRng,
}
impl Default for Slider {
    fn default() -> Slider {
        Slider::new()
    }
}
impl Slider {
    pub fn new() -> Slider {
        Slider {
            grid: Grid::new(),
            rng: rand::thread_rng(),
        }
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
        for _ in 0..1000 * self.grid.n_rows() * self.grid.n_cols() {
            let between = Range::new(0, 4);
            let dir = between.ind_sample(&mut self.rng);
            match dir {
                0 => {
                    self.left();
                }
                1 => {
                    self.right();
                }
                2 => {
                    self.up();
                }
                3 => {
                    self.down();
                }
                _ => {
                    panic!("There is a bug with the rng!");
                }
            }
        }
    }
    pub fn solve(&mut self) {
        self.grid.sort();
    }

    pub fn is_sorted(& self) -> bool {
        self.grid.is_sorted()
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
