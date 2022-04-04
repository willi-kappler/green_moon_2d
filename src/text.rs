

use std::rc::Rc;

use crate::draw_object::GMDrawT;
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
        }
    }

    pub fn draw(&self) {
        let mut x = self.movement_inner.x;
        let mut y = self.movement_inner.y;

        for c in self.text.chars() {
            let (c_width, c_height) = self.font.get_char_dimensions(c);

            self.font.draw(c, x, y);

            if self.horizontal {
                x += c_width + self.spacing_x;
            } else {
                y += c_height + self.spacing_y;
            }
        }
    }
}

pub trait GMTextEffectT {
    fn update(&mut self, text_inner: &mut GMTextInner);
    fn draw(&self, text_inner: &GMTextInner);
    fn set_active(&mut self, active: bool);
    fn box_clone(&self) -> Box<dyn GMTextEffectT>;
}

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
}

impl GMDrawT for GMText {
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

    fn draw(&self) {
        if self.text_inner.active {
            for effect in self.effects.iter() {
                effect.draw(&self.text_inner);
            }
        }
    }

    fn get_z_index(&self) -> i32 {
        self.text_inner.z_index
    }

    fn box_clone(&self) -> Box<dyn GMDrawT> {
        let result = GMText {
            text_inner: self.text_inner.clone(),
            movements: self.movements.iter().map(|m| m.box_clone()).collect(),
            effects: self.effects.iter().map(|e| e.box_clone()).collect(),
        };

        Box::new(result)
    }
}

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
    fn update(&mut self, _text_inner: &mut GMTextInner) {}

    fn draw(&self, text_inner: &GMTextInner) {
        if self.active {
            text_inner.draw();
        }
    }

    fn set_active(&mut self, active: bool) {
        self.active = active;
    }

    fn box_clone(&self) -> Box<dyn GMTextEffectT> {
        let result = GMTextEffectStatic {
            active: self.active,
        };

        Box::new(result)
    }
}
