
pub struct GM_SpriteSheet {
    pub (crate) texture_id: usize,
    pub (crate) cell_width: u32,
    pub (crate) cell_height: u32,
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

}
