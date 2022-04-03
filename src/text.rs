

use std::rc::Rc;

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

    pub fn draw(&mut self) {
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

pub trait GMTextT {
    fn update(&mut self);
    fn draw(&mut self);
    fn set_active(&mut self, active: bool);
    fn get_inner(&self) -> &GMTextInner;
    fn get_inner_mut(&mut self) -> &mut GMTextInner;
    fn get_movements(&self) -> &[Box<dyn GMMovementT>];
    fn get_movements_mut(&mut self) -> &mut [Box<dyn GMMovementT>];
    fn get_effects(&self) -> &[Box<dyn GMTextEffectT>];
    fn get_effects_mut(&mut self) -> &mut [Box<dyn GMTextEffectT>];
    fn box_clone(&self) -> Box<dyn GMTextT>;
}

pub trait GMTextEffectT {
    fn update(&mut self, text_inner: &mut GMTextInner);
    fn draw(&mut self, text_inner: &mut GMTextInner);
    fn set_active(&mut self, active: bool);
    fn box_clone(&self) -> Box<dyn GMTextEffectT>;
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
            effects: vec![Box::new(GMTextEffectStatic::new())],
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

    fn get_inner(&self) -> &GMTextInner {
        &self.text_inner
    }

    fn get_inner_mut(&mut self) -> &mut GMTextInner {
        &mut self.text_inner
    }

    fn get_movements(&self) -> &[Box<dyn GMMovementT>] {
        todo!()
    }

    fn get_movements_mut(&mut self) -> &mut [Box<dyn GMMovementT>] {
        todo!()
    }

    fn get_effects(&self) -> &[Box<dyn GMTextEffectT>] {
        todo!()
    }

    fn get_effects_mut(&mut self) -> &mut [Box<dyn GMTextEffectT>] {
        todo!()
    }

    fn box_clone(&self) -> Box<dyn GMTextT> {
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

    fn draw(&mut self, text_inner: &mut GMTextInner) {
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
