

use crate::font::GMFontT;
use crate::spritesheet::GMSpriteSheet;
use crate::sound::GMSound;
use crate::animation::{GMAnimationT};

use std::collections::HashMap;
use std::rc::Rc;

pub struct GMResourceManager {
    fonts: HashMap<String, Rc<dyn GMFontT>>,
    sprite_sheets: HashMap<String, Rc<GMSpriteSheet>>,
    sounds: HashMap<String, Rc<GMSound>>,
    animations: HashMap<String, Box<dyn GMAnimationT>>,
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
    pub fn new_from_json(json: &str) -> Self {
        // TODO: create from JSON string
        let mut result = Self::new();
        result
    }
    pub fn add_font<T: 'static + GMFontT>(&mut self, name: &str, font: T) {
        self.fonts.insert(name.to_string(), Rc::new(font));
    }
    pub fn fonts_from_file(&mut self, file_name: &str) {
        // TODO: read in font file
        let result = "";
        self.fonts_from_json(result);
    }
    pub fn fonts_from_json(&mut self, json: &str) {
        // TODO: create frons from JSON string
    }
    pub fn get_font(&self, name: &str) -> Option<Rc<dyn GMFontT>> {
        self.fonts.get(name).map(|v| v.clone())
    }
    pub fn add_sprite_sheet(&mut self, name: &str, sprite_sheet: GMSpriteSheet) {
        self.sprite_sheets.insert(name.to_string(), Rc::new(sprite_sheet));
    }
    pub fn sprite_sheets_from_file(&mut self, file_name: &str) {
        // TODO: read multiple sprite sheets from file
        let result = "";
        self.sprite_sheets_from_json(result);
    }
    pub fn sprite_sheets_from_json(&mut self, json: &str) {
        // TODO: create sprite sheets from JSON string
    }
    pub fn get_sprite_sheet(&self, name: &str) -> Option<Rc<GMSpriteSheet>> {
        self.sprite_sheets.get(name).map(|v| v.clone())
    }
    pub fn add_animation<T: 'static + GMAnimationT>(&mut self, name: &str, animation: T) {
        self.animations.insert(name.to_string(), Box::new(animation));
    }
    pub fn animations_from_file(&mut self, file_name: &str) {
        // TODO: read in animations from file
        let mut result = "";
        self.animation_from_json(result);
    }
    pub fn animation_from_json(&mut self, json: &str) {
        // TODO: create animation from JSON string
    }
    pub fn get_animation(&self, name: &str) -> Option<Box<dyn GMAnimationT>> {
        self.animations.get(name).map(|v| v.clone_animation())
    }
}
