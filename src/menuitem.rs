
use crate::font::GMFont;
use crate::text::{GMText, GMTextStatic, GMTextArrow, GMTextSprite};
use crate::sprite::GMSpriteSimple;
use crate::resources::GMResourceManager;
use crate::utils::{GMKeyValue, in_rect, GMValue};

use macroquad::input::{is_key_pressed, KeyCode, mouse_position, is_mouse_button_pressed, MouseButton};


// TODO:
// - use GMMenuItem instead of GMMenuItemT

pub enum GMMenuItemEvent {
    SelectThisItem,
    HighlightPrevItem,
    HighlightThisItem,
    HighlightNextItem,
    NewValue(GMValue),
}

pub trait GMMenuItemT {
    fn set_text(&mut self, text: &str);
    fn draw(&self);
    fn update(&mut self);
    fn set_active(&mut self, active: bool);
    fn get_active(&self) -> bool;
    fn set_x(&mut self, x: f32);
    fn get_x(&self) -> f32;
    fn set_y(&mut self, y: f32);
    fn get_y(&self) -> f32;
    fn event(&mut self) -> Option<GMMenuItemEvent>;
    fn set_font(&mut self, font: &GMFont);
    fn set_property(&mut self, data: &GMKeyValue);
}

pub struct GMMenuItem {
    menu_item: Box<dyn GMMenuItemT>,
}

impl GMMenuItem {
    pub fn new<T: 'static + GMMenuItemT>(menu_item: T) -> Self {
        Self {
            menu_item: Box::new(menu_item),
        }
    }
    pub fn set_text(&mut self, text: &str) {
        self.menu_item.set_text(text);
    }
    pub fn draw(&self) {
        self.menu_item.draw();
    }
    pub fn update(&mut self) {
        self.menu_item.update();
    }
    pub fn set_active(&mut self, active: bool) {
        self.menu_item.set_active(active);
    }
    pub fn get_active(&self) -> bool {
        self.menu_item.get_active()
    }
    pub fn set_x(&mut self, x: f32) {
        self.menu_item.set_x(x);
    }
    pub fn get_x(&self) -> f32 {
        self.menu_item.get_x()
    }
    pub fn set_y(&mut self, y: f32) {
        self.menu_item.set_y(y)
    }
    pub fn get_y(&self) -> f32 {
        self.menu_item.get_y()
    }
    pub fn event(&mut self) -> Option<GMMenuItemEvent> {
        self.menu_item.event()
    }
    pub fn set_font(&mut self, font: &GMFont) {
        self.menu_item.set_font(font);
    }
    pub fn set_property(&mut self, data: &GMKeyValue) {
        self.menu_item.set_property(data);
    }
}

pub struct GMMenuItemStatic {
    inactive_text: GMText,
    active_text: GMText,
    active: bool,
}

impl GMMenuItemStatic {
    pub fn new(inactive_text: GMText, active_text: GMText) -> Self {
        Self {
            inactive_text,
            active_text,
            active: false,
        }
    }
    pub fn new_box(inactive_text: GMText, active_text: GMText) -> GMMenuItem {
        GMMenuItem::new(Self::new(inactive_text, active_text))
    }
    pub fn new_static_arrow(text: &str, x: f32, y: f32, font: &GMFont) -> GMMenuItem {
        let inactive_text = GMTextStatic::new_box(text, x, y, font);
        let active_text = GMTextArrow::new_static(text, x, y, font);
        Self::new_box(inactive_text, active_text)
    }
    pub fn new_static_sprite(text: &str, x: f32, y: f32, font: &GMFont, sprite: &GMSpriteSimple) -> GMMenuItem {
        let inactive_text = GMTextStatic::new_box(text, x, y, font);
        let active_text = GMTextSprite::new_static(text, x, y, font, sprite);
        Self::new_box(inactive_text, active_text)
    }
    pub fn new_from_resource(text: &str, x: f32, y: f32, resources: &GMResourceManager, font_name: &str, sprite_name: &str) -> GMMenuItem {
        let font = resources.get_font(font_name).unwrap();
        let sprite = resources.get_sprite_simple(sprite_name).unwrap();
        Self::new_static_sprite(text, x, y, &font, &sprite)
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
                Some(HighlightPrevItem)
            } else if is_key_pressed(KeyCode::Down) {
                self.active = false;
                Some(HighlightNextItem)
            } else if is_key_pressed(KeyCode::Enter) || (is_mouse_button_pressed(MouseButton::Left) && point_inside) {
                Some(SelectThisItem)
            } else {
                None
            }
        } else {
            if point_inside {
                self.active = true;
                Some(HighlightThisItem)
            } else {
                None
            }
        }
    }
    fn set_font(&mut self, font: &GMFont) {
        self.inactive_text.set_font(font);
        self.active_text.set_font(font);
    }
    fn set_property(&mut self, data: &GMKeyValue) {
        self.inactive_text.set_property(data);
        self.active_text.set_property(data);
    }

    fn set_x(&mut self, x: f32) {
        self.inactive_text.set_x(x);
        self.active_text.set_x(x);
    }
    fn get_x(&self) -> f32 {
        self.inactive_text.get_x()
    }
    fn set_y(&mut self, y: f32) {
        self.inactive_text.set_y(y);
        self.active_text.set_y(y);
    }
    fn get_y(&self) -> f32 {
        self.inactive_text.get_y()
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
    pub fn new(inactive_text: GMText, active_text: GMText, prefix: &str, min_val: f32, max_val: f32, current_val: f32, step: f32) -> Self {
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
    pub fn new_box(inactive_text: GMText, active_text: GMText, prefix: &str, min_val: f32, max_val: f32, current_val: f32, step: f32) -> GMMenuItem {
        GMMenuItem::new(Self::new(inactive_text, active_text, prefix, min_val, max_val, current_val, step))
    }
    pub fn new_static_arrow(prefix: &str, x: f32, y: f32, font: &GMFont, min_val: f32, max_val: f32, current_val: f32, step: f32) -> GMMenuItem {
        let inactive_text = GMTextStatic::new_box(prefix, x, y, font);
        let active_text = GMTextArrow::new_static(prefix, x, y, font);
        Self::new_box(inactive_text, active_text, prefix, min_val, max_val, current_val, step)
    }
    pub fn new_static_sprite(prefix: &str, x: f32, y: f32, font: &GMFont, sprite: &GMSpriteSimple, min_val: f32, max_val: f32, current_val: f32, step: f32) -> GMMenuItem {
        let inactive_text = GMTextStatic::new_box(prefix, x, y, font);
        let active_text = GMTextSprite::new_static(prefix, x, y, font, sprite);
        Self::new_box(inactive_text, active_text, prefix, min_val, max_val, current_val, step)
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
                Some(HighlightPrevItem)
            } else if is_key_pressed(KeyCode::Down) {
                self.base.set_active(false);
                Some(HighlightNextItem)
            } else if is_key_pressed(KeyCode::Left) || (is_mouse_button_pressed(MouseButton::Left) && point_inside) {
                self.current_val -= self.step;
                if self.current_val < self.min_val {
                    self.current_val = self.min_val
                }
                self.update_text();
                Some(NewValue(GMValue::F32(self.current_val)))
            } else if is_key_pressed(KeyCode::Right) || (is_mouse_button_pressed(MouseButton::Right) && point_inside) {
                self.current_val += self.step;
                if self.current_val > self.max_val {
                    self.current_val = self.max_val
                }
                self.update_text();
                Some(NewValue(GMValue::F32(self.current_val)))
            } else {
                None
            }
        } else {
            if point_inside {
                self.base.set_active(true);
                Some(GMMenuItemEvent::HighlightThisItem)
            } else {
                None
            }
        }
    }
    fn set_font(&mut self, font: &GMFont) {
        self.base.set_font(font);
    }
    fn set_property(&mut self, data: &GMKeyValue) {
        self.base.set_property(data);
    }
    fn set_x(&mut self, x: f32) {
        self.base.set_x(x);
    }
    fn get_x(&self) -> f32 {
        self.base.get_x()
    }
    fn set_y(&mut self, y: f32) {
        self.base.set_y(y);
    }
    fn get_y(&self) -> f32 {
        self.base.get_y()
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
    pub fn new(inactive_text: GMText, active_text: GMText, prefix: &str, items: &[&str], current_item: usize) -> Self {
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
    pub fn new_box(inactive_text: GMText, active_text: GMText, prefix: &str, items: &[&str], current_item: usize) -> GMMenuItem {
        GMMenuItem::new(Self::new(inactive_text, active_text, prefix, items, current_item))
    }
    pub fn new_static_arrow(prefix: &str, x: f32, y: f32, font: &GMFont, items: &[&str], current_item: usize) -> GMMenuItem {
        let inactive_text = GMTextStatic::new_box(prefix, x, y, font);
        let active_text = GMTextArrow::new_static(prefix, x, y, font);
        Self::new_box(inactive_text, active_text, prefix, items, current_item)
    }
    pub fn new_static_sprite(prefix: &str, x: f32, y: f32, font: &GMFont, sprite: &GMSpriteSimple, items: &[&str], current_item: usize) -> GMMenuItem {
        let inactive_text = GMTextStatic::new_box(prefix, x, y, font);
        let active_text = GMTextSprite::new_static(prefix, x, y, font, sprite);
        Self::new_box(inactive_text, active_text, prefix, items, current_item)
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
                Some(HighlightPrevItem)
            } else if is_key_pressed(KeyCode::Down) {
                self.base.set_active(false);
                Some(HighlightNextItem)
            } else if is_key_pressed(KeyCode::Left) || (is_mouse_button_pressed(MouseButton::Left) && point_inside) {
                if self.current_item > first {
                    self.current_item -= 1;
                } else {
                    self.current_item = last;
                }
                self.update_text();
                Some(NewValue(GMValue::USize(self.current_item)))
            } else if is_key_pressed(KeyCode::Right) || (is_mouse_button_pressed(MouseButton::Right) && point_inside) {
                if self.current_item < last {
                    self.current_item += 1;
                } else {
                    self.current_item = first;
                }
                self.update_text();
                Some(NewValue(GMValue::USize(self.current_item)))
            } else {
                None
            }
        } else {
            if point_inside {
                self.base.set_active(true);
                Some(GMMenuItemEvent::HighlightThisItem)
            } else {
                None
            }
        }
    }
    fn set_font(&mut self, font: &GMFont) {
        self.base.set_font(font);
    }
    fn set_property(&mut self, data: &GMKeyValue) {
        self.base.set_property(data);
    }
    fn set_x(&mut self, x: f32) {
        self.base.set_x(x);
    }
    fn get_x(&self) -> f32 {
        self.base.get_x()
    }
    fn set_y(&mut self, y: f32) {
        self.base.set_y(y);
    }
    fn get_y(&self) -> f32 {
        self.base.get_y()
    }
}
