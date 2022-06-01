


use std::collections::HashMap;
use std::rc::Rc;
use std::fmt::Debug;

use log::debug;

use crate::texture::{GMTexture, GMTextureConfig, GMTextureConfigOptional};
use crate::context::GMContext;
use crate::math::GMVec2D;
use crate::object::GMObjectT;
use crate::message::{GMObjectMessage, GMObjectReply};

#[derive(Debug, Clone)]
pub struct GMBitmapFont {
    texture: Rc<GMTexture>,
    mapping: HashMap<char, u32>,
}

impl GMBitmapFont {
    pub fn new(texture: Rc<GMTexture>, char_mapping: &str) -> Self {
        let mut mapping = HashMap::new();

        for (i, c) in char_mapping.chars().enumerate() {
            mapping.insert(c, i as u32);
        }

        Self {
            texture,
            mapping,
        }
    }

    pub fn get_char_dimensions(&self) -> (f32, f32) {
        self.texture.get_unit_dimension()
    }

    pub fn draw(&self, c: char, x: f32, y: f32, context: &mut GMContext) {
        let options = GMTextureConfigOptional::default();
        self.draw_opt(c, x, y, options, context)
    }

    pub fn draw_opt(&self, c: char, x: f32, y: f32, options: GMTextureConfigOptional, context: &mut GMContext) {
        match self.mapping.get(&c) {
            Some(index) => {
                context.draw_texture(GMTextureConfig::new_opt(self.texture.clone(), x, y, *index, options));
            }
            None => {
                panic!("GMBitmapFont::draw_opt(), Character '{}' not in map.", c);
            }
        }
    }
}


#[derive(Debug, Clone)]
pub struct GMBitmapText {
    font: Rc<GMBitmapFont>,
    text: String,
    position: GMVec2D,
    spacing_x: f32,
    spacing_y: f32,
    horizontal: bool,
}


impl GMBitmapText {
    pub fn new(font: Rc<GMBitmapFont>, text: String, x: f32, y: f32) -> Self {
        Self {
            font,
            text: text.to_string(),
            position: GMVec2D::new(x, y),
            spacing_x: 0.0,
            spacing_y: 0.0,
            horizontal: true,
        }
    }

    pub fn set_font(&mut self, font: Rc<GMBitmapFont>) {
        self.font = font;
    }

    pub fn set_text(&mut self, text: &str) {
        self.text = text.to_string();
    }

    pub fn set_position(&mut self, position: GMVec2D) {
        self.position = position;
    }

    pub fn set_spacing_x(&mut self, spacing_x: f32) {
        self.spacing_x = spacing_x;
    }

    pub fn set_spacing_y(&mut self, spacing_y: f32) {
        self.spacing_y = spacing_y;
    }

    pub fn set_horizontal(&mut self, horizontal: bool) {
        self.horizontal = horizontal
    }
}

impl GMObjectT for GMBitmapText {
    fn send_message(&mut self, message: GMObjectMessage, context: &mut GMContext) -> GMObjectReply {
        use GMObjectMessage::*;

        match message {
            Update => {
                let mut x = self.position.x;
                let mut y = self.position.y;

                let (c_width, c_height) = self.font.get_char_dimensions();

                for c in self.text.chars() {
                    self.font.draw(c, x, y, context);

                    if self.horizontal {
                        x += c_width + self.spacing_x;
                    } else {
                        y += c_height + self.spacing_y;
                    }
                }
            }
            SetFont(font) => {
                self.set_font(font);
            }
            SetPosition(position) => {
                self.set_position(position);
            }
            SetSpacingX(spacing_x) => {
                self.set_spacing_x(spacing_x);
            }
            SetSpacingY(spacing_y) => {
                self.set_spacing_y(spacing_y);
            }
            SetHorizontal(horizontal) => {
                self.set_horizontal(horizontal);
            }
            GetPosition => {
                return GMObjectReply::Position(self.position)
            }
            _ => {
                debug!("GMBitmapText::send_message(), unhandled message: {:?}", message);
            }
        }

        crate::message::GMObjectReply::Empty
    }
}


// use std::rc::Rc;
// use std::fmt::{self, Debug, Formatter};
// use std::f32::consts::TAU;

// use crate::context::{GMUpdateContext, GMDrawContext};
// use crate::error::GMError;
// use crate::font::GMFontT;
// use crate::math::GMVec2D;




/*
#[derive(Clone)]
pub struct GMTextCommon {
    pub font: Rc<dyn GMFontT>,
    pub text: String,
    pub spacing_x: f32,
    pub spacing_y: f32,
    pub horizontal: bool,
    pub char_positions: Vec<GMVec2D>,
    pub draw_object_common: GMDrawObjectCommon,
}

impl GMTextCommon {
    pub fn new(font: Rc<dyn GMFontT>, text: &str, name: &str, x: f32, y: f32) -> Self {
        let mut width: f32 = 0.0;
        let mut height: f32 = 0.0;

        for c in text.chars() {
            let (c_width, c_height) = font.get_char_dimensions(c);

            width += c_width;
            height = height.max(c_height);

            // TODO: set up char positions
        }

        Self {
            font: font.clone(),
            text: text.to_string(),
            spacing_x: 0.0,
            spacing_y: 0.0,
            horizontal: true,
            char_positions: Vec::new(),
            draw_object_common: GMDrawObjectCommon::new(name, x, y, width, height),
        }
    }

    pub fn update(&mut self) {
        self.draw_object_common.update();

        // TODO: update char positions
    }

    pub fn draw(&self, _context: &mut GMDrawContext) {
        if self.draw_object_common.active {
            //let mut x = self.draw_object_common.movement_common.x;
            //let mut y = self.draw_object_common.movement_common.y;

            // Use char positions...

            for c in self.text.chars() {
                let (_c_width, _c_height) = self.font.get_char_dimensions(c);

                // self.font.draw(c, x, y, context);

                if self.horizontal {
                    // x += c_width + self.spacing_x;
                } else {
                    // y += c_height + self.spacing_y;
                }
            }
        }
    }
}


impl Debug for GMTextCommon {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("GMTextCommon")
            .field("font", &self.font.get_name())
            .field("text", &self.text)
            .field("spacing_x", &self.spacing_x)
            .field("spacing_y", &self.spacing_y)
            .field("horizontal", &self.horizontal)
            .field("draw_object_common", &self.draw_object_common)
            .finish()
    }
}



#[derive(Debug, Clone)]
pub struct GMText {
    pub text_common: GMTextCommon,
}

impl GMText {
    pub fn new(font: Rc<dyn GMFontT>, text: &str, name: &str, x: f32, y: f32) -> Self {
        let text_common = GMTextCommon::new(font, text, name, x, y);

        Self {
            text_common,
        }
    }
}

impl GMDrawObjectT for GMText {
    fn update(&mut self, _context: &mut GMUpdateContext) -> Result<(), GMError> {
        self.text_common.update();

        Ok(())
    }

    fn draw(&self, context: &mut GMDrawContext) -> Result<(), GMError> {
        self.text_common.draw(context);

        Ok(())
    }

    fn get_common_ref(&self) -> &GMDrawObjectCommon {
        &self.text_common.draw_object_common
    }

    fn get_common_mut_ref(&mut self) -> &mut GMDrawObjectCommon {
        &mut self.text_common.draw_object_common
    }

    fn box_clone(&self) -> Box<dyn GMDrawObjectT> {
        let result = self.clone();

        Box::new(result)
    }
}

/*
pub trait GMTextEffectT {
    fn update(&mut self, _text_inner: &mut GMTextCommon, _context: &mut GMUpdateContext);

    fn draw(&self, _text_inner: &GMTextCommon, _context: &mut GMDrawContext);

    fn box_clone(&self) -> Box<dyn GMTextEffectT>;
}

impl Clone for Box<dyn GMTextEffectT> {
    fn clone(&self) -> Self {
        self.box_clone()
    }
}

impl Debug for Box<dyn GMTextEffectT> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "GMTextEffectT")
    }
}

#[derive(Clone, Debug)]
pub struct GMTextEffectStatic {
    active: bool,
}

impl Default for GMTextEffectStatic {
    fn default() -> Self {
        Self { active: true }
    }
}

impl GMTextEffectT for GMTextEffectStatic {
    fn update(&mut self, _text_inner: &mut GMTextCommon, _context: &mut GMUpdateContext) {
    }

    fn draw(&self, text_inner: &GMTextCommon, context: &mut GMDrawContext) {
        if self.active {
            text_inner.draw(context);
        }
    }

    fn box_clone(&self) -> Box<dyn GMTextEffectT> {
        let result = self.clone();

        Box::new(result)
    }
}

#[derive(Clone, Debug)]
pub struct GMTextEffectWave {
    pub active: bool,
    pub amplitude: f32,
    pub step: f32,
    pub frequency: f32,
    pub time: f32,
}

impl Default for GMTextEffectWave {
    fn default() -> Self {
        Self {
            active: true,
            amplitude: 10.0,
            step: 1.0,
            frequency: 10.0,
            time: 0.0
        }
    }
}

impl GMTextEffectT for GMTextEffectWave {
    fn update(&mut self, _text_inner: &mut GMTextCommon, _context: &mut GMUpdateContext) {
        if self.active {
            self.time += 0.01;
            if self.time > TAU {
                self.time -= TAU;
            }
        }
    }

    fn draw(&self, text_inner: &GMTextCommon, context: &mut GMDrawContext) {
        if self.active {
            let mut x = text_inner.draw_object_common.movement_common.x;
            let mut y = text_inner.draw_object_common.movement_common.y;
            let mut offset = 0.0;

            for c in text_inner.text.chars() {
                let (c_width, c_height) = text_inner.font.get_char_dimensions(c);
                let angle = offset + (self.frequency * self.time);
                let delta = self.amplitude * angle.sin();

                if text_inner.horizontal {
                    text_inner.font.draw(c, x, y + delta, context);
                    x += c_width + text_inner.spacing_x;
                } else {
                    text_inner.font.draw(c, x + delta, y, context);
                    y += c_height + text_inner.spacing_y;
                }

                offset += self.step;
            }
        }
    }

    fn box_clone(&self) -> Box<dyn GMTextEffectT> {
        let result = self.clone();

        Box::new(result)
    }
}
*/

*/

