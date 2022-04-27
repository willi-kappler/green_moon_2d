
use std::rc::Rc;
use std::fs;
use std::collections::HashMap;

use sdl2::image::LoadTexture;
use sdl2::render::TextureCreator;
use sdl2::video::WindowContext;

use nanoserde::DeJson;

use log::debug;

use crate::animation::GMAnimationT;
use crate::texture::GMTexture;
use crate::font::{GMFontT, GMBitmapFont};
use crate::error::GMError;


pub struct GMResources {
    texture_creator: TextureCreator<WindowContext>,
    textures: HashMap<String, Rc<GMTexture>>,
    fonts: HashMap<String, Rc<dyn GMFontT>>,
    animations: HashMap<String, Rc<dyn GMAnimationT>>,
}

impl GMResources {
    pub fn new(texture_creator: TextureCreator<WindowContext>) -> Self {
        Self {
            texture_creator,
            textures: HashMap::new(),
            fonts: HashMap::new(),
            animations: HashMap::new(),
        }
    }

    pub fn load_resources(&mut self, file_name: &str) -> Result<(), GMError> {
        debug!("GMResources::load_resources(), file_name: '{}'", file_name);

        let json_string = fs::read_to_string(file_name)?;
        let resource: GMResourceFormat = DeJson::deserialize_json(&json_string)?;

        for texture in resource.textures {
            let new_texture = self.create_texture(&texture.file_name, texture.cols, texture.unit_width, texture.unit_height)?;
            self.add_texture(&texture.texture_name, new_texture)?;
        }

        for font in resource.fonts {
            let new_font = self.create_bitmap_font(&font.texture_name, &font.char_mapping)?;
            self.add_font(&font.font_name, new_font)?;
        }

        for animation in resource.animations {
            let new_animation = self.create_animation(&animation.animation_type, &animation.frames)?;
            self.add_animation(&animation.animation_name, new_animation)?;
        }

        Ok(())
    }

    pub fn clear_resources(&mut self) {
        debug!("GMResources::clear_resources()");

        self.textures.clear();
        self.fonts.clear();
        self.animations.clear();
    }

    // Textures:
    pub fn clear_textures(&mut self) {
        debug!("GMResources::clear_textures()");

        self.textures.clear();
    }

    pub fn create_texture(&self, file_name: &str, cols: u32, unit_width: u32, unit_height: u32) -> Result<GMTexture, GMError> {
        debug!("GMResources::create_texture(), file_name: '{}', cols: '{}', unit_width: '{}', unit_height: '{}'",
            file_name, cols, unit_width, unit_height);

        let image = self.texture_creator.load_texture(file_name)
            .map_err(|_| GMError::CouldNotLoadTexture(file_name.to_string()))?;

        Ok(GMTexture::new(cols, unit_width, unit_height, image))
    }

    pub fn add_texture(&mut self, name: &str, texture: GMTexture) -> Result<(), GMError> {
        debug!("GMResources::add_texture(), name: '{}'", name);

        if self.textures.contains_key(name) {
            Err(GMError::TextureAlreadyExists(name.to_string()))
        } else {
            self.textures.insert(name.to_string(), Rc::new(texture));
            Ok(())
        }
    }

    pub fn remove_texture(&mut self, name: &str) -> Result<(), GMError> {
        debug!("GMResources::remove_texture(), name: '{}'", name);

        self.textures.remove(name)
            .map(|_| ())
            .ok_or(GMError::TextureNotFound(name.to_string()))
    }

    pub fn replace_texture(&mut self, name: &str, texture: GMTexture) -> Result<(), GMError> {
        debug!("GMResources::replace_texture(), name: '{}'", name);

        if self.textures.contains_key(name) {
            self.textures.insert(name.to_string(), Rc::new(texture));
            Ok(())
        } else {
            Err(GMError::TextureNotFound(name.to_string()))
        }
    }

    pub fn get_texture_clone(&self, name: &str) -> Result<Rc<GMTexture>, GMError> {
        debug!("GMResources::get_texture_clone(), name: '{}'", name);

        self.textures.get(name)
            .map(|texture| texture.clone())
            .ok_or(GMError::TextureNotFound(name.to_string()))
    }

    // Fonts:
    pub fn clear_fonts(&mut self) {
        debug!("GMResources::clear_fonts()");

        self.fonts.clear();
    }

    pub fn create_bitmap_font(&self, texture: &str, char_mapping: &str) -> Result<Rc<dyn GMFontT>, GMError> {
        debug!("GMResources::create_bitmap_font(), texture: '{}'", texture);

        let texture = self.get_texture_clone(texture)?;
        let font = GMBitmapFont::new(texture, char_mapping);

        Ok(Rc::new(font))
    }

    pub fn add_font(&mut self, name: &str, font: Rc<dyn GMFontT>) -> Result<(), GMError> {
        debug!("GMResources::add_font(), name: '{}'", name);

        if self.fonts.contains_key(name) {
            Err(GMError::FontAlreadyExists(name.to_string()))
        } else {
            self.fonts.insert(name.to_string(), font);
            Ok(())
        }
    }

    pub fn remove_font(&mut self, name: &str) -> Result<(), GMError> {
        debug!("GMResources::remove_font(), name: '{}'", name);

        self.fonts.remove(name)
            .map(|_| ())
            .ok_or(GMError::FontNotFound(name.to_string()))
    }

    pub fn replace_font(&mut self, name: &str, font: Rc<dyn GMFontT>) -> Result<(), GMError> {
        debug!("GMResources::replace_font(), name: '{}'", name);

        if self.fonts.contains_key(name) {
            self.fonts.insert(name.to_string(), font);
            Ok(())
        } else {
            Err(GMError::FontNotFound(name.to_string()))
        }
    }

    pub fn get_font_clone(&self, name: &str) -> Result<Rc<dyn GMFontT>, GMError> {
        debug!("GMResources::get_font_clone(), name: '{}'", name);

        self.fonts.get(name)
            .map(|font| font.clone())
            .ok_or(GMError::FontNotFound(name.to_string()))
    }


    // Animations:
    pub fn clear_animations(&mut self) {
        debug!("GMResources::clear_animations()");

        self.animations.clear();
    }

    pub fn create_animation(&self, animation_type: &str, frames: &[(usize, f32)]) -> Result<Rc<dyn GMAnimationT>, GMError> {
        debug!("GMResources::create_animation(), animation_type: '{}'", animation_type);

        todo!();
    }

    pub fn add_animation(&mut self, name: &str, animation: Rc<dyn GMAnimationT>) -> Result<(), GMError> {
        debug!("GMResources::add_animation(), name: '{}'", name);

        if self.animations.contains_key(name) {
            Err(GMError::AnimationAlreadyExists(name.to_string()))
        } else {
            self.animations.insert(name.to_string(), animation);
            Ok(())
        }
    }

    pub fn remove_animation(&mut self, name: &str) -> Result<(), GMError> {
        debug!("GMResources::remove_animation(), name: '{}'", name);

        self.animations.remove(name)
            .map(|_| ())
            .ok_or(GMError::AnimationNotFound(name.to_string()))
    }

    pub fn replace_animation(&mut self, name: &str, animation: Rc<dyn GMAnimationT>) -> Result<(), GMError> {
        debug!("GMResources::replace_animation(), name: '{}'", name);

        if self.animations.contains_key(name) {
            self.animations.insert(name.to_string(), animation);
            Ok(())
        } else {
            Err(GMError::AnimationNotFound(name.to_string()))
        }
    }

    pub fn get_animation_clone(&self, name: &str) -> Result<Rc<dyn GMAnimationT>, GMError> {
        debug!("GMResources::get_animation_clone(), name: '{}'", name);

        self.animations.get(name)
            .map(|animation| animation.clone())
            .ok_or(GMError::AnimationNotFound(name.to_string()))
    }


    // Sounds:

    // Musics:
}

#[derive(Debug, DeJson)]
struct GMResourceFormat {
    textures: Vec<GMTextureFormat>,
    fonts: Vec<GMFontFormat>,
    animations: Vec<GMAnimationFormat>,
    sounds: Vec<GMSoundFormat>,
    musics: Vec<GMMusicFormat>,
}

#[derive(Debug, DeJson)]
struct GMTextureFormat {
    texture_name: String,
    file_name: String,
    cols: u32,
    unit_width: u32,
    unit_height: u32,
}

#[derive(Debug, DeJson)]
struct GMFontFormat {
    font_name: String,
    texture_name: String,
    char_mapping: String,
}

#[derive(Debug, DeJson)]
struct GMAnimationFormat {
    animation_type: String,
    animation_name: String,
    frames: Vec<(usize, f32)>, // (texture index, duration in seconds)
}

#[derive(Debug, DeJson)]
struct GMSoundFormat {
    sound_name: String,
    file_name: String,
}

#[derive(Debug, DeJson)]
struct GMMusicFormat {
    sound_name: String,
    file_name: String,
}
