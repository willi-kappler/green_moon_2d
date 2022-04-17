

use std::rc::Rc;
use std::fmt::{self, Debug, Formatter};
use std::f32::consts::TAU;

use crate::context::GMContext;
use crate::draw_object::GMDrawT;
use crate::error::GMError;
use crate::font::{GMFontT, GMBitmapFont};
use crate::movement::{GMMovementT, GMMovementInner};


#[derive(Clone)]
pub struct GMTextInner {
    pub font: Rc<dyn GMFontT>,
    pub text: String,
    pub movement_inner: GMMovementInner,
    pub movements: Vec<Box<dyn GMMovementT>>,
    pub spacing_x: f32,
    pub spacing_y: f32,
    pub horizontal: bool,
    pub active: bool,
    pub z_index: i32,
    pub name: String,
}

impl Default for GMTextInner {
    fn default() -> Self {
        Self {
            font: Rc::new(GMBitmapFont::default()),
            text: "".to_string(),
            movement_inner: Default::default(),
            movements: Vec::new(),
            spacing_x: 0.0,
            spacing_y: 0.0,
            horizontal: true,
            active: true,
            z_index: 0,
            name: "".to_string(),
        }
    }
}

impl GMTextInner {
    pub fn new(font: Rc<dyn GMFontT>, text: &str, x: f32, y: f32) -> Self {
        let mut width: f32 = 0.0;
        let mut height: f32 = 0.0;

        for c in text.chars() {
            let (c_width, c_height) = font.get_char_dimensions(c);

            width += c_width;
            height = height.max(c_height);
        }

        let movement_inner = GMMovementInner {
            x, y, width, height, ..Default::default()
        };

        Self {
            font: font.clone(),
            text: text.to_string(),
            movement_inner,
            .. Default::default()
        }
    }

    pub fn draw(&self, context: &mut GMContext) {
        let mut x = self.movement_inner.x;
        let mut y = self.movement_inner.y;

        for c in self.text.chars() {
            let (c_width, c_height) = self.font.get_char_dimensions(c);

            self.font.draw(c, x, y, context);

            if self.horizontal {
                x += c_width + self.spacing_x;
            } else {
                y += c_height + self.spacing_y;
            }
        }
    }

    pub fn update(&mut self, context: &mut GMContext) {
        if self.active {
            for movement in self.movements.iter_mut() {
                movement.update(&mut self.movement_inner, context);
            }
        }
    }
}

impl Debug for GMTextInner {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("GMTextInner")
            .field("text", &self.text)
            .field("movement_inner", &self.movement_inner)
            .field("spacing_x", &self.spacing_x)
            .field("spacing_y", &self.spacing_y)
            .field("horizontal", &self.horizontal)
            .field("active", &self.active)
            .field("z_index", &self.z_index)
            .field("name", &self.name)
            .finish()
    }
}

#[derive(Debug, Clone)]
pub struct GMText {
    pub text_inner: GMTextInner,
    pub effects: Vec<Box<dyn GMTextEffectT>>,
}

impl Default for GMText {
    fn default() -> Self {
        Self {
            text_inner: Default::default(),
            effects: Vec::new(),
        }
    }
}

impl GMText {
    pub fn new(font: Rc<dyn GMFontT>, text: &str, x: f32, y: f32) -> Self {
        let text_inner = GMTextInner::new(font, text, x, y);

        Self {
            text_inner,
            effects: vec![Box::new(GMTextEffectStatic::default())],
        }
    }
}

impl GMDrawT for GMText {
    fn update(&mut self, context: &mut GMContext) -> Result<(), GMError> {
        self.text_inner.update(context);
        if self.text_inner.active {

            for effect in self.effects.iter_mut() {
                effect.update(&mut self.text_inner, context);
            }
        }

        Ok(())
    }

    fn draw(&self, context: &mut GMContext) {
        if self.text_inner.active {
            for effect in self.effects.iter() {
                effect.draw(&self.text_inner, context);
            }
        }
    }

    fn get_z_index(&self) -> i32 {
        self.text_inner.z_index
    }

    fn get_name(&self) -> &str {
        &self.text_inner.name
    }

    fn get_groups(&self) -> &[&str] {
        todo!();
    }

    fn box_clone(&self) -> Box<dyn GMDrawT> {
        let result = self.clone();

        Box::new(result)
    }
}

pub trait GMTextEffectT {
    fn update(&mut self, _text_inner: &mut GMTextInner, _context: &mut GMContext) {}

    fn draw(&self, _text_inner: &GMTextInner, _context: &mut GMContext) {}

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
    fn draw(&self, text_inner: &GMTextInner, context: &mut GMContext) {
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
    fn update(&mut self, _text_inner: &mut GMTextInner, _context: &mut GMContext) {
        if self.active {
            self.time += 0.01;
            if self.time > TAU {
                self.time -= TAU;
            }
        }
    }

    fn draw(&self, text_inner: &GMTextInner, context: &mut GMContext) {
        if self.active {
            let mut x = text_inner.movement_inner.x;
            let mut y = text_inner.movement_inner.y;
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
