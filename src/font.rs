

use std::collections::HashMap;
use std::rc::Rc;

use crate::texture::GMTexture;
use crate::context::GMDrawContext;

pub trait GMFontT {
    fn draw(&self, c: char, x: f32, y: f32, context: &mut GMDrawContext);

    fn get_char_dimensions(&self, c: char) -> (f32, f32);

    fn rc_clone(&self) -> Rc<dyn GMFontT>;
}

#[derive(Clone)]
pub struct GMBitmapFont {
    texture: Rc<GMTexture>,
    mapping: HashMap<char, u32>,
}

impl GMBitmapFont {
    pub fn new(texture: Rc<GMTexture>, char_mapping: &str) -> Self {
        let mut mapping = HashMap::new();

        for (i, c) in char_mapping.chars().enumerate() {
            mapping.insert(c, i as u32);
        }

        Self {
            texture,
            mapping,
        }
    }
}

impl GMFontT for GMBitmapFont {
    fn draw(&self, c: char, x: f32, y: f32, context: &mut GMDrawContext) {
        let index = self.mapping.get(&c).unwrap();
        self.texture.draw(x, y, *index, context);
    }

    fn get_char_dimensions(&self, _c: char) -> (f32, f32) {
        self.texture.get_unit_dimension()
    }

    fn rc_clone(&self) -> Rc<dyn GMFontT> {
        let result = self.clone();

        Rc::new(result)
    }
}
