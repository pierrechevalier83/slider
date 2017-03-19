extern crate find_folder;
extern crate piston_window;

use  piston_window::*;

pub struct Font{
    font: Option<Glyphs>,
}

impl Font {
    pub fn new() -> Font {
        Font { font: None }
    }

    pub fn load(&mut self, window: &mut PistonWindow) {
        let assets = find_folder::Search::ParentsThenKids(3, 3)
            .for_folder("assets").unwrap();
        let ref font = assets.join("OpenSans-Regular.ttf");
        let ref factory = window.factory;
        self.font = Some(Glyphs::new(font, factory.clone()).unwrap());
    }

    pub fn get_font(&mut self) -> &mut Glyphs {
        self.font.as_mut().unwrap()
    }
}
