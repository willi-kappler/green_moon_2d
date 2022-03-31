

use std::collections::HashMap;
use std::rc::Rc;

use crate::texture::GMTexture;


pub struct GMBitmapFont {
    texture: Rc<GMTexture>,
    mapping: HashMap<char, u32>,
}

impl GMBitmapFont {
    pub fn draw(&self, c: char, x: i32, y: i32) {
        let index = self.mapping.get(&c).unwrap();
        self.texture.draw(x, y, *index);
    }

    pub fn get_char_dimensions(&self) -> (u32, u32) {
        self.texture.get_unit_dimension()
    }
}
