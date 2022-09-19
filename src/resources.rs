
use std::rc::Rc;
use std::fs;
use std::path::Path;
use std::collections::HashMap;

use sdl2::image::LoadTexture;
use sdl2::render::TextureCreator;
use sdl2::video::WindowContext;

use nanoserde::DeJson;

use log::debug;

use crate::animation::{GMAnimation, GMAnimationBuilder};
use crate::texture::GMTexture;
use crate::bitmap_text::{GMBitmapFont};
use crate::util::{error_panic, GMRepetition};


pub struct GMResources {
    texture_creator: TextureCreator<WindowContext>,
    textures: HashMap<String, Rc<GMTexture>>,
    fonts: HashMap<String, Rc<GMBitmapFont>>,
    animations: HashMap<String, GMAnimation>,
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

    pub fn load_resources<P: AsRef<Path>>(&mut self, path: P) {
        debug!("GMResources::load_resources(), file_name: '{:?}'", path.as_ref());

        let json_string = match fs::read_to_string(path) {
            Ok(content) => {
                content
            }
            Err(e) => {
                error_panic(&format!("Could not load resources: '{}'", e));
            }
        };

        let resources: GMResourceFormat = match DeJson::deserialize_json(&json_string) {
            Ok(resources) => {
                resources
            }
            Err(e) => {
                error_panic(&format!("JSON error while parsing resources: '{}'", e));
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
            todo!("handle sound: '{:?}'", sound);
        }
    }

    pub fn clear_resources(&mut self) {
        debug!("GMResources::clear_resources()");

        self.textures.clear();
        self.fonts.clear();
        self.animations.clear();
    }

    // Textures:

    fn no_texture_found(&self, texture_name: &str) -> ! {
        error_panic(&format!("No texture with name '{}' found!", texture_name));
    }

    pub fn clear_textures(&mut self) {
        debug!("GMResources::clear_textures()");

        self.textures.clear();
    }

    pub fn create_texture<P: AsRef<Path>>(&self, path: P, cols: u32, unit_width: u32, unit_height: u32) -> GMTexture {
        debug!("GMResources::create_texture(), path: '{:?}', cols: '{}', unit_width: '{}', unit_height: '{}'",
            path.as_ref(), cols, unit_width, unit_height);

        let image = match self.texture_creator.load_texture(path) {
            Ok(image) => {
                image
            }
            Err(e) => {
                error_panic(&format!("Error while loading texture: {}", e));
            }
        };

        GMTexture::new(cols, unit_width, unit_height, image)
    }

    pub fn add_texture(&mut self, name: &str, texture: GMTexture) {
        debug!("GMResources::add_texture(), name: '{}'", name);

        if self.textures.contains_key(name) {
            error_panic(&format!("A texture with that name already exists: '{}'", name));
        } else {
            self.textures.insert(name.to_string(), Rc::new(texture));
        }
    }

    pub fn remove_texture(&mut self, name: &str) {
        debug!("GMResources::remove_texture(), name: '{}'", name);

        if self.textures.remove(name).is_some() {
            self.no_texture_found(name);
        }
    }

    pub fn replace_texture(&mut self, name: &str, texture: GMTexture) {
        debug!("GMResources::replace_texture(), name: '{}'", name);

        if self.textures.contains_key(name) {
            self.textures.insert(name.to_string(), Rc::new(texture));
        } else {
            self.no_texture_found(name);
        }
    }

    pub fn get_texture(&self, name: &str) -> &Rc<GMTexture> {
        debug!("GMResources::get_texture(), name: '{}'", name);

        match self.textures.get(name) {
            Some(texture) => {
                texture
            }
            None => {
                self.no_texture_found(name);
            }
        }
    }

    // Fonts:
    fn no_font_found(&self, font_name: &str) -> ! {
        error_panic(&format!("No font with name '{}' found!", font_name));
    }

    pub fn clear_fonts(&mut self) {
        debug!("GMResources::clear_fonts()");

        self.fonts.clear();
    }

    pub fn create_bitmap_font(&self, texture: &str, char_mapping: &str) -> Rc<GMBitmapFont> {
        debug!("GMResources::create_bitmap_font(), texture: '{}'", texture);

        let texture = self.get_texture(texture);
        let font = GMBitmapFont::new(texture, char_mapping);

        Rc::new(font)
    }

    pub fn add_font(&mut self, name: &str, font: Rc<GMBitmapFont>) {
        debug!("GMResources::add_font(), name: '{}'", name);

        if self.fonts.contains_key(name) {
            error_panic(&format!("A font with the name '{}' already exists!", name));
        } else {
            self.fonts.insert(name.to_string(), font);
        }
    }

    pub fn remove_font(&mut self, name: &str) {
        debug!("GMResources::remove_font(), name: '{}'", name);

        if self.fonts.remove(name).is_some() {
            self.no_font_found(name);
        }
    }

    pub fn replace_font(&mut self, name: &str, font: Rc<GMBitmapFont>) {
        debug!("GMResources::replace_font(), name: '{}'", name);

        if self.fonts.contains_key(name) {
            self.fonts.insert(name.to_string(), font);
        } else {
            self.no_font_found(name);
        }
    }

    pub fn get_font(&self, name: &str) -> &Rc<GMBitmapFont> {
        debug!("GMResources::get_font(), name: '{}'", name);

        match self.fonts.get(name) {
            Some(font) => {
                font
            }
            None => {
                self.no_font_found(name);
            }
        }
    }


    // Animations:
    fn no_animation_found(&self, animation_name: &str) -> ! {
        error_panic(&format!("No animation with name '{}' found!", animation_name));
    }

    pub fn clear_animations(&mut self) {
        debug!("GMResources::clear_animations()");

        self.animations.clear();
    }

    pub fn create_animation(&self, animation_type: &str, frames: &[(u32, f32)]) -> GMAnimation {
        debug!("GMResources::create_animation(), animation_type: '{}'", animation_type);

        let repetition = GMRepetition::from(animation_type);
        GMAnimationBuilder::new(frames)
            .with_repetition(repetition).build()
    }

    pub fn add_animation(&mut self, name: &str, animation: GMAnimation) {
        debug!("GMResources::add_animation2(), name: '{}'", name);

        if self.animations.contains_key(name) {
            error_panic(&format!("An animation with name '{}' does already exist!", name));
        } else {
            self.animations.insert(name.to_string(), animation);
        }
    }

    pub fn remove_animation(&mut self, name: &str) {
        debug!("GMResources::remove_animation(), name: '{}'", name);

        if self.animations.remove(name).is_some() {
            self.no_animation_found(name);
        }
    }

    pub fn replace_animation(&mut self, name: &str, animation: GMAnimation) {
        debug!("GMResources::replace_animation(), name: '{}'", name);

        if self.animations.contains_key(name) {
            self.animations.insert(name.to_string(), animation);
        } else {
            self.no_animation_found(name);
        }
    }

    pub fn get_animation(&self, name: &str) -> GMAnimation {
        debug!("GMResources::get_animation(), name: '{}'", name);

        match self.animations.get(name) {
            Some(animation) => {
                animation.clone()
            }
            None => {
                self.no_animation_found(name);
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
    frames: Vec<(u32, f32)>, // (texture index, duration in seconds)
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
