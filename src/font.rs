

use std::collections::HashMap;
use std::rc::Rc;

use crate::texture::GMTexture;


pub struct GMBitmapFont {
    texture: Rc<GMTexture>,
    mapping: HashMap<char, u32>,
}
