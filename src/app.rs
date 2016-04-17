extern crate piston_window;

use piston_window::{Button, PressEvent, Key, PistonWindow};

use skin::Render;
use slider::Slider;

pub struct App {
    slider: Slider,
    window: PistonWindow,
    skin: Render,
}

impl App {
    pub fn run(&mut self) {
        for e in self.window.clone() {
            self.render(&e);
            if let Some(Button::Keyboard(key)) = e.press_args() {
                self.handle_key_input(key);
                self.render(&e);
            };
        }
    }

    pub fn load_texture(&mut self) {
        self.skin.load_texture(&mut self.window.clone());
    }

    pub fn render(&mut self, w: &PistonWindow) {
        w.draw_2d(|c, g| {
            self.skin.render_all(&c, g, &self.slider);
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
            Key::S => {
                self.slider.solve();
            }
            _ => {}
        }
    }

    pub fn new(w: PistonWindow) -> App {
        App {
            slider: Slider::new(),
            window: w,
            skin: Render::new(),
        }
    }
}
