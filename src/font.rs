

use std::collections::HashMap;
use std::rc::Rc;

use crate::texture::GMTexture;

pub trait GMFontT {
    fn draw(&self, c: char, x: f32, y: f32);
    fn get_char_dimensions(&self, c: char) -> (f32, f32);
}

pub struct GMBitmapFont {
    texture: Rc<GMTexture>,
    mapping: HashMap<char, u32>,
}

impl GMBitmapFont {
    pub fn new(texture: Rc<GMTexture>, mapping: HashMap<char, u32>) -> Self {
        Self {
            texture: texture.clone(),
            mapping,
        }
    }
}

impl GMFontT for GMBitmapFont {
    fn draw(&self, c: char, x: f32, y: f32) {
        let index = self.mapping.get(&c).unwrap();
        self.texture.draw(x, y, *index);
    }

    fn get_char_dimensions(&self, _c: char) -> (f32, f32) {
        self.texture.get_unit_dimension()
    }
}
