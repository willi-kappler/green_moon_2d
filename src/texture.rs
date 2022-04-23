

use sdl2::rect::Rect;

use crate::GMDrawContext;

#[derive(Debug)]
pub struct GMTexture {
    //width: u32,
    //height: u32,
    //rows: u32,
    pub cols: u32,
    pub unit_width: u32,
    pub unit_height: u32,
    //data: u8, // TODO
}

impl Default for GMTexture {
    fn default() -> Self {
        Self { cols: 0, unit_width: 0, unit_height: 0 }
    }
}

impl GMTexture {
    pub fn draw(&self, dx: f32, dy: f32, index: u32, context: &mut GMDrawContext) {
        self.draw_ex(dx, dy, index, 0.0, false, false, context)
    }

    pub fn draw_ex(&self, dx: f32, dy: f32, index: u32, _angle: f32, _flip_x: bool, _flip_y: bool, _context: &mut GMDrawContext) {
        let yi = index / self.cols;
        let xi = index - (yi * self.cols);

        let sx = (xi * self.unit_width) as i32;
        let sy = (yi * self.unit_height) as i32;

        let _src_rect = Rect::new(sx, sy, self.unit_width, self.unit_height);
        let _dst_rect = Rect::new(dx as i32, dy as i32, self.unit_width, self.unit_height);

        // context.canvas.copy_ex(self.texture, src_rect, dst_rect, angle as f64, None, flip_x, flip_y);
    }

    pub fn get_unit_dimension(&self) -> (f32, f32) {
        (self.unit_width as f32, self.unit_height as f32)
    }
}
