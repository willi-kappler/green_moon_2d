

use sdl2::rect::Rect;

pub struct GMTexture {
    width: u32,
    height: u32,
    rows: u32,
    cols: u32,
    unit_width: u32,
    unit_height: u32,
    data: u8, // TODO
}

impl GMTexture {
    pub fn draw(&self, dx: f32, dy: f32, index: u32) {
        let yi = index / self.cols;
        let xi = index - (yi * self.cols);

        let sx = (xi * self.unit_width) as i32;
        let sy = (yi * self.unit_height) as i32;

        let src_rect = Rect::new(sx, sy, self.unit_width, self.unit_height);
        let dst_rect = Rect::new(dx as i32, dy as i32, self.unit_width, self.unit_height);


    }

    pub fn get_unit_dimension(&self) -> (f32, f32) {
        (self.unit_width as f32, self.unit_height as f32)
    }
}
