

use sdl2::rect::Rect;

use crate::GMContext;

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
    pub fn draw(&self, dx: f32, dy: f32, index: u32, _context: &mut GMContext) {
        let yi = index / self.cols;
        let xi = index - (yi * self.cols);

        let sx = (xi * self.unit_width) as i32;
        let sy = (yi * self.unit_height) as i32;

        let _src_rect = Rect::new(sx, sy, self.unit_width, self.unit_height);
        let _dst_rect = Rect::new(dx as i32, dy as i32, self.unit_width, self.unit_height);


    }

    pub fn get_unit_dimension(&self) -> (f32, f32) {
        (self.unit_width as f32, self.unit_height as f32)
    }
}
