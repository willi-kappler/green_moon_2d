

use crate::error::GMError;
use crate::font::GMFontT;
use crate::spritesheet::GMSpriteSheet;
use crate::sound::GMSound;
use crate::animation::GMAnimationT;

use macroquad::file::load_string;

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
    pub async fn new_from_file(file_name: &str) -> Result<Self, GMError> {
        // TODO: read resources from file
        let result = load_string(file_name).await?;
        Self::new_from_json(&result)
    }
    pub fn new_from_json(json: &str) -> Result<Self, GMError> {
        // TODO: create from JSON string
        let mut result = Self::new();
        Ok(result)
    }
    pub fn add_font<T: 'static + GMFontT>(&mut self, name: &str, font: T) {
        self.fonts.insert(name.to_string(), Rc::new(font));
    }
    pub async fn fonts_from_file(&mut self, file_name: &str) -> Result<(), GMError>{
        // TODO: read in font file
        let result = load_string(file_name).await?;
        self.fonts_from_json(&result)
    }
    pub fn fonts_from_json(&mut self, json: &str) -> Result<(), GMError> {
        // TODO: create frons from JSON string
        Ok(())
    }
    pub fn get_font(&self, name: &str) -> Option<Rc<dyn GMFontT>> {
        self.fonts.get(name).map(|v| v.clone())
    }
    pub fn add_sprite_sheet(&mut self, name: &str, sprite_sheet: GMSpriteSheet) {
        self.sprite_sheets.insert(name.to_string(), Rc::new(sprite_sheet));
    }
    pub async fn sprite_sheets_from_file(&mut self, file_name: &str) -> Result<(), GMError> {
        // TODO: read multiple sprite sheets from file
        let result = load_string(file_name).await?;
        self.sprite_sheets_from_json(&result)
    }
    pub fn sprite_sheets_from_json(&mut self, json: &str) -> Result<(), GMError> {
        // TODO: create sprite sheets from JSON string
        Ok(())
    }
    pub fn get_sprite_sheet(&self, name: &str) -> Option<Rc<GMSpriteSheet>> {
        self.sprite_sheets.get(name).map(|v| v.clone())
    }
    pub fn add_animation<T: 'static + GMAnimationT>(&mut self, name: &str, animation: T) {
        self.animations.insert(name.to_string(), Box::new(animation));
    }
    pub async fn animations_from_file(&mut self, file_name: &str) -> Result<(), GMError> {
        // TODO: read in animations from file
        let result = load_string(file_name).await?;
        self.animation_from_json(&result)
    }
    pub fn animation_from_json(&mut self, json: &str) -> Result<(), GMError> {
        // TODO: create animation from JSON string
        Ok(())
    }
    pub fn get_animation(&self, name: &str) -> Option<Box<dyn GMAnimationT>> {
        self.animations.get(name).map(|v| v.clone_animation())
    }
    pub fn add_sound(&mut self, name: &str, sound: GMSound) {
        self.sounds.insert(name.to_string(), Rc::new(sound));
    }
    pub async fn sounds_from_file(&mut self, file_name: &str) -> Result<(), GMError> {
        // TODO: read in sound file
        let result = load_string(file_name).await?;
        self.sounds_from_json(&result)

    }
    pub fn sounds_from_json(&mut self, json: &str) -> Result<(), GMError> {
        // TODO: create sounds from JSON string
        Ok(())
    }
    pub fn get_sound(&self, name: &str) -> Option<Rc<GMSound>> {
        self.sounds.get(name).map(|v| v.clone())
    }
}
