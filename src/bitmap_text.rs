

use std::collections::{HashMap};
use std::sync::Arc;
use std::fmt::Debug;

use log::debug;

use crate::texture::GMTexture;
use crate::util::{error_panic, GMAlign};
use crate::math::{GMVec2D, GMSize};
use crate::context::GMContext;
use crate::movement::{GMPositionT, GMRotationT, GMScaleT};

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
pub struct GMChar {
    index: u32,
    relative_position: GMVec2D,
    rotation: f32,
    scale: f32,
}

impl GMChar {
    pub fn get_index(&self) -> u32 {
        self.index
    }

    pub fn set_index(&mut self, index: u32) { 
        self.index = index;
    }
}

impl GMPositionT for GMChar {
    fn get_position(&self) -> GMVec2D {
        self.relative_position
    }

    fn get_position_mut(&mut self) -> &mut GMVec2D {
        &mut self.relative_position
    }
}

impl GMRotationT for GMChar {
    fn get_rotation(&self) -> f32 {
        self.rotation
    }

    fn get_rotation_mut(&mut self) -> &mut f32 {
        &mut self.rotation
    }
}

impl GMScaleT for GMChar {
    fn get_scale(&self) -> f32 {
        self.scale
    }

    fn get_scale_mut(&mut self) -> &mut f32 {
        &mut self.scale
    }
}


#[derive(Debug, Clone)]
pub struct GMBitmapTextBase {
    font: Arc<GMBitmapFont>,
    position: GMVec2D,
    text: String,
    spacing: GMVec2D,
    horizontal: bool,
    align: GMAlign,
    size: GMSize,
    chars: Vec<GMChar>,
}

impl GMBitmapTextBase {
    pub fn set_font(&mut self, font: Arc<GMBitmapFont>) {
        self.font = font;
    }

    pub fn get_font(&self) -> Arc<GMBitmapFont> {
        self.font.clone()
    }

    pub fn set_text<T: Into<String>>(&mut self, text: T) {
        self.text = text.into();
    }

    pub fn get_text(&self) -> &str {
        &self.text
    }

    pub fn set_spacing<T: Into<GMVec2D>>(&mut self, spacing: T) {
        self.spacing = spacing.into();
    }

    pub fn get_spacing(&self) -> GMVec2D{
        self.spacing
    }

    pub fn set_horizontal(&mut self, horizontal: bool) {
        self.horizontal = horizontal;
    }

    pub fn get_horizontal(&self) -> bool {
        self.horizontal
    }

    pub fn set_align(&mut self, align: GMAlign) {
        self.align = align;
    }

    pub fn get_align(&self) -> GMAlign {
        self.align
    }

    pub fn get_size(&self) -> GMSize {
        self.size
    }

    pub fn get_chars(&self) -> &Vec<GMChar> {
        &self.chars
    }

    pub fn get_chars_mut(&mut self) -> &mut Vec<GMChar> {
        &mut self.chars
    }

    pub fn reset_chars(&mut self) {

    }

    pub fn reset_positions(&mut self) {

    }

    pub fn draw(&self, context: &mut GMContext) {
        
    }
}

impl GMPositionT for GMBitmapTextBase {
    fn get_position(&self) -> GMVec2D {
        self.position
    }

    fn get_position_mut(&mut self) -> &mut GMVec2D {
        &mut self.position
    }
}

pub struct GMBitmapText {
    base: GMBitmapTextBase,
    effects: Vec<Box<dyn GMBitmapTextEffectT>>,
}

impl GMBitmapText {
    pub fn new() {

    }

    pub fn get_base(&self) -> &GMBitmapTextBase {
        &self.base
    }

    pub fn get_base_mut(&mut self) -> &mut GMBitmapTextBase {
        &mut self.base
    }

    pub fn add_effect(&mut self) {

    }

    pub fn add_effect2(&mut self) {

    }

    pub fn update(&mut self, context: &mut GMContext) {
        for effect in self.effects.iter_mut() {
            effect.update(&mut self.base, context);
        }
    }
}

pub trait GMBitmapTextEffectT {
    fn update(&mut self, _text_base: &mut GMBitmapTextBase, _context: &mut GMContext) {
    }

    fn draw(&self, _context: &mut GMContext) {
    }
}
