use settings::{N_COLS, N_ROWS, RED_BASE, GREEN_BASE, BLUE_BASE};

const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
pub const BG_COLOR: [f32; 4] = BLACK;

pub struct Colors {
    colors: Vec<[f32; 4]>,
}

impl Colors {
    pub fn new() -> Colors {
        Colors { colors: Colors::gen_gradient() }
    }

    fn gen_gradient() -> Vec<[f32; 4]> {
        let mut v: Vec<[f32; 4]> = Vec::new();
        let n_tiles = N_COLS * N_ROWS - 1;
        for cell in 0..n_tiles {
            let percent: f32 = (cell as f32 + 1.0) / (n_tiles as f32 + 1.0);
            let r = percent * RED_BASE;
            let g = percent * GREEN_BASE;
            let b = percent * BLUE_BASE;
            v.push([r, g, b, 1.0]);
        }
        v.push(BG_COLOR);
        v
    }

    pub fn get_shape_color(&self, cell: i32) -> [f32; 4] {
        if cell >= 0 && cell < self.colors.len() as i32 {
            self.colors[cell as usize]
        } else {
            BG_COLOR
        }
    }
}
