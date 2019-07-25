


#[derive(Debug, Clone)]
pub struct GM_Dimension {
    width: u32,
    height: u32,
}

impl GM_Dimension {
    pub fn new(width: u32, height: u32) -> GM_Dimension {
        GM_Dimension {
            width,
            height,
        }
    }

    pub fn get_width(&self) -> u32 {
        self.width
    }

    pub fn get_height(&self) -> u32 {
        self.height
    }
}
