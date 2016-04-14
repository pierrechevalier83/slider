extern crate piston_window;

use piston_window::*;
use point::Point;
use grid::EMPTY_CELL;
use settings;
use settings::*;
use slider::Slider;

pub const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

pub fn get_shape_color(cell: i32) -> [f32; 4] {
    if cell == EMPTY_CELL {
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

const LIGHT_GRAY: [f32; 4] = [0.3, 0.3, 0.3, 1.0];

const MARGIN: f64 = MARGIN_RATIO * RESOLUTION_X;
const LINING: f64 = LINING_RATIO * RESOLUTION_X;
const CELL_SIZE: f64 = (RESOLUTION_X - 2f64 * (MARGIN + LINING)) / (N_COLS as f64);
pub const RESOLUTION_Y: f64 = CELL_SIZE * N_ROWS as f64 + 2f64 * (LINING + MARGIN);


pub struct Render;

impl Render {
    pub fn render_cell<G>(c: &Context, g: &mut G, transform: [[f64; 3]; 2], color: [f32; 4])
        where G: Graphics
    {
        let mut rectangle = Rectangle::new(color);
        let square = rectangle::square(0f64, 0f64, CELL_SIZE - 1f64);
        rectangle.shape = rectangle::Shape::Round(4.0, 60);
        rectangle.draw(square, &c.draw_state, transform, g);
    }

    pub fn render_game_board<G>(c: &Context, g: &mut G, slider: &Slider)
        where G: Graphics
    {
        for col in 0..N_COLS as i32 {
            for row in 0..N_ROWS as i32 {
                let color = get_shape_color(slider.get_grid_cell(Point::new(col, row)));
                let (x, y) = (col as f64 * CELL_SIZE, row as f64 * CELL_SIZE);
                let transform = c.transform.trans(MARGIN, MARGIN).trans(x, y);
                Render::render_cell(&c, g, transform, color);
            }
        }
    }

    pub fn render_border<G>(c: &Context, g: &mut G)
        where G: Graphics
    {
        // draw a white border around the game board
        let rect_border = Rectangle::new_border(LIGHT_GRAY, LINING);
        rect_border.draw([MARGIN - LINING,
                          MARGIN - LINING,
                          (CELL_SIZE * settings::N_COLS as f64) + 1.8 * LINING,
                          (CELL_SIZE * settings::N_ROWS as f64) + 1.8 * LINING],
                         &c.draw_state,
                         c.transform,
                         g);
    }

    pub fn render_all<G>(slider: &Slider, c: &Context, g: &mut G)
        where G: Graphics
    {
        clear(BLACK, g);
        Render::render_border(&c, g);
        Render::render_game_board(&c, g, slider);
    }
}
