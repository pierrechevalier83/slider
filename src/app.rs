extern crate piston_window;

use piston_window::{Key, PistonWindow};
use slider::Slider;
use skin::Render;

pub struct App {
    slider: Slider,
}

impl App {
    pub fn render(&mut self, w: &PistonWindow) {
        w.draw_2d(|c, g| {
            Render::render_all(&self.slider, &c, g);
        });
    }

    pub fn handle_key_input(&mut self, key: Key) {
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

    pub fn new() -> App {
        App { slider: Slider::new() }
    }
}
