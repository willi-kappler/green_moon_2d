
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

    pub fn load_resources(&mut self, file_name: &str) {
        debug!("GMResources::load_resources(), file_name: '{}'", file_name);

        let json_string = match fs::read_to_string(file_name) {
            Ok(content) => {
                content
            }
            Err(e) => {
                panic!("Could not load resources: {}", e);
            }
        };

        let resources: GMResourceFormat = match DeJson::deserialize_json(&json_string) {
            Ok(resources) => {
                resources
            }
            Err(e) => {
                panic!("JSON error while parsing rescources: {}", e);
            }
        };

        for texture in resources.textures {
            let new_texture = self.create_texture(&texture.file_name, texture.cols, texture.unit_width, texture.unit_height);
            self.add_texture(&texture.name, new_texture);
        }

        for font in resources.fonts {
            let new_font = self.create_bitmap_font(&font.texture_name, &font.char_mapping);
            self.add_font(&font.name, new_font);
        }

        for animation in resources.animations {
            let new_animation = self.create_animation(&animation.animation_type, &animation.frames);
            self.add_animation(&animation.name, new_animation);
        }

        for sound in resources.sounds {
            dbg!(sound);
            todo!();
        }
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

    pub fn create_texture(&self, file_name: &str, cols: u32, unit_width: u32, unit_height: u32) -> GMTexture {
        debug!("GMResources::create_texture(), file_name: '{}', cols: '{}', unit_width: '{}', unit_height: '{}'",
            file_name, cols, unit_width, unit_height);

        let image = match self.texture_creator.load_texture(file_name) {
            Ok(image) => {
                image
            }
            Err(e) => {
                panic!("Error while loading texture: {}", e);
            }
        };

        GMTexture::new(cols, unit_width, unit_height, image)
    }

    pub fn add_texture(&mut self, name: &str, texture: GMTexture) {
        debug!("GMResources::add_texture(), name: '{}'", name);

        if self.textures.contains_key(name) {
            panic!("A texture with that name already exists: {}", name);
        } else {
            self.textures.insert(name.to_string(), Rc::new(texture));
        }
    }

    pub fn remove_texture(&mut self, name: &str) {
        debug!("GMResources::remove_texture(), name: '{}'", name);

        if self.textures.remove(name).is_some() {
            panic!("No texture with name {} found!", name);
        }
    }

    pub fn replace_texture(&mut self, name: &str, texture: GMTexture) {
        debug!("GMResources::replace_texture(), name: '{}'", name);

        if self.textures.contains_key(name) {
            self.textures.insert(name.to_string(), Rc::new(texture));
        } else {
            panic!("");
        }
    }

    pub fn get_texture_clone(&self, name: &str) -> Rc<GMTexture> {
        debug!("GMResources::get_texture_clone(), name: '{}'", name);

        match self.textures.get(name) {
            Some(texture) => {
                texture.clone()
            }
            None => {
                panic!("No texture with name {} found!", name);
            }
        }
    }

    // Fonts:
    pub fn clear_fonts(&mut self) {
        debug!("GMResources::clear_fonts()");

        self.fonts.clear();
    }

    pub fn create_bitmap_font(&self, texture: &str, char_mapping: &str) -> Rc<dyn GMFontT> {
        debug!("GMResources::create_bitmap_font(), texture: '{}'", texture);

        let texture = self.get_texture_clone(texture);
        let font = GMBitmapFont::new(texture, char_mapping);

        Rc::new(font)
    }

    pub fn add_font(&mut self, name: &str, font: Rc<dyn GMFontT>) {
        debug!("GMResources::add_font(), name: '{}'", name);

        if self.fonts.contains_key(name) {
            panic!("A font with the name {} already exists!", name);
        } else {
            self.fonts.insert(name.to_string(), font);
        }
    }

    pub fn remove_font(&mut self, name: &str) {
        debug!("GMResources::remove_font(), name: '{}'", name);

        if self.fonts.remove(name).is_some() {
            panic!("A font with the name {} does not exist!", name);
        }
    }

    pub fn replace_font(&mut self, name: &str, font: Rc<dyn GMFontT>) {
        debug!("GMResources::replace_font(), name: '{}'", name);

        if self.fonts.contains_key(name) {
            self.fonts.insert(name.to_string(), font);
        } else {
            panic!("");
        }
    }

    pub fn get_font_clone(&self, name: &str) -> Rc<dyn GMFontT> {
        debug!("GMResources::get_font_clone(), name: '{}'", name);

        match self.fonts.get(name) {
            Some(font) => {
                font.clone()
            }
            None => {
                panic!("A font with the name {} does not exist!", name);
            }
        }
    }


    // Animations:
    pub fn clear_animations(&mut self) {
        debug!("GMResources::clear_animations()");

        self.animations.clear();
    }

    pub fn create_animation(&self, animation_type: &str, frames: &[(usize, f32)]) -> Rc<dyn GMAnimationT> {
        debug!("GMResources::create_animation(), animation_type: '{}'", animation_type);

        dbg!(frames);

        todo!();
    }

    pub fn add_animation(&mut self, name: &str, animation: Rc<dyn GMAnimationT>) {
        debug!("GMResources::add_animation(), name: '{}'", name);

        if self.animations.contains_key(name) {
            panic!("An animation with name {} does already exist!", name);
        } else {
            self.animations.insert(name.to_string(), animation);
        }
    }

    pub fn remove_animation(&mut self, name: &str) {
        debug!("GMResources::remove_animation(), name: '{}'", name);

        if self.animations.remove(name).is_some() {
            panic!("An animation with name {} does not exist!", name);
        }
    }

    pub fn replace_animation(&mut self, name: &str, animation: Rc<dyn GMAnimationT>) {
        debug!("GMResources::replace_animation(), name: '{}'", name);

        if self.animations.contains_key(name) {
            self.animations.insert(name.to_string(), animation);
        } else {
            panic!("An animation with name {} does not exist!", name);
        }
    }

    pub fn get_animation_clone(&self, name: &str) -> Rc<dyn GMAnimationT> {
        debug!("GMResources::get_animation_clone(), name: '{}'", name);

        match self.animations.get(name) {
            Some(animation) => {
                animation.clone()
            }
            None => {
                panic!("An animation with name {} does not exist!", name);
            }
        }
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
    // musics: Vec<GMMusicFormat>,
}

#[derive(Debug, DeJson)]
struct GMTextureFormat {
    name: String,
    file_name: String,
    cols: u32,
    unit_width: u32,
    unit_height: u32,
}

#[derive(Debug, DeJson)]
struct GMFontFormat {
    name: String,
    texture_name: String,
    char_mapping: String,
}

#[derive(Debug, DeJson)]
struct GMAnimationFormat {
    name: String,
    animation_type: String,
    frames: Vec<(usize, f32)>, // (texture index, duration in seconds)
}

#[derive(Debug, DeJson)]
struct GMSoundFormat {
    // name: String,
    // file_name: String,
}

#[derive(Debug, DeJson)]
struct GMMusicFormat {
    // name: String,
    // file_name: String,
}
