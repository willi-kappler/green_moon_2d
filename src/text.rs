

use std::rc::Rc;

use crate::font::GMBitmapFont;
use crate::movement::{GMMovementT, GMConstPos};


pub trait GMTextEffectT {
    fn update(&mut self);
    fn draw(&self, text: &GMTextInner);
}

pub struct GMTextInner {
    pub font: Rc<GMBitmapFont>,
    pub text: String,
    pub x: i32,
    pub y: i32,
    pub char_width: u32,
    pub char_height: u32,
    pub horizontal: bool,
}


pub struct GMText {
    pub inner: GMTextInner,
    movement: Box<dyn GMMovementT>,
    effect: Box<dyn GMTextEffectT>,
}

impl GMText {
    pub fn new(font: Rc<GMBitmapFont>, text: &str, x: i32, y: i32) -> Self {
        let (char_width, char_height) = font.get_char_dimensions();

        let inner = GMTextInner {
            font,
            text: text.to_string(),
            x,
            y,
            char_width,
            char_height,
            horizontal: true };
        Self {
            inner,
            movement: Box::new(GMConstPos::new()),
            effect: Box::new(GMTextEffectStatic::new()),
        }
    }

    pub fn update(&mut self) {
        let (new_x, new_y) = self.movement.update(self.inner.x, self.inner.y);
        self.inner.x = new_x;
        self.inner.y = new_y;
    }

    pub fn draw(&self) {
        self.effect.draw(&self.inner);
    }

    pub fn set_movement<M: 'static + GMMovementT>(&mut self, movement: M) {
        self.movement = Box::new(movement);
    }

    pub fn set_effect<E: 'static + GMTextEffectT>(&mut self, effect: E) {
        self.effect = Box::new(effect);
    }
}

pub struct GMTextEffectStatic {
}

impl GMTextEffectStatic {
    pub fn new() -> Self {
        Self {}
    }
}

impl GMTextEffectT for GMTextEffectStatic {
    fn update(&mut self) {
    }

    fn draw(&self, text: &GMTextInner) {
        let mut x = text.x;
        let mut y = text.y;

        for c in text.text.chars() {
            text.font.draw(c, x, y);

            if text.horizontal {
                x += text.char_width as i32;
            } else {
                y += text.char_height as i32;
            }
        }
    }
}
