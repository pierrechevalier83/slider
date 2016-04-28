extern crate piston_window;

use piston_window::{G2d, G2dTexture, Context, Rectangle, rectangle, clear, PistonWindow};
use piston_window::Transformed;
use point::Point;
use grid::EMPTY_CELL;
use settings::{MARGIN_RATIO, RESOLUTION_X, LINING_RATIO, N_COLS, N_ROWS};
use colors;
use colors::Colors;
use textures::Textures;
use font::Font;
use slider::Slider;


const LIGHT_GRAY: [f32; 4] = [0.3, 0.3, 0.3, 1.0];

const MARGIN: f64 = MARGIN_RATIO * RESOLUTION_X;
const LINING: f64 = LINING_RATIO * RESOLUTION_X;
pub const CELL_SIZE: f64 = (RESOLUTION_X - 2f64 * (MARGIN + LINING)) / (N_COLS as f64);
pub const RESOLUTION_Y: f64 = CELL_SIZE * N_ROWS as f64 + 2f64 * (LINING + MARGIN);

pub struct Render {
    colors: Colors,
    textures: Textures,
    font: Font,
}

impl Render {
    pub fn new() -> Render {
        Render {
            colors: Colors::new(),
            textures: Textures::new(),
            font: Font::new(),
        }
    }

    pub fn load_texture(&mut self, window: &mut PistonWindow) {
        self.textures.load(window);
    }

    pub fn load_font(&mut self, window: &mut PistonWindow) {
        self.font.load(window);
    }

    fn render_cell_as_color(c: &Context, g: &mut G2d, transform: [[f64; 3]; 2], color: [f32; 4]) {
        let mut rectangle = Rectangle::new(color);
        let square = rectangle::square(0f64, 0f64, CELL_SIZE - 1f64);
        rectangle.shape = rectangle::Shape::Round(4.0, 60);
        rectangle.draw(square, &c.draw_state, transform, g);
    }

    fn render_cell_as_texture(g: &mut G2d, transform: [[f64; 3]; 2], texture: &G2dTexture) {
        piston_window::image(texture, transform, g);
    }

    fn render_cell(&self, c: &Context, g: &mut G2d, cell: i32, x: f64, y: f64) {
        let transform = c.transform.trans(MARGIN, MARGIN).trans(x, y);
        let img = self.textures.get_shape_texture(cell);
        if img.is_some() {
            if cell != EMPTY_CELL {
                Render::render_cell_as_texture(g, transform, img.as_ref().unwrap());
            }
        } else {
            let color = self.colors.get_shape_color(cell);
            Render::render_cell_as_color(&c, g, transform, color);
        }
    }


    fn render_game_board(&self, c: &Context, g: &mut G2d, slider: &Slider) {
        for col in 0..N_COLS as i32 {
            for row in 0..N_ROWS as i32 {
                Render::render_cell(self,
                                    &c,
                                    g,
                                    slider.get_grid_cell(Point::new(col, row)),
                                    col as f64 * CELL_SIZE,
                                    row as f64 * CELL_SIZE);
            }
        }
    }

    fn render_border(c: &Context, g: &mut G2d) {
        // draw a white border around the game board
        let rect_border = Rectangle::new_border(LIGHT_GRAY, LINING);
        rect_border.draw([MARGIN - LINING,
                          MARGIN - LINING,
                          (CELL_SIZE * N_COLS as f64) + 1.8 * LINING,
                          (CELL_SIZE * N_ROWS as f64) + 1.8 * LINING],
                         &c.draw_state,
                         c.transform,
                         g);
    }

    pub fn render_all(&self, c: &Context, g: &mut G2d, slider: &Slider) {
        clear(colors::BG_COLOR, g);
        Render::render_border(&c, g);
        Render::render_game_board(self, &c, g, slider);
    }

    pub fn render_success(&mut self, c: &Context, g: &mut G2d) {
        use piston_window::*;
        use settings::RESOLUTION_X;
        use skin::RESOLUTION_Y;
        let font = self.font.get_font();
                // TODO: font = 100, from settings
        let congratulations = "Well done!";
        let transform = c.transform.trans((RESOLUTION_X)/ 2f64 - (100f64 * congratulations.len() as f64 / 4f64), RESOLUTION_Y / 2f64);
        text::Text::new_color([0.0, 1.0, 0.0, 1.0], 100).draw(
        congratulations,
        font,
        &c.draw_state,
        transform, g
       );
    }
}
