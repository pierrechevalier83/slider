// magic numbers (need to devise some kind of input mechanism for configuration)

// image (optional)
pub const IMAGE_PATH: &'static str = "butterfly.jpg";

// colour (if can't load image, will use a gradient based on this color instead)
pub const RED_BASE: f32 = 1.0;
pub const GREEN_BASE: f32 = 0.0;
pub const BLUE_BASE: f32 = 0.2;

// dimensions
pub const MARGIN_RATIO: f64 = 0.05f64;
pub const LINING_RATIO: f64 = 0.005f64;
pub const RESOLUTION_X: f64 = 1200f64;

// size of grid
pub const N_COLS: i32 = 4;
pub const N_ROWS: i32 = 3;
