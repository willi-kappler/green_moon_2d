

use std::collections::{HashMap};
use std::sync::Arc;
use std::fmt::Debug;

use log::debug;

use crate::texture::GMTexture;
use crate::util::{error_panic, GMAlign, GMDrawT, GMUpdateT, GMVisibleT, GMActiveT};
use crate::math::{GMVec2D, GMSize};
use crate::context::GMContext;
use crate::movement::{GMPositionT, GMRotationT, GMScaleT};

use crate::{gen_effect_trait, gen_effect_impl_for_type, gen_type_effect_methods};

#[derive(Debug, Clone)]
pub struct GMBitmapFont {
    texture: Arc<GMTexture>,
    mapping: HashMap<char, u32>,
}

impl GMBitmapFont {
    pub fn new(texture: Arc<GMTexture>, char_mapping: &str) -> Self {
        debug!("GMBitmapFont::new(), char_mapping: '{}'", char_mapping);
        // Maybe split texture into smaller char sized textures...

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

    pub fn set_texture(&mut self, texture: Arc<GMTexture>) {
        self.texture = texture;
    }

    pub fn get_texture(&self) -> &Arc<GMTexture> {
        &self.texture
    }

    pub fn set_mapping(&mut self, mapping: HashMap<char, u32>) {
        self.mapping = mapping;
    }

    pub fn get_mapping(&self) -> &HashMap<char, u32> {
        &self.mapping
    }
}

#[derive(Debug, Clone)]
pub struct GMBitmapChar {
    index: u32,
    relative_position: GMVec2D,
    rotation: f32,
    scale: f32,
    flip_x: bool,
    flip_y: bool,
    visible: bool,
    // TODO: alpha value
}

impl GMBitmapChar {
    pub fn new(index: u32, position: GMVec2D) -> Self {
        Self {
            index,
            relative_position: position,
            rotation: 0.0,
            scale: 1.0,
            flip_x: false,
            flip_y: false,
            visible: true,
        }
    }

    pub fn set_index(&mut self, index: u32) {
        self.index = index;
    }

    pub fn get_index(&self) -> u32 {
        self.index
    }

    pub fn set_flip_x(&mut self, flip_x: bool) {
        self.flip_x = flip_x;
    }

    pub fn get_flip_x(&mut self) -> bool {
        self.flip_x
    }

    pub fn set_flip_y(&mut self, flip_y: bool) {
        self.flip_y = flip_y;
    }

    pub fn get_flip_y(&mut self) -> bool {
        self.flip_y
    }
}

impl GMPositionT for GMBitmapChar {
    fn get_position(&self) -> GMVec2D {
        self.relative_position
    }

    fn get_position_mut(&mut self) -> &mut GMVec2D {
        &mut self.relative_position
    }
}

impl GMRotationT for GMBitmapChar {
    fn get_rotation(&self) -> f32 {
        self.rotation
    }

    fn get_rotation_mut(&mut self) -> &mut f32 {
        &mut self.rotation
    }
}

impl GMScaleT for GMBitmapChar {
    fn get_scale(&self) -> f32 {
        self.scale
    }

    fn get_scale_mut(&mut self) -> &mut f32 {
        &mut self.scale
    }
}

impl GMVisibleT for GMBitmapChar {
    fn set_visible(&mut self, visible: bool) {
        self.visible = visible;
    }

    fn get_visible(&self) -> bool {
        self.visible
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
    chars: Vec<GMBitmapChar>,
    draw_text_first: bool,
    visible: bool,
}

impl GMBitmapTextBase {
    pub fn new<T: Into<GMVec2D>, S: Into<String>>(font: Arc<GMBitmapFont>, position: T, text: S) -> Self {
        let mut text = Self {
            font,
            position: position.into(),
            text: text.into(),
            spacing: GMVec2D::new(0.0, 0.0),
            horizontal: true,
            align: GMAlign::BottomLeft,
            size: GMSize::new(0.0, 0.0),
            chars: Vec::new(),
            draw_text_first: true,
            visible: true,
        };

        text.reset_chars();

        text
    }

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

    pub fn get_chars(&self) -> &Vec<GMBitmapChar> {
        &self.chars
    }

    pub fn get_chars_mut(&mut self) -> &mut Vec<GMBitmapChar> {
        &mut self.chars
    }

    pub fn set_draw_text_first(&mut self, draw_text_first: bool) {
        self.draw_text_first = draw_text_first;
    }

    pub fn get_draw_text_first(&self) -> bool {
        self.draw_text_first
    }

    pub fn reset_chars(&mut self) {
        // Remove all the characters and recreate them
        self.chars.clear();

        for c in self.text.chars() {
            let index = self.font.get_index(c);
            let position = GMVec2D::new(0.0, 0.0);
            let bitmap_char = GMBitmapChar::new(index, position);
            self.chars.push(bitmap_char);
        }

        self.reset_positions();
    }

    pub fn reset_positions(&mut self) {
        // Keep characters, just change position
        let (dx, dy) = self.font.get_char_dimensions();
        let num_of_chars = self.chars.len() as f32;
        let mut x = 0.0;
        let mut y = 0.0;
        let mut dx2 = dx + self.spacing.x;
        let mut dy2 = dy + self.spacing.y;

        if self.horizontal {
            self.size.width = (dx * num_of_chars) + (self.spacing.x * (num_of_chars - 1.0));
            self.size.height = dy;
            dy2 = 0.0;
        } else {
            self.size.width = dx;
            self.size.height = (dy * num_of_chars) + (self.spacing.y * (num_of_chars - 1.0));
            dx2 = 0.0;
        }

        match self.align {
            GMAlign::TopLeft => {
            }
            GMAlign::TopCenter => {
                x = -(self.size.width / 2.0);
            }
            GMAlign::TopRight => {
                x = -self.size.width;
            }
            GMAlign::MiddleLeft => {
                y = -(self.size.height / 2.0);
            }
            GMAlign::MiddleCenter => {
                x = -(self.size.width / 2.0);
                y = -(self.size.height / 2.0);
            }
            GMAlign::MiddleRight => {
                x = -self.size.width;
                y = -(self.size.height / 2.0);
            }
            GMAlign::BottomLeft => {
                y = -self.size.height;
            }
            GMAlign::BottomCenter => {
                x = -(self.size.width / 2.0);
                y = -self.size.height;
            }
            GMAlign::BottomRight => {
                x = -self.size.width;
                y = -self.size.height;
            }
        }

        for c in self.chars.iter_mut() {
            c.relative_position.x = x;
            c.relative_position.y = y;
            c.rotation = 0.0;

            x += dx2;
            y += dy2;
        }
    }
}

impl GMDrawT for GMBitmapTextBase {
    fn draw(&self, context: &mut GMContext) {
        if self.visible {
            for c in self.chars.iter() {
                if c.visible {
                    let dx = self.position.x + c.relative_position.x;
                    let dy = self.position.y + c.relative_position.y;
                    self.font.texture.draw_opt(dx, dy, c.index, c.rotation, c.scale, c.flip_x, c.flip_y, context);
                }
            }
        }
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

impl GMVisibleT for GMBitmapTextBase {
    fn set_visible(&mut self, visible: bool) {
        self.visible = visible;
    }

    fn get_visible(&self) -> bool {
        self.visible
    }
}

pub struct GMBitmapText {
    base: GMBitmapTextBase,
    effects: Vec<Box<dyn GMBitmapTextEffectT>>,
    active: bool,
    visible: bool,
}

impl GMBitmapText {
    pub fn new<T: Into<GMVec2D>, S: Into<String>>(font: Arc<GMBitmapFont>, position: T, text: S) -> Self {
        let base = GMBitmapTextBase::new(font, position, text);

        Self {
            base,
            effects: Vec::new(),
            active: true,
            visible: true,
        }
    }

    gen_type_effect_methods!(GMBitmapTextBase, GMBitmapTextEffectT);

    /*
    pub fn get_base(&self) -> &GMBitmapTextBase {
        &self.base
    }

    pub fn get_base_mut(&mut self) -> &mut GMBitmapTextBase {
        &mut self.base
    }

    pub fn add_effect<T: 'static + GMBitmapTextEffectT>(&mut self, effect: T) {
        debug!("::add_effect()");
        self.effects.push(Box::new(effect));
    }

    pub fn add_effect2(&mut self, effect: Box<dyn GMBitmapTextEffectT>) {
        debug!("::add_effect2()");
        self.effects.push(effect);
    }

    pub fn remove_effect(&mut self, index: usize) {
        debug!("::remove_effect(), index: {}", index);
        self.effects.remove(index);
    }

    pub fn set_effects(&mut self, effects: Vec<Box<dyn GMBitmapTextEffectT>>) {
        debug!("::set_effects()");
        self.effects = effects;
    }

    pub fn replace_effect(&mut self, index: usize, effect: Box<dyn GMBitmapTextEffectT>) {
        debug!(":replace_effect(), index: {}", index);
        self.effects[index] = effect;
    }

    pub fn clear_effects(&mut self) {
        debug!("GMEffectManager::clear_effects()");
        self.effects.clear();
    }

    pub fn swap_effects(&mut self, index1: usize, index2: usize) {
        debug!("GMEffectManager::swap_effect(), index1: {}, index2: {}", index1, index2);
        self.effects.swap(index1, index2);
    }

    pub fn send_effect_message(&mut self, index: usize, message: &str) {
        self.effects[index].send_message(message);
    }

    pub fn send_effect_message_all(&mut self, message: &str) {
        for effect in self.effects.iter_mut() {
            effect.send_message(message);
        }
    }

    pub fn get_effects(&self) -> &Vec<Box<dyn GMBitmapTextEffectT>> {
        &self.effects
    }

    pub fn get_effect(&self, index: usize) -> &Box<dyn GMBitmapTextEffectT> {
        &self.effects[index]
    }

    pub fn get_effects_mut(&mut self) -> &mut Vec<Box<dyn GMBitmapTextEffectT>> {
        &mut self.effects
    }

    pub fn get_effect_mut(&mut self, index: usize) -> &mut Box<dyn GMBitmapTextEffectT> {
        &mut self.effects[index]
    }
    */
}

gen_effect_impl_for_type!(GMBitmapText);

/*
impl GMUpdateT for GMBitmapText {
    fn update(&mut self, context: &mut GMContext) {
        if self.active {
            for effect in self.effects.iter_mut() {
                effect.update(&mut self.base, context);
            }
        }
    }
}

impl GMDrawT for GMBitmapText {
    fn draw(&self, context: &mut GMContext) {
        if self.visible {
            if self.base.draw_text_first {
                self.base.draw(context);

                for effect in self.effects.iter() {
                    effect.draw(&self.base, context);
                }
            } else {
                for effect in self.effects.iter() {
                    effect.draw(&self.base, context);
                }

                self.base.draw(context);
            }
        }
    }
}

impl GMActiveT  for GMBitmapText {
    fn set_active(&mut self, active: bool) {
        self.active = active;
    }

    fn get_active(&self) -> bool {
        self.active
    }
}

impl GMVisibleT for GMBitmapText {
    fn set_visible(&mut self, visible: bool) {
        self.visible = visible;
    }

    fn get_visible(&self) -> bool {
        self.visible
    }
}
*/

gen_effect_trait!(GMBitmapTextEffectT, GMBitmapTextBase);

/*
pub trait GMBitmapTextEffectT {
    fn update(&mut self, _text_base: &mut GMBitmapTextBase, _context: &mut GMContext) {
    }

    fn draw(&self, _text_base: &GMBitmapTextBase, _context: &mut GMContext) {
    }

    fn send_message(&mut self, _message: &str) {
    }
}
*/

