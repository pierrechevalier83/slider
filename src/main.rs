extern crate piston_window;
extern crate rand;

pub mod app;
pub mod grid;
pub mod colors;
pub mod font;
pub mod point;
pub mod settings;
pub mod skin;
pub mod slider;
pub mod textures;

use piston_window::{PistonWindow, WindowSettings};
use settings::RESOLUTION_X;
use app::App;

fn main() {
    let mut window: PistonWindow = WindowSettings::new("Slider",
                                                   [RESOLUTION_X as u32,
                                                    skin::RESOLUTION_Y as u32])
                                   .exit_on_esc(true)
                                   .build()
                                   .unwrap();

    let mut app = App::new();
    app.load_texture(&mut window);
    app.load_font(&mut window);
    app.run(&mut window);
}
