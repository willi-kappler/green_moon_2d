


pub struct GM_Texture {
    width: u32,
    height: u32,
    data: Vec<u32>,
}

impl GM_Texture {
    pub fn new(width: u32, height: u32, data: Vec<u32>) -> GM_Texture {
        GM_Texture {
            width,
            height,
            data,
        }
    }
}

