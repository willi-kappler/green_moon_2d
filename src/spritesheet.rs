
// Local modules
use crate::texture::{GM_Texture};
use crate::canvas::{GM_Canvas};


pub struct GM_SpriteSheet {
    texture_id: usize,
    cell_width: u32,
    cell_height: u32,
    rows: u32,
    cols: u32,
}

impl GM_SpriteSheet {
    pub fn new() -> GM_SpriteSheet {
        GM_SpriteSheet {
            texture_id: 0,
            cell_width: 0,
            cell_height: 0,
            rows: 0,
            cols: 0,
        }
    }

    pub fn frame_to_coordinates(&self, id: usize) -> (u32, u32) {
        let id2 = id as u32;
        let row = id2 / self.cols;
        let col = id2 - (row * self.cols);

        (col * self.cell_width, row * self.cell_height)
    }

    pub fn draw(&self, x: u32, y: u32, current_frame: usize, texture_pool: &Vec<GM_Texture>, canvas: &mut GM_Canvas) {
        // TODO move from GM_Sprite to here
        let texture = &texture_pool[self.texture_id];
        let (tx, ty) = self.frame_to_coordinates(current_frame);

        canvas.draw_sub_texture(x, y, texture, tx, ty, self.cell_width, self.cell_height);
    }
}
