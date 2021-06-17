
use crate::font::GMFontT;
use crate::value::GMValue;
use crate::text::{GMTextT};
use crate::sprite::in_rect;

use macroquad::input::{is_key_pressed, KeyCode, mouse_position, is_mouse_button_pressed, MouseButton};

use std::any::Any;
use std::rc::Rc;

pub enum GMMenuItemEvent {
    GMSelectThisItem,
    GMHighlightPrevItem,
    GMHighlightThisItem,
    GMHighlightNextItem,
    GMNewValue(GMValue),
}

pub trait GMMenuItemT {
    fn set_text(&mut self, text: &str);
    fn draw(&self);
    fn update(&mut self);
    fn set_active(&mut self, active: bool);
    fn get_active(&self) -> bool;
    fn event(&mut self) -> Option<GMMenuItemEvent>;
    fn set_font(&mut self, font: &Rc<dyn GMFontT>);
    fn set_property(&mut self, name: &str, value: &Rc<dyn Any>);
}

pub struct GMMenuItemStatic {
    inactive_text: Box<dyn GMTextT>,
    active_text: Box<dyn GMTextT>,
    active: bool,
}

impl GMMenuItemStatic {
    pub fn new(inactive_text: Box<dyn GMTextT>, active_text: Box<dyn GMTextT>) -> Self {
        Self {
            inactive_text,
            active_text,
            active: false,
        }
    }
    pub fn new_box(inactive_text: Box<dyn GMTextT>, active_text: Box<dyn GMTextT>) -> Box<dyn GMMenuItemT> {
        Box::new(Self::new(inactive_text, active_text))
    }
    pub fn get_x(&self) -> f32 {
        self.inactive_text.get_x()
    }
    pub fn get_y(&self) -> f32 {
        self.inactive_text.get_y()
    }
    pub fn get_extend(&self) -> (f32, f32) {
        self.inactive_text.get_extend()
    }
    fn point_inside(&self, x: f32, y: f32) -> bool {
        let x1 = self.get_x();
        let y1 = self.get_y();
        let (w, h) = self.get_extend();
        let x2 = x1 + w;
        let y2 = y1 + h;

        in_rect(x1, x2, y1, y2, x, y)
    }
}

impl GMMenuItemT for GMMenuItemStatic {
    fn set_text(&mut self, text: &str) {
        self.inactive_text.set_text(text);
        self.active_text.set_text(text);
    }
    fn draw(&self) {
        if self.active {
            self.active_text.draw();
        } else {
            self.inactive_text.draw();
        }
    }
    fn update(&mut self) {
        if self.active {
            self.active_text.update();
        } else {
            self.inactive_text.update();
        }
    }
    fn set_active(&mut self, active: bool) {
        self.active = active;
    }
    fn get_active(&self) -> bool {
        self.active
    }
    fn event(&mut self) -> Option<GMMenuItemEvent> {
        use GMMenuItemEvent::*;

        let (mousex, mousey) = mouse_position();
        let point_inside = self.point_inside(mousex, mousey);

        if self.active {
            if is_key_pressed(KeyCode::Up) {
                self.active = false;
                Some(GMHighlightPrevItem)
            } else if is_key_pressed(KeyCode::Down) {
                self.active = false;
                Some(GMHighlightNextItem)
            } else if is_key_pressed(KeyCode::Enter) || (is_mouse_button_pressed(MouseButton::Left) && point_inside) {
                Some(GMSelectThisItem)
            } else {
                None
            }
        } else {
            if point_inside {
                self.active = true;
                Some(GMHighlightThisItem)
            } else {
                None
            }
        }
    }
    fn set_font(&mut self, font: &Rc<dyn GMFontT>) {
        self.inactive_text.set_font(font);
        self.active_text.set_font(font);
    }
    fn set_property(&mut self, name: &str, value: &Rc<dyn Any>) {
        self.inactive_text.set_property(name, value);
        self.active_text.set_property(name, value);
    }
}

pub struct GMMenuItemNumeric {
    base: GMMenuItemStatic,
    prefix: String,
    min_val: f32,
    max_val: f32,
    current_val: f32,
    step: f32,
}

impl GMMenuItemNumeric {
    pub fn new(inactive_text: Box<dyn GMTextT>, active_text: Box<dyn GMTextT>, prefix: &str, min_val: f32, max_val: f32, current_val: f32, step: f32) -> Self {
        let mut base = GMMenuItemStatic::new(inactive_text, active_text);
        let text = format!("{}{:.2}", prefix, current_val);
        base.set_text(&text);

        Self {
            base,
            prefix: prefix.to_string(),
            min_val,
            max_val,
            current_val,
            step,
        }
    }
    pub fn new_box(inactive_text: Box<dyn GMTextT>, active_text: Box<dyn GMTextT>, prefix: &str, min_val: f32, max_val: f32, current_val: f32, step: f32) -> Box<dyn GMMenuItemT> {
        Box::new(Self::new(inactive_text, active_text, prefix, min_val, max_val, current_val, step))
    }
    pub fn update_text(&mut self) {
        let text = format!("{}{:.2}", self.prefix, self.current_val);
        self.base.set_text(&text);
    }
}

impl GMMenuItemT for GMMenuItemNumeric {
    fn set_text(&mut self, text: &str) {
        self.base.set_text(text);
    }
    fn draw(&self) {
        self.base.draw();
    }
    fn update(&mut self) {
        self.base.update();
    }
    fn set_active(&mut self, active: bool) {
        self.base.set_active(active);
    }
    fn get_active(&self) -> bool {
        self.base.get_active()
    }
    fn event(&mut self) -> Option<GMMenuItemEvent> {
        use GMMenuItemEvent::*;

        let (mousex, mousey) = mouse_position();
        let point_inside = self.base.point_inside(mousex, mousey);

        if self.base.get_active() {
            if is_key_pressed(KeyCode::Up) {
                self.base.set_active(false);
                Some(GMHighlightPrevItem)
            } else if is_key_pressed(KeyCode::Down) {
                self.base.set_active(false);
                Some(GMHighlightNextItem)
            } else if is_key_pressed(KeyCode::Left) || (is_mouse_button_pressed(MouseButton::Left) && point_inside) {
                self.current_val -= self.step;
                if self.current_val < self.min_val {
                    self.current_val = self.min_val
                }
                self.update_text();
                Some(GMNewValue(GMValue::GMF32(self.current_val)))
            } else if is_key_pressed(KeyCode::Right) || (is_mouse_button_pressed(MouseButton::Right) && point_inside) {
                self.current_val += self.step;
                if self.current_val > self.max_val {
                    self.current_val = self.max_val
                }
                self.update_text();
                Some(GMNewValue(GMValue::GMF32(self.current_val)))
            } else {
                None
            }
        } else {
            if point_inside {
                self.base.set_active(true);
                Some(GMMenuItemEvent::GMHighlightThisItem)
            } else {
                None
            }
        }
    }
    fn set_font(&mut self, font: &Rc<dyn GMFontT>) {
        self.base.set_font(font);
    }
    fn set_property(&mut self, name: &str, value: &Rc<dyn Any>) {
        self.base.set_property(name, value);
    }
}

// TODO: Slider with sprites


pub struct GMMenuItemEnum {
    base: GMMenuItemStatic,
    prefix: String,
    items: Vec<String>,
    current_item: usize,
}

impl GMMenuItemEnum {
    pub fn new(inactive_text: Box<dyn GMTextT>, active_text: Box<dyn GMTextT>, prefix: &str, items: &[&str], current_item: usize) -> Self {
        let mut base = GMMenuItemStatic::new(inactive_text, active_text);
        let text = format!("{}{}", prefix, items[current_item]);
        base.set_text(&text);

        let mut string_items = Vec::new();
        for item in items.iter() {
            string_items.push(item.to_string());
        }

        Self {
            base,
            prefix: prefix.to_string(),
            items: string_items,
            current_item,
        }
    }
    pub fn new_box(inactive_text: Box<dyn GMTextT>, active_text: Box<dyn GMTextT>, prefix: &str, items: &[&str], current_item: usize) -> Box<dyn GMMenuItemT> {
        Box::new(Self::new(inactive_text, active_text, prefix, items, current_item))
    }
    pub fn update_text(&mut self) {
        let text = format!("{}{}", self.prefix, self.items[self.current_item]);
        self.base.set_text(&text);
    }
}

impl GMMenuItemT for GMMenuItemEnum {
    fn set_text(&mut self, text: &str) {
        self.base.set_text(text);
    }
    fn draw(&self) {
        self.base.draw();
    }
    fn update(&mut self) {
        self.base.update();
    }
    fn set_active(&mut self, active: bool) {
        self.base.set_active(active);
    }
    fn get_active(&self) -> bool {
        self.base.get_active()
    }
    fn event(&mut self) -> Option<GMMenuItemEvent> {
        use GMMenuItemEvent::*;

        let (mousex, mousey) = mouse_position();
        let point_inside = self.base.point_inside(mousex, mousey);

        if self.base.get_active() {
            let first = 0;
            let last = self.items.len() - 1;

            if is_key_pressed(KeyCode::Up) {
                self.base.set_active(false);
                Some(GMHighlightPrevItem)
            } else if is_key_pressed(KeyCode::Down) {
                self.base.set_active(false);
                Some(GMHighlightNextItem)
            } else if is_key_pressed(KeyCode::Left) || (is_mouse_button_pressed(MouseButton::Left) && point_inside) {
                if self.current_item > first {
                    self.current_item -= 1;
                } else {
                    self.current_item = last;
                }
                self.update_text();
                Some(GMNewValue(GMValue::GMUSize(self.current_item)))
            } else if is_key_pressed(KeyCode::Right) || (is_mouse_button_pressed(MouseButton::Right) && point_inside) {
                if self.current_item < last {
                    self.current_item += 1;
                } else {
                    self.current_item = first;
                }
                self.update_text();
                Some(GMNewValue(GMValue::GMUSize(self.current_item)))
            } else {
                None
            }
        } else {
            if point_inside {
                self.base.set_active(true);
                Some(GMMenuItemEvent::GMHighlightThisItem)
            } else {
                None
            }
        }
    }
    fn set_font(&mut self, font: &Rc<dyn GMFontT>) {
        self.base.set_font(font);
    }
    fn set_property(&mut self, name: &str, value: &Rc<dyn Any>) {
        self.base.set_property(name, value);
    }
}
