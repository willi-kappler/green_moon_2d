
use std::fmt::{self, Debug, Formatter};
use std::rc::Rc;

use sdl2::render::Texture;

#[derive(Clone, Debug)]
pub struct GMTextureConfigOptional {
    pub z_index: i32,
    pub angle: f32,
    pub flip_x: bool,
    pub flip_y: bool,
}

impl Default for GMTextureConfigOptional {
    fn default() -> Self {
        Self {
            z_index: 0,
            angle: 0.0,
            flip_x: false,
            flip_y: false,
        }
    }
}


#[derive(Clone, Debug)]
pub struct GMTextureConfig {
    pub(crate) texture: Rc<GMTexture>,
    pub(crate) x: f32,
    pub(crate) y: f32,
    pub(crate) index: u32,
    pub(crate) z_index: i32,
    pub(crate) angle: f32,
    pub(crate) flip_x: bool,
    pub(crate) flip_y: bool,
}

impl GMTextureConfig {
    pub fn new(texture: Rc<GMTexture>, x: f32, y: f32, index: u32) -> Self {
        let optional = GMTextureConfigOptional::default();

        Self::new_opt(texture, x, y, index, optional)
    }

    pub fn new_opt(texture: Rc<GMTexture>, x: f32, y: f32, index: u32, optional: GMTextureConfigOptional) -> Self {
        Self {
            texture,
            x,
            y,
            index,
            z_index: optional.z_index,
            angle: optional.angle,
            flip_x: optional.flip_x,
            flip_y: optional.flip_y,
        }
    }
}

pub struct GMTexture {
    pub(crate) cols: u32,
    pub(crate) unit_width: u32,
    pub(crate) unit_height: u32,
    pub(crate) texture: Texture,
}

impl Debug for GMTexture {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "GMTexture, cols: {}, unit_width: {}, uni_height: {}", self.cols, self.unit_width, self.unit_height)
    }
}

impl GMTexture {
    pub fn new(cols: u32, unit_width: u32, unit_height: u32, texture: Texture) -> Self {
        Self {
            cols,
            unit_width,
            unit_height,
            texture,
        }
    }

    /*
    pub fn draw(&self, dx: f32, dy: f32, index: u32, context: &mut GMContext) {
        self.draw_ex(dx, dy, index, 0.0, false, false, context)
    }

    pub fn draw_ex(&self, dx: f32, dy: f32, index: u32, angle: f32, flip_x: bool, flip_y: bool, context: &mut GMContext) {
        let yi = index / self.cols;
        let xi = index - (yi * self.cols);

        let sx = (xi * self.unit_width) as i32;
        let sy = (yi * self.unit_height) as i32;

        let src_rect = Rect::new(sx, sy, self.unit_width, self.unit_height);
        let dst_rect = Rect::new(dx as i32, dy as i32, self.unit_width, self.unit_height);

        context.draw_ex(&self.texture, src_rect, dst_rect, angle as f64, flip_x, flip_y);
    }
*/

    pub fn get_unit_dimension(&self) -> (f32, f32) {
        (self.unit_width as f32, self.unit_height as f32)
    }

    pub fn get_cols(&self) -> u32 {
        self.cols
    }

    pub fn get_unit_width(&self) -> u32 {
        self.unit_width
    }

    pub fn get_unit_height(&self) -> u32 {
        self.unit_height
    }
}
