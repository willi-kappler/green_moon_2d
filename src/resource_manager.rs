

use crate::font::{GMBitmapFont, GMFontT};
use crate::spritesheet::GMSpriteSheet;
use crate::sound::GMSound;
use crate::animation::{GMAnimationT};

use std::collections::HashMap;
use std::rc::Rc;

pub struct GMResourceManager {
    fonts: HashMap<String, Rc<dyn GMFontT>>,
    sprite_sheets: HashMap<String, Rc<GMSpriteSheet>>,
    sounds: HashMap<String, Rc<GMSound>>,
    animations: HashMap<String, Rc<dyn GMAnimationT>>,
}

impl GMResourceManager {
    pub fn new() -> Self {
        Self {
            fonts: HashMap::new(),
            sprite_sheets: HashMap::new(),
            sounds: HashMap::new(),
            animations: HashMap::new(),
        }
    }
    pub fn new_from_file(file_name: &str) -> Self {
        // TODO: read resources from file
        let result = "";
        Self::new_from_json(result)
    }
    pub fn new_from_json(json: &str) -> Self{
        // TODO: create from JSON string
        let mut result = Self::new();
        result
    }
    pub fn add_font<T: 'static + GMFontT>(&mut self, name: &str, font: T) {
        self.fonts.insert(name.to_string(), Rc::new(font));
    }
    pub fn fonts_from_file(file_name: &str) -> Self {
        // TODO: read in font file
        let result = "";
        Self::fonts_from_json(result)
    }
    pub fn fonts_from_json(json: &str) -> Self {
        // TODO: parse frons from JSON string
        let mut result = Self::new();

        result
    }
    pub fn add_sprite_sheet(&mut self, name: &str, sprite_sheet: GMSpriteSheet) {
        self.sprite_sheets.insert(name.to_string(), Rc::new(sprite_sheet));
    }
    pub fn sprite_sheets_from_file(file_name: &str) -> Self  {
        // TODO: read multiple sprite sheets from file
        let result = "";
        Self::sprite_sheets_from_json(result)
    }
    pub fn sprite_sheets_from_json(json: &str) -> Self {
        // TODO: create sprite sheets from json string
        let mut result = Self::new();

        result
    }
}
