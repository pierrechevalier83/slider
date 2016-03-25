extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;
extern crate rand;

pub mod slider;
pub mod point;
pub mod grid;
pub mod skin;
pub mod settings;

use piston::window::{AdvancedWindow, WindowSettings};
use glutin_window::GlutinWindow as Window;
use piston::event_loop::*;
use opengl_graphics::{GlGraphics, OpenGL};
use graphics::Transformed;
use piston::input::*;

use slider::*;
use settings::*;
use point::Point;

struct App {
    gl: GlGraphics,
    slider: Slider,
}

const LIGHT_GRAY: [f32; 4] = [0.3, 0.3, 0.3, 1.0];

const MARGIN: f64 = MARGIN_RATIO * RESOLUTION_X;
const LINING: f64 = LINING_RATIO * RESOLUTION_X;
const CELL_SIZE: f64 = (RESOLUTION_X - 2f64 * (MARGIN + LINING)) / (N_COLS as f64);
const RESOLUTION_Y: f64 = CELL_SIZE * N_ROWS as f64 + 2f64 * (LINING + MARGIN);

struct Render;

impl Render {
    pub fn render_cell(c: &graphics::Context,
                       gl: &mut GlGraphics,
                       transform: [[f64; 3]; 2],
                       color: [f32; 4]) {
        let square = graphics::rectangle::square(0f64, 0f64, CELL_SIZE - 1f64);
        let mut rectangle = graphics::Rectangle::new(color);
        rectangle.shape = graphics::rectangle::Shape::Round(4.0, 60);
        rectangle.draw(square, &c.draw_state, transform, gl);
    }

    pub fn render_game_board(c: &graphics::Context, gl: &mut GlGraphics, slider: &Slider) {
        for col in 0..settings::N_COLS as i32 {
            for row in 0..settings::N_ROWS as i32 {
                let color = skin::get_shape_color(slider.get_grid_cell(Point::new(col, row)));
                let (x, y) = (col as f64 * CELL_SIZE, row as f64 * CELL_SIZE);
                let transform = c.transform.trans(MARGIN, MARGIN).trans(x, y);
                Render::render_cell(&c, gl, transform, color);
            }
        }
    }
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        // so that we can access inside closure
        let use_slider = &self.slider;

        self.gl.draw(args.viewport(), |c, gl| {
            // clear the viewport
            graphics::clear(skin::BLACK, gl);

            // draw a white border around the game board
            let rect_border = graphics::Rectangle::new_border(LIGHT_GRAY, LINING);
            rect_border.draw([MARGIN - LINING,
                              MARGIN - LINING,
                              (CELL_SIZE * settings::N_COLS as f64) + 1.8 * LINING,
                              (CELL_SIZE * settings::N_ROWS as f64) + 1.8 * LINING],
                             &c.draw_state,
                             c.transform,
                             gl);

            Render::render_game_board(&c, gl, use_slider);
        });
    }

    fn handle_key_input(&mut self, key: keyboard::Key) {
        match key {
            Key::Left => {
                self.slider.left();
            }

            Key::Right => {
                self.slider.right();
            }

            Key::Up => {
                self.slider.up();
            }

            Key::Down => {
                self.slider.down();
            }

            Key::Space => {
                self.slider.shuffle();
            }

            _ => {}
        }
    }
}

fn main() {
    start_app();
}

fn start_app() {
    let opengl = OpenGL::V2_1;

    let mut window: Window = WindowSettings::new("Slider",
                                                 [RESOLUTION_X as u32, RESOLUTION_Y as u32])
                                 .exit_on_esc(true)
                                 .opengl(opengl)
                                 .build()
                                 .unwrap();

    let mut app = App {
        gl: GlGraphics::new(opengl),
        slider: Slider::new(),
    };

    let mut events = window.events();
    while let Some(e) = events.next(&mut window) {
        if let Some(Button::Keyboard(key)) = e.press_args() {
            app.handle_key_input(key);
        };

        if let Some(args) = e.render_args() {
            app.render(&args);
        };
    }
}
