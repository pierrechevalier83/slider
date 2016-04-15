extern crate find_folder;
extern crate image;
extern crate piston_window;

use piston_window::{PistonWindow, G2dTexture, Texture, TextureSettings};
use skin::{CELL_SIZE};
use grid::EMPTY_CELL;
use settings::{N_COLS, N_ROWS, IMAGE_PATH};

pub struct Textures {
    cell_textures: Vec<Option<G2dTexture<'static>>>,
}

impl Textures {
    pub fn new() -> Textures {
        Textures { cell_textures: Vec::new() }
    }

    pub fn load(&mut self, window: &mut PistonWindow) {
        let assets = find_folder::Search::ParentsThenKids(3, 3).for_folder("assets").unwrap();
        let path = assets.join(IMAGE_PATH);
        let factory = &mut *window.factory.borrow_mut();
        let maybe_img = image::open(path);
        if maybe_img.is_ok() {
            let mut img = image::imageops::resize(maybe_img.as_ref().unwrap(),
                                                  CELL_SIZE as u32 * N_COLS as u32,
                                                  CELL_SIZE as u32 * N_ROWS as u32,
                                                  image::FilterType::Nearest);
            for row in 0u32..N_ROWS as u32 {
                for col in 0u32..N_COLS as u32 {
                    let cell_size = CELL_SIZE as u32;
                    let subimg = image::imageops::crop(&mut img,
                                                       col * cell_size as u32,
                                                       row * cell_size as u32,
                                                       cell_size as u32,
                                                       cell_size as u32);

                    self.cell_textures.push(match Texture::from_image(factory,
                                                                      &subimg.to_image(),
                                                                      &TextureSettings::new()) {
                        Ok(t) => Some(t),
                        Err(_) => None,
                    });
                }
            }
        } else {
            for _ in 0..N_COLS * N_ROWS {
                self.cell_textures.push(None);
            }
        }
    }

    pub fn get_shape_texture(&self, cell: i32) -> &Option<G2dTexture<'static>> {
        if cell == EMPTY_CELL {
            &self.cell_textures[(N_COLS * N_ROWS - 1) as usize]
        } else {
            &self.cell_textures[cell as usize]
        }
    }
}
