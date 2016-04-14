extern crate piston_window;
extern crate rand;

pub mod slider;
pub mod point;
pub mod grid;
pub mod skin;
pub mod settings;

use piston_window::{Button, Key, PistonWindow, PressEvent, WindowSettings};
use slider::*;
use settings::*;

struct App {
    slider: Slider,
}

impl App {
    fn render(&mut self, w: &PistonWindow) {
        w.draw_2d(|c, g| {
            skin::Render::render_all(&self.slider, &c, g);
        });
    }

    fn handle_key_input(&mut self, key: Key) {
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

    fn new() -> App {
        App { slider: Slider::new() }
    }
}

fn main() {
    start_app();
}

fn start_app() {
    let window: PistonWindow = WindowSettings::new("Slider",
                                                   [RESOLUTION_X as u32,
                                                    skin::RESOLUTION_Y as u32])
                                   .exit_on_esc(true)
                                   .build()
                                   .unwrap();
    let mut app = App::new();

    for e in window {
        app.render(&e);
        if let Some(Button::Keyboard(key)) = e.press_args() {
            app.handle_key_input(key);
            app.render(&e);
        };
    }
}
