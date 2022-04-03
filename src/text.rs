

use std::rc::Rc;

use crate::font::GMFontT;
use crate::movement::{GMMovementT, GMMovementInner};


pub struct GMTextInner {
    pub font: Rc<dyn GMFontT>,
    pub text: String,
    pub movement_inner: GMMovementInner,
    pub spacing_x: f32,
    pub spacing_y: f32,
    pub horizontal: bool,
    pub active: bool,
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
        }
    }
}

pub trait GMTextT {
    fn update(&mut self);
    fn draw(&mut self);
    fn set_active(&mut self, active: bool);
}

pub trait GMTextEffectT {
    fn update(&mut self, text_inner: &mut GMTextInner);
    fn draw(&mut self, text_inner: &mut GMTextInner);
    fn set_active(&mut self, active: bool);
}

pub struct GMText {
    text_inner: GMTextInner,
    movements: Vec<Box<dyn GMMovementT>>,
    effects: Vec<Box<dyn GMTextEffectT>>,
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
            effects: Vec::new(),
        }
    }
}

impl GMTextT for GMText {
    fn update(&mut self) {
        if self.text_inner.active {
            for movement in self.movements.iter_mut() {
                movement.update(&mut self.text_inner.movement_inner);
            }

            for effect in self.effects.iter_mut() {
                effect.update(&mut self.text_inner);
            }
        }
    }

    fn draw(&mut self) {
        if self.text_inner.active {
            for effect in self.effects.iter_mut() {
                effect.draw(&mut self.text_inner);
            }
        }
    }

    fn set_active(&mut self, active: bool) {
        self.text_inner.active = active;
    }
}
