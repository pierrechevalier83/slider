use grid;
use settings::*;

pub const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

pub fn get_shape_color(cell: i32) -> [f32; 4] {
    if cell == grid::EMPTY_CELL {
        BLACK
    } else {
        let num_tiles = N_COLS * N_ROWS - 1;
        let percent: f32 = (cell as f32 + 1.0) / (num_tiles as f32 + 1.0);
        let r = percent * RED_BASE;
        let g = percent * GREEN_BASE;
        let b = percent * BLUE_BASE;
        [r, g, b, 1.0]
    }
}

// TODO: instead of mapping index to color, implement mapping
// index to section of image
