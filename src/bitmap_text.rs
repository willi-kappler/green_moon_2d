

use std::collections::{HashMap};
use std::sync::Arc;
use std::fmt::Debug;

use log::debug;


use crate::texture::GMTexture;
use crate::util::{error_panic, GMAlign};
use crate::math::{GMVec2D, GMSize};

#[derive(Debug, Clone)]
pub struct GMBitmapFont {
    texture: Arc<GMTexture>,
    mapping: HashMap<char, u32>,
}

impl GMBitmapFont {
    pub fn new(texture: Arc<GMTexture>, char_mapping: &str) -> Self {
        debug!("GMBitmapFont::new(), char_mapping: '{}'", char_mapping);

        let mut mapping = HashMap::new();

        for (i, c) in char_mapping.chars().enumerate() {
            mapping.insert(c, i as u32);
        }

        Self {
            texture: texture,
            mapping,
        }
    }

    pub fn get_char_dimensions(&self) -> (f32, f32) {
        self.texture.get_unit_dimension()
    }

    pub fn get_index(&self, c: char) -> u32 {
        match self.mapping.get(&c) {
            Some(index) => {
                *index
            }
            None => {
                error_panic(&format!("GMBitmapFont::draw_opt(), Character '{}' not in map.", c));
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct GMBitmapText { // TODO: Remove struct and use components instead ?
    pub font: Arc<GMBitmapFont>,
    pub text: String,
    pub spacing: GMVec2D,
    pub horizontal: bool,
    pub align: GMAlign,
    pub size: GMSize,
    pub chars: Vec<u8>,
}

