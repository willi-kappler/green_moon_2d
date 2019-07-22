

use super::texture::{GM_Texture};



pub struct GM_SpriteSheet {
    texture: GM_Texture,
    cell_width: u16,
    cell_height: u16,
    rows: u16,
    cols: u16,
}

impl GM_SpriteSheet {
    pub fn new(texture: GM_Texture, cell_width: u16, cell_height: u16,
        rows: u16, cols: u16) -> GM_SpriteSheet {
        GM_SpriteSheet {
            texture,
            cell_width,
            cell_height,
            rows,
            cols,
        }
    }

    pub fn frame_to_coordinates(&self, id: u16) -> (u32, u32) {
        let row = id / self.cols;
        let col = id - (row * self.cols);

        ((col * self.cell_width) as u32, (row * self.cell_height) as u32)
    }
}
