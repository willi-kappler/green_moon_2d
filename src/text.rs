

use std::rc::Rc;
use std::fmt::{self, Debug, Formatter};
//use std::any::Any;

use crate::context::GMContext;
use crate::draw_object::{GMDrawT, GMDrawRefType, GMDrawMutRefType};
use crate::error::GMError;
use crate::font::GMFontT;
use crate::movement::{GMMovementT, GMMovementInner};


#[derive(Clone)]
pub struct GMTextInner {
    pub font: Rc<dyn GMFontT>,
    pub text: String,
    pub movement_inner: GMMovementInner,
    pub spacing_x: f32,
    pub spacing_y: f32,
    pub horizontal: bool,
    pub active: bool,
    pub z_index: i32,
    pub group: u64,
    pub state: u64,
}

impl GMTextInner {
    pub fn new(font: Rc<dyn GMFontT>, text: String, movement_inner: GMMovementInner) -> Self {
        Self {
            font,
            text,
            movement_inner,
            spacing_x: 0.0,
            spacing_y: 0.0,
            horizontal: true,
            active: true,
            z_index: 0,
            group: 0,
            state: 0,
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
            .finish()
    }
}

#[derive(Debug, Clone)]
pub struct GMText {
    pub text_inner: GMTextInner,
    pub movements: Vec<Box<dyn GMMovementT>>,
    pub effects: Vec<Box<dyn GMTextEffectT>>,
}

impl GMText {
    pub fn new(font: Rc<dyn GMFontT>, text: &str, x: f32, y: f32) -> Self {
        let mut width: f32 = 0.0;
        let mut height: f32 = 0.0;

        for c in text.chars() {
            let (c_width, c_height) = font.get_char_dimensions(c);

            width += c_width;
            height = height.max(c_height);
        }

        let movement_inner = GMMovementInner::new(
            x,
            y,
            width,
            height,
        );

        let text_inner = GMTextInner::new(
            font.clone(),
            text.to_string(),
            movement_inner,
        );

        Self {
            text_inner,
            movements: Vec::new(),
            effects: vec![Box::new(GMTextEffectStatic::new())],
        }
    }

    pub fn update(&mut self, context: &mut GMContext) -> Result<(), GMError> {
        if self.text_inner.active {
            for movement in self.movements.iter_mut() {
                movement.update(&mut self.text_inner.movement_inner, context);
            }

            for effect in self.effects.iter_mut() {
                effect.update(&mut self.text_inner, context);
            }
        }

        Ok(())
    }

    pub fn draw(&self, context: &mut GMContext) {
        if self.text_inner.active {
            for effect in self.effects.iter() {
                effect.draw(&self.text_inner, context);
            }
        }
    }

    pub fn get_z_index(&self) -> i32 {
        self.text_inner.z_index
    }

    pub fn set_z_index(&mut self, z_index: i32) {
        self.text_inner.z_index = z_index;
    }

    pub fn get_movement_inner_ref(&self) -> &GMMovementInner {
        &self.text_inner.movement_inner
    }

    pub fn get_movement_inner_mut_ref(&mut self) -> &mut GMMovementInner {
        &mut self.text_inner.movement_inner
    }
}

#[derive(Debug, Clone)]
pub struct GMTextObject {
    pub text: GMText,
}

impl GMTextObject {
    pub fn new(text: GMText) -> Self {
        Self {
            text
        }
    }
}

impl GMDrawT for GMTextObject {
    fn update(&mut self, context: &mut GMContext) -> Result<(), GMError> {
        self.text.update(context)
    }

    fn draw(&self, context: &mut GMContext) {
        self.text.draw(context)
    }

    fn get_z_index(&self) -> i32 {
        self.text.get_z_index()
    }

    fn set_z_index(&mut self, z_index: i32) {
        self.text.set_z_index(z_index);
    }

    fn box_clone(&self) -> Box<dyn GMDrawT> {
        let result = self.clone();

        Box::new(result)
    }

    fn cast_ref(&self) -> GMDrawRefType {
        GMDrawRefType::Text(&self.text)
    }

    fn cast_mut_ref(&mut self) -> GMDrawMutRefType {
        GMDrawMutRefType::Text(&mut self.text)
    }
}

pub trait GMTextEffectT {
    fn update(&mut self, text_inner: &mut GMTextInner, context: &mut GMContext);

    fn draw(&self, text_inner: &GMTextInner, context: &mut GMContext);

    fn set_active(&mut self, active: bool);

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

impl GMTextEffectStatic {
    pub fn new() -> Self {
        Self {
            active: true,
        }
    }
}

impl GMTextEffectT for GMTextEffectStatic {
    fn update(&mut self, _text_inner: &mut GMTextInner, _context: &mut GMContext) {}

    fn draw(&self, text_inner: &GMTextInner, context: &mut GMContext) {
        if self.active {
            text_inner.draw(context);
        }
    }

    fn set_active(&mut self, active: bool) {
        self.active = active;
    }

    fn box_clone(&self) -> Box<dyn GMTextEffectT> {
        let result = self.clone();

        Box::new(result)
    }
}
