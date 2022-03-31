

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
    pub fn draw(&mut self, dx: i32, dy: i32, index: u32) {
        let yi = index / self.cols;
        let xi = index - (yi * self.cols);

        let sx = (xi * self.unit_width) as i32;
        let sy = (yi * self.unit_height) as i32;

        let src_rect = Rect::new(sx, sy, self.unit_width, self.unit_height);
        let dst_rect = Rect::new(dx, dy, self.unit_width, self.unit_height);


    }
}
