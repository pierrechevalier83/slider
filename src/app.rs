extern crate piston_window;
extern crate find_folder;

use piston_window::{Button, PressEvent, Key, PistonWindow};

use skin::Render;
use slider::Slider;

pub struct App {
    slider: Slider,
    window: PistonWindow,
    skin: Render,
    player_moved_last: bool,
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

    pub fn load_font(&mut self) {
        self.skin.load_font(&mut self.window.clone());
    }

    pub fn render(&mut self, w: &PistonWindow) {
        w.draw_2d(|c, g| {
            self.skin.render_all(&c, g, &self.slider);
            if self.is_success() {
                self.skin.render_success(&c, g);
            }
        });
    }

    fn is_success(&self) -> bool {
        self.player_moved_last && self.slider.is_sorted()
    }

    pub fn handle_key_input(&mut self, key: Key) {
        match key {
            Key::Left => {
                self.slider.left();
                self.player_moved_last = true;
            }
            Key::Right => {
                self.slider.right();
                self.player_moved_last = true;
            }
            Key::Up => {
                self.slider.up();
                self.player_moved_last = true;
            }
            Key::Down => {
                self.slider.down();
                self.player_moved_last = true;
            }
            Key::Space => {
                self.slider.shuffle();
                self.player_moved_last = false;
            }
            Key::S => {
                self.slider.solve();
                self.player_moved_last = false;
            }
            _ => {}
        }
    }

    pub fn new(w: PistonWindow) -> App {
        App {
            slider: Slider::new(),
            window: w,
            skin: Render::new(),
            player_moved_last: false,
        }
    }
}
