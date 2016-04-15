extern crate piston_window;
extern crate rand;

pub mod app;
pub mod point;
pub mod grid;
pub mod skin;
pub mod slider;
pub mod settings;

use piston_window::{Button, PistonWindow, PressEvent, WindowSettings};
use settings::*;
use app::App;

fn main() {
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
