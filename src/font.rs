use std::rc::Rc;

use crate::error::GMError;
use crate::bitmap::GMBitmap;


pub struct GMFontManager {
    fonts: Vec<Rc<GMBitmapFont>>,
}

impl GMFontManager {
    pub fn new() -> GMFontManager {
        GMFontManager {
            fonts: Vec::new(),
        }
    }

    pub fn get_font_by_name(&self, font_name: &str) -> Result<Rc<GMBitmapFont>, GMError> {
        for font in self.fonts.iter() {
            if font_name == font.name {
                return Ok(font.clone())
            }
        }

        Err(GMError::FontNotFound(font_name.to_string()))
    }
}


pub struct GMBitmapFont {
    name: String,
    width: u32,
    height: u32,
    char_width: u32,
    char_height: u32,
    data: Vec<GMBitmap>,
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

    pub fn get_bitmap(&self, c: char) -> &GMBitmap {
        &self.data[0]
    }
}
