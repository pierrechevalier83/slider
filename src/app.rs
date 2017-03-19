extern crate piston_window;
extern crate find_folder;

use piston_window::{Button, PressEvent, Key, PistonWindow};

use skin::Render;
use slider::Slider;

pub struct App {
    slider: Slider,
    skin: Render,
    player_moved_last: bool,
}

impl App {
    pub fn run(&mut self, window: &mut PistonWindow) {
        while let Some(e) = window.next() {
            self.render(&e, window);
            if let Some(Button::Keyboard(key)) = e.press_args() {
                self.handle_key_input(key);
                self.render(&e, window);
            };
        }
    }

    pub fn load_texture(&mut self, window: &mut PistonWindow) {
        self.skin.load_texture(window);
    }

    pub fn load_font(&mut self, window: &mut PistonWindow) {
        self.skin.load_font(window);
    }

    pub fn render<E>(&mut self, event: &E, window: &mut PistonWindow) where E : piston_window::GenericEvent {
            window.draw_2d(event, |c, g| {
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

    pub fn new() -> App {
        App {
            slider: Slider::new(),
            skin: Render::new(),
            player_moved_last: false,
        }
    }
}
