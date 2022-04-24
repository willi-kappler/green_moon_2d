

use sdl2::rect::Rect;

use crate::GMDrawContext;

#[derive(Debug)]
pub struct GMTexture {
    name: String,
    //width: u32,
    //height: u32,
    //rows: u32,
    cols: u32,
    unit_width: u32,
    unit_height: u32,
    data: u8, // TODO
}

impl GMTexture {
    pub fn new(name: &str, cols: u32, unit_width: u32, unit_height: u32, data: u8) -> Self {
        Self {
            name: name.to_string(),
            cols,
            unit_width,
            unit_height,
            data,
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

        // context.canvas.copy_ex(self.data, src_rect, dst_rect, angle as f64, None, flip_x, flip_y);
    }

    pub fn get_unit_dimension(&self) -> (f32, f32) {
        (self.unit_width as f32, self.unit_height as f32)
    }

    pub fn get_name(&self) -> &str {
        &self.name
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
