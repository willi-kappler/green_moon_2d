

use sdl2::rect::Rect;
use sdl2::render::Texture;

use crate::GMDrawContext;

pub struct GMTexture {
    cols: u32,
    unit_width: u32,
    unit_height: u32,
    texture: Texture,
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

    pub fn draw(&self, dx: f32, dy: f32, index: u32, context: &mut GMDrawContext) {
        self.draw_ex(dx, dy, index, 0.0, false, false, context)
    }

    pub fn draw_ex(&self, dx: f32, dy: f32, index: u32, angle: f32, flip_x: bool, flip_y: bool, context: &mut GMDrawContext) {
        let yi = index / self.cols;
        let xi = index - (yi * self.cols);

        let sx = (xi * self.unit_width) as i32;
        let sy = (yi * self.unit_height) as i32;

        let src_rect = Rect::new(sx, sy, self.unit_width, self.unit_height);
        let dst_rect = Rect::new(dx as i32, dy as i32, self.unit_width, self.unit_height);

        context.draw_ex(&self.texture, src_rect, dst_rect, angle as f64, flip_x, flip_y);
    }

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
