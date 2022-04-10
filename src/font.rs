

use std::collections::HashMap;
use std::rc::Rc;

use crate::texture::GMTexture;
use crate::context::GMContext;

pub trait GMFontT {
    fn draw(&self, c: char, x: f32, y: f32, context: &mut GMContext);
    fn get_char_dimensions(&self, c: char) -> (f32, f32);
    fn rc_clone(&self) -> Rc<dyn GMFontT>;
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
    fn draw(&self, c: char, x: f32, y: f32, context: &mut GMContext) {
        let index = self.mapping.get(&c).unwrap();
        self.texture.draw(x, y, *index, context);
    }

    fn get_char_dimensions(&self, _c: char) -> (f32, f32) {
        self.texture.get_unit_dimension()
    }

    fn rc_clone(&self) -> Rc<dyn GMFontT> {
        let result = GMBitmapFont::new(
            self.texture.clone(),
            self.mapping.clone()
        );

        Rc::new(result)
    }
}
