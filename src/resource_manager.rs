

use crate::error::GMError;
use crate::font::{GMBitmapFont, GMFontT};
use crate::spritesheet::GMSpriteSheet;
use crate::sound::GMSound;
use crate::animation::{GMAnimationBackwardLoop, GMAnimationBackwardOnce, GMAnimationForwardLoop, GMAnimationForwardOnce, GMAnimationPingPong, GMAnimationT};

use macroquad::file::load_string;
use macroquad::math::Rect;
use nanoserde::DeJson;

use std::collections::HashMap;
use std::rc::Rc;

#[derive(Clone, Debug, Default, DeJson)]
pub struct GMResourceFormat {
    font_file: Option<String>,
    sprite_sheet_file: Option<String>,
    sounds_file: Option<String>,
    animations_file: Option<String>,
}

#[derive(Clone, Debug, Default, DeJson)]
pub struct GMFontFormat{
    name: String,
    image_file: String,
    char_width: f32,
    char_height: f32,
    char_order: String,
}

#[derive(Clone, Debug, Default, DeJson)]
pub struct GMFontFormatMultiple {
    fonts: Vec<GMFontFormat>,
}

#[derive(Clone, Debug, Default, DeJson)]
pub struct GMSpriteSheetFormat {
    name: String,
    image_file: String,
}

#[derive(Clone, Debug, Default, DeJson)]
pub struct GMSpriteSheetFormatMultiple {
    sprite_sheets: Vec<GMSpriteSheetFormat>,
}

#[derive(Clone, Debug, Default, DeJson)]
pub struct GMFrameFormat {
    x: f32,
    y: f32,
    w: f32,
    h: f32,
    duration: f64,
}

#[derive(Clone, Debug, DeJson)]
pub enum GMAnimationType {
    ForeWardOnce,
    ForeWardLoop,
    BackwardOnce,
    BackwardLoop,
    PingPong,
}

#[derive(Clone, Debug, DeJson)]
pub struct GMAnimationFormat {
    name: String,
    animation_type: GMAnimationType,
    frames: Vec<GMFrameFormat>,
}

#[derive(Clone, Debug, DeJson)]
pub struct GMAnimationFormatMultiple {
    animations: Vec<GMAnimationFormat>,
}

#[derive(Clone, Debug, Default, DeJson)]
pub struct GMSoundFormat {
    name: String,
    sound_file: String,
}

#[derive(Clone, Debug, Default, DeJson)]
pub struct GMSoundFormatMultiple {
    sounds: Vec<GMSoundFormat>,
}

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
        let json = load_string(file_name).await?;
        let result: GMResourceFormat = DeJson::deserialize_json(&json)?;
        let mut resource = Self::new();
        if let Some(font_file) = result.font_file {
            resource.fonts_from_file(&font_file).await?;
        }
        if let Some(sprite_sheet_file) = result.sprite_sheet_file {
            resource.sprite_sheets_from_file(&&sprite_sheet_file).await?;
        }
        if let Some(sounds_file) = result.sounds_file {
            resource.sounds_from_file(&&sounds_file).await?;
        }
        if let Some(animations_file) = result.animations_file {
            resource.animations_from_file(&&animations_file).await?;
        }
        Ok(resource)
    }
    pub fn add_font<T: 'static + GMFontT>(&mut self, name: &str, font: T) {
        self.fonts.insert(name.to_string(), Rc::new(font));
    }
    pub async fn fonts_from_file(&mut self, file_name: &str) -> Result<(), GMError>{
        let json = load_string(file_name).await?;
        let result: GMFontFormatMultiple = DeJson::deserialize_json(&json)?;

        for item in result.fonts {
            let font = GMBitmapFont::new_rc(&item.image_file,
                item.char_width, item.char_height, &item.char_order).await?;
            self.fonts.insert(item.name, font);
        }

        Ok(())
    }
    pub fn get_font(&self, name: &str) -> Option<Rc<dyn GMFontT>> {
        self.fonts.get(name).map(|v| v.clone())
    }
    pub fn add_sprite_sheet(&mut self, name: &str, sprite_sheet: GMSpriteSheet) {
        self.sprite_sheets.insert(name.to_string(), Rc::new(sprite_sheet));
    }
    pub async fn sprite_sheets_from_file(&mut self, file_name: &str) -> Result<(), GMError> {
        let json = load_string(file_name).await?;
        let result: GMSpriteSheetFormatMultiple = DeJson::deserialize_json(&json)?;

        for item in result.sprite_sheets {
            let sprite_sheet = GMSpriteSheet::new_rc(&item.image_file).await?;
            self.sprite_sheets.insert(item.name, sprite_sheet);
        }

        Ok(())
    }
    pub fn get_sprite_sheet(&self, name: &str) -> Option<Rc<GMSpriteSheet>> {
        self.sprite_sheets.get(name).map(|v| v.clone())
    }
    pub fn add_animation<T: 'static + GMAnimationT>(&mut self, name: &str, animation: T) {
        self.animations.insert(name.to_string(), Box::new(animation));
    }
    pub async fn animations_from_file(&mut self, file_name: &str) -> Result<(), GMError> {
        let json = load_string(file_name).await?;
        let result: GMAnimationFormatMultiple = DeJson::deserialize_json(&json)?;

        for item in result.animations {
            let frames: Vec<(Rect, f64)> = item.frames.iter().map(|f| (Rect::new(f.x, f.y, f.w, f.h), f.duration)).collect();
            use GMAnimationType::*;
            let animation = match item.animation_type {
                ForeWardOnce => {
                    GMAnimationForwardOnce::new_box(&frames)
                }
                ForeWardLoop => {
                    GMAnimationForwardLoop::new_box(&frames)
                }
                BackwardOnce => {
                    GMAnimationBackwardOnce::new_box(&frames)
                }
                BackwardLoop => {
                    GMAnimationBackwardLoop::new_box(&frames)
                }
                PingPong => {
                    GMAnimationPingPong::new_box(&frames)
                }
            };
            self.animations.insert(item.name, animation);
        }

        Ok(())
    }
    pub fn get_animation(&self, name: &str) -> Option<Box<dyn GMAnimationT>> {
        self.animations.get(name).map(|v| v.clone_animation())
    }
    pub fn add_sound(&mut self, name: &str, sound: GMSound) {
        self.sounds.insert(name.to_string(), Rc::new(sound));
    }
    pub async fn sounds_from_file(&mut self, file_name: &str) -> Result<(), GMError> {
        let json = load_string(file_name).await?;
        let result: GMSoundFormatMultiple = DeJson::deserialize_json(&json)?;

        for item in result.sounds {
            let sound = GMSound::new_rc(&item.sound_file).await?;
            self.sounds.insert(item.name, sound);
        }

        Ok(())
    }
    pub fn get_sound(&self, name: &str) -> Option<Rc<GMSound>> {
        self.sounds.get(name).map(|v| v.clone())
    }
}
