/// Grid maps 2D Points into a 2D domain (internally, a vector)
extern crate rand;
use rand::Rng;
use point::Point;
use settings::{N_COLS, N_ROWS};

pub struct Grid {
    values: Vec<i32>,
    n_cols: usize,
    rng: rand::ThreadRng,
}

pub const EMPTY_CELL: i32 = -1;

// A vector full of values going from 0..n_tiles, with an empty slot at the end
fn fill_original_grid() -> Vec<i32> {
    let num_tiles = N_COLS * N_ROWS - 1;
    let mut g = (0..num_tiles).collect::<Vec<i32>>();
    g.push(EMPTY_CELL);
    g
}

impl Default for Grid {
    fn default() -> Grid {
        Grid::new()
    }
}

impl Grid {
    pub fn new() -> Grid {
        Grid {
            values: fill_original_grid(),
            n_cols: N_COLS as usize,
            rng: rand::thread_rng(),
        }
    }

    pub fn get_cell(&self, pos: Point) -> i32 {
        assert!(self.fits(pos));
        self.values[self.index(pos) as usize]
    }

    pub fn set_cell(&mut self, pos: Point, val: i32) {
        assert!(self.fits(pos));
        let i = self.index(pos) as usize;
        self.values[i] = val;
    }

    pub fn swap(&mut self, a: Point, b: Point) {
        if self.fits(a) && self.fits(b) {
            let a_i = self.index(a) as usize;
            let b_i = self.index(b) as usize;
            self.values.swap(a_i, b_i);
        }
    }

    pub fn find_empty_cell(&self) -> Point {
        self.find(EMPTY_CELL)
    }

    pub fn shuffle(&mut self) {
        self.rng.shuffle(&mut self.values.as_mut_slice());
    }

    fn find(&self, val: i32) -> Point {
        self.position(self.values.iter().position(|&r| r == val).unwrap() as i32)
    }

    fn n_cols(&self) -> i32 {
        self.n_cols as i32
    }

    fn n_rows(&self) -> i32 {
        self.values.len() as i32 / self.n_cols as i32
    }

    fn index(&self, pos: Point) -> i32 {
        pos.x + self.n_cols() * pos.y
    }

    fn position(&self, index: i32) -> Point {
        Point::new(index % self.n_cols(), index / self.n_cols())
    }

    fn fits(&self, pos: Point) -> bool {
        pos.within_bounds(Point::new(0, 0), Point::new(self.n_cols(), self.n_rows()))
    }
}
