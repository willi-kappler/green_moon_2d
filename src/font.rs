use crate::bitmap::GMBitmap;
use crate::resource_manager::GMName;

pub struct GMBitmapFont {
    pub(crate) name: String,
    pub(crate) width: u32,
    pub(crate) height: u32,
    pub(crate) char_width: u32,
    pub(crate) char_height: u32,
    pub(crate) data: Vec<GMBitmap>,
}

impl GMBitmapFont {
    pub fn from_img_file(name: &str, char_width: u32, char_height: u32, file_name: &str) -> GMBitmapFont {
        // TODO: Read from image file
        let width = 0;
        let height = 0;
        let data = Vec::new();

        GMBitmapFont {
            name: name.to_string(),
            width,
            height,
            char_width,
            char_height,
            data
        }
    }

    pub fn from_config_file(file_name: &str) -> GMBitmapFont {
        // TODO: Read from config file (TOML ?)
        GMBitmapFont::from_img_file("", 0, 0, "")
    }


    pub fn get_char_width(&self) -> u32 {
        self.char_width
    }

    pub fn get_bitmap(&self, c: u8) -> &GMBitmap {
        // TODO: get index of correct bitmap
        &self.data[0]
    }
}

impl GMName for GMBitmapFont {
    fn get_name(&self) -> &str {
        &self.name
    }

    fn set_name(&mut self, name: &str) {
        self.name = name.to_string();
    }

    fn has_name(&self, name: &str) -> bool {
        self.name == name
    }

    fn has_prefix(&self, name: &str) -> bool {
        self.name.starts_with(name)
    }
}
