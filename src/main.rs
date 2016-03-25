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
use piston::input::*;

use slider::*;
use settings::*;
use point::Point;

struct App {
    gl: GlGraphics,
    slider: Slider,
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        // so that we can access inside closure
        let use_slider = &self.slider;
        skin::Render::render_all(&mut self.gl, use_slider, args);
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
                                                 [RESOLUTION_X as u32, skin::RESOLUTION_Y as u32])
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
