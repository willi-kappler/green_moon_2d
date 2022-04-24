
use std::rc::Rc;

use sdl2::render::TextureCreator;
use sdl2::video::WindowContext;


use crate::texture::GMTexture;
use crate::font::GMFontT;
use crate::error::GMError;


pub struct GMResources {
    texture_creator: TextureCreator<WindowContext>,
    textures: Vec<Rc<GMTexture>>,
    fonts: Vec<Rc<dyn GMFontT>>,
}

impl GMResources {
    pub fn new(texture_creator: TextureCreator<WindowContext>) -> Self {
        Self {
            texture_creator,
            textures: Vec::new(),
            fonts: Vec::new(),
        }
    }

    pub fn load_resources(&mut self, file_name: &str) -> Result<(), GMError> {
        todo!();
    }

    pub fn clear_resources(&mut self) {
        self.textures.clear();
        self.fonts.clear();
    }

    // Textures:
    pub fn create_texture(&self, file_name: &str, cols: u32, unit_width: u32, unit_height: u32) -> Result<GMTexture, GMError> {
        todo!();
    }

    pub fn clear_textures(&mut self) {
        self.textures.clear();
    }

    fn get_texture_index(&self, name: &str) -> Option<usize> {
        todo!();
    }

    pub fn add_texture(&mut self, texture: GMTexture) -> Result<(), GMError> {
        todo!();

        // Ok(())
    }

    pub fn remove_texture(&mut self, name: &str) -> Result<(), GMError> {
        todo!();
    }

    pub fn replace_texture(&mut self, texture: GMTexture) -> Result<(), GMError> {
        todo!();
    }

    pub fn get_texture_clone(&self, name: &str) -> Result<Rc<GMTexture>, GMError> {
        todo!();
    }

    // Fonts:

    // Animations:

    // Sounds:

    // Musics:
}

struct GMResourceFormat {
    textures: Vec<GMTextureFormat>,
    fonts: Vec<GMFontFormat>,
}

struct GMTextureFormat {
    file_name: String,
    cols: u32,
    unit_width: u32,
    unit_height: u32,
}

struct GMFontFormat {
    texture_name: String,
    char_mapping: String,
}
