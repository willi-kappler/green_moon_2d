

use crate::error::GMError;
use crate::font::{GMBitmapFont, GMFontT};
use crate::spritesheet::GMSpriteSheet;
use crate::sprite::GMSprite;
use crate::sound::GMSound;
use crate::animation::{GMAnimationBackwardLoop, GMAnimationBackwardOnce, GMAnimationForwardLoop, GMAnimationForwardOnce, GMAnimationPingPong, GMAnimationT};

use macroquad::file::load_string;
use macroquad::math::Rect;
use nanoserde::DeJson;

use log::{debug, info};

use std::collections::HashMap;
use std::rc::Rc;
use std::path::Path;

#[derive(Clone, Debug, Default, DeJson)]
pub struct GMResourceFormat {
    font_files: Option<Vec<String>>,
    sprite_sheets: Option<Vec<GMSpriteSheetFormat>>,
    sprites: Option<Vec<GMSpriteFormat>>,
    sounds: Option<Vec<GMSoundFormat>>,
    animation_files: Option<Vec<String>>,
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
pub struct GMSpriteSheetFormat {
    name: String,
    file: String,
}

#[derive(Clone, Debug, Default, DeJson)]
pub struct GMSpriteFormat {
    name: String,
    sprite_sheet: String,
    animation: String,
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
    ForwardOnce,
    ForwardLoop,
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
    file: String,
}

pub struct GMResourceManager {
    fonts: HashMap<String, Rc<dyn GMFontT>>,
    sprite_sheets: HashMap<String, Rc<GMSpriteSheet>>,
    sprites: HashMap<String, GMSprite>,
    sounds: HashMap<String, Rc<GMSound>>,
    animations: HashMap<String, Box<dyn GMAnimationT>>,
}

impl GMResourceManager {
    pub fn new() -> Self {
        Self {
            fonts: HashMap::new(),
            sprite_sheets: HashMap::new(),
            sprites: HashMap::new(),
            sounds: HashMap::new(),
            animations: HashMap::new(),
        }
    }
    pub async fn new_from_file(file_name: &str) -> Result<Self, GMError> {
        info!("Loading resource file: '{}'", file_name);
        let json = load_string(file_name).await?;
        let result: GMResourceFormat = DeJson::deserialize_json(&json)?;
        let mut resource = Self::new();

        if let Some(font_files) = result.font_files {
            for file_name in font_files.iter() {
                resource.fonts_from_file(&file_name).await?;
            }
        }
        if let Some(sprite_sheets) = result.sprite_sheets {
            for item in sprite_sheets.into_iter() {
                debug!("SpriteSheet name: '{}'", item.name);

                let sprite_sheet = GMSpriteSheet::new_rc(&item.file).await?;
                resource.sprite_sheets.insert(item.name, sprite_sheet);
            }
        }
        if let Some(sounds) = result.sounds {
            for item in sounds.into_iter() {
                debug!("Sound name: '{}'", item.name);

                let sound = GMSound::new_rc(&item.file).await?;
                resource.sounds.insert(item.name, sound);
            }
        }
        if let Some(animation_files) = result.animation_files {
            for file_name in animation_files.iter() {
                resource.animations_from_file(file_name).await?;
            }
        }
        if let Some(sprites) = result.sprites {
            for item in sprites.into_iter() {
                debug!("Sprite name: '{}', sprite sheet: '{}', animation: '{}'", item.name, item.sprite_sheet, item.animation);

                let sprite = GMSprite::new(
                    &resource.get_sprite_sheet(&item.sprite_sheet).unwrap(),
                    resource.get_animation(&item.animation).unwrap(), 0.0, 0.0);
                resource.sprites.insert(item.name, sprite);
            }
        }

        Ok(resource)
    }
    pub fn add_font<T: 'static + GMFontT>(&mut self, name: &str, font: T) {
        self.fonts.insert(name.to_string(), Rc::new(font));
    }
    pub async fn fonts_from_file(&mut self, file_name: &str) -> Result<(), GMError>{
        info!("Loading font file: '{}'", file_name);
        let json = load_string(file_name).await?;
        let item: GMFontFormat = DeJson::deserialize_json(&json)?;

        debug!("Processing font...");
        debug!("Font name: '{}', width: {}, height: {}", item.name, item.char_width, item.char_height);

        let p1 = Path::new(file_name);
        let parent = p1.parent().unwrap();
        let p2 = Path::new(&item.image_file);
        let new_path = parent.join(p2);
        let os_str = new_path.into_os_string();
        let img_file_name = os_str.into_string().unwrap();

        debug!("Font image file: '{}'", img_file_name);

        let font = GMBitmapFont::new_rc(&img_file_name,
            item.char_width, item.char_height, &item.char_order).await?;

        self.fonts.insert(item.name, font);

        Ok(())
    }
    pub fn get_font(&self, name: &str) -> Option<Rc<dyn GMFontT>> {
        self.fonts.get(name).map(|v| v.clone())
    }
    pub fn remove_font(&mut self, name: &str) {
        self.fonts.remove(name);
    }
    pub fn add_sprite_sheet(&mut self, name: &str, sprite_sheet: GMSpriteSheet) {
        self.sprite_sheets.insert(name.to_string(), Rc::new(sprite_sheet));
    }
    pub fn get_sprite_sheet(&self, name: &str) -> Option<Rc<GMSpriteSheet>> {
        self.sprite_sheets.get(name).map(|v| v.clone())
    }
    pub fn remove_sprite_sheet(&mut self, name: &str) {
        self.sprite_sheets.remove(name);
    }
    pub fn add_sprite(&mut self, name: &str, sprite: &GMSprite) {
        self.sprites.insert(name.to_string(), sprite.clone());
    }
    pub fn get_sprite(&self, name: &str) -> Option<&GMSprite> {
        self.sprites.get(name)
    }
    pub fn remove_sprite(&mut self, name: &str) {
        self.sprites.remove(name);
    }
    pub fn add_animation<T: 'static + GMAnimationT>(&mut self, name: &str, animation: T) {
        self.animations.insert(name.to_string(), Box::new(animation));
    }
    pub async fn animations_from_file(&mut self, file_name: &str) -> Result<(), GMError> {
        info!("Loading animation file: '{}'", file_name);
        let json = load_string(file_name).await?;
        let result: GMAnimationFormatMultiple = DeJson::deserialize_json(&json)?;

        for item in result.animations.into_iter() {
            debug!("Processing animations...");
            debug!("Animation name: '{}', type: {:?}", item.name, item.animation_type);

            let frames: Vec<(Rect, f64)> = item.frames.iter().map(|f| (Rect::new(f.x, f.y, f.w, f.h), f.duration)).collect();

            use GMAnimationType::*;

            let animation = match item.animation_type {
                ForwardOnce => {
                    GMAnimationForwardOnce::new_box(&frames)
                }
                ForwardLoop => {
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
    pub fn remove_animation(&mut self, name: &str) {
        self.animations.remove(name);
    }
    pub fn add_sound(&mut self, name: &str, sound: GMSound) {
        self.sounds.insert(name.to_string(), Rc::new(sound));
    }
    pub fn get_sound(&self, name: &str) -> Option<Rc<GMSound>> {
        self.sounds.get(name).map(|v| v.clone())
    }
    pub fn remove_sound(&mut self, name: &str) {
        self.sounds.remove(name);
    }
}
