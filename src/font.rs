use crate::error::GMError;



pub struct GMFontManager {
    fonts: Vec<GMBitmapFont>,
}

impl GMFontManager {
    pub fn new() -> GMFontManager {
        GMFontManager {
            fonts: Vec::new(),
        }
    }

    pub fn get_font_by_name(&self, font_name: &str) -> Result<&GMBitmapFont, GMError> {
        for font in self.fonts.iter() {
            if font_name == font.name {
                return Ok(font)
            }
        }

        Err(GMError::FontNotFound(font_name.to_string()))
    }
}


pub struct GMBitmapFont {
    name: String,
    char_width: u32,
    char_height: u32,
    data: bool // TODO
}

impl GMBitmapFont {
}
