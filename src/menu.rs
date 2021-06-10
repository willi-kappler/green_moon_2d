
use crate::text::GMTextT;
use crate::value::GMValue;

// use macroquad::window::{screen_width};
use macroquad::input::{is_key_pressed, KeyCode};

pub trait GMMenuItemT {
    fn set_text(&mut self, text: &str);
    fn draw(&self);
    fn update(&mut self);
    fn set_active(&mut self, active: bool);
    fn event(&mut self) {
    }
    fn get_value(&self) -> GMValue {
        GMValue::GMNone
    }
}

pub struct GMMenuItemStatic {
    pub(crate) inactive_text: Box<dyn GMTextT>,
    pub(crate) active_text: Box<dyn GMTextT>,
    pub(crate) active: bool,
}

impl GMMenuItemStatic {
    pub fn new(inactive_text: Box<dyn GMTextT>, active_text: Box<dyn GMTextT>) -> Self {
        Self {
            inactive_text,
            active_text,
            active: false,
        }
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
}

pub struct GMMenuItemNumeric {
    pub(crate) base: GMMenuItemStatic,
    pub(crate) prefix: String,
    pub(crate) min_val: f32,
    pub(crate) max_val: f32,
    pub(crate) current_val: f32,
    pub(crate) step: f32,
}

impl GMMenuItemNumeric {
    pub fn new(inactive_text: Box<dyn GMTextT>, active_text: Box<dyn GMTextT>, prefix: &str, min_val: f32, max_val: f32, current_val: f32, step: f32) -> Self {
        let mut base = GMMenuItemStatic::new(inactive_text, active_text);
        let text = format!("{}: {}", prefix, current_val);
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
    pub fn update_text(&mut self) {
        let text = format!("{}: {}", self.prefix, self.current_val);
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
    fn event(&mut self) {
        if is_key_pressed(KeyCode::Left) {
            self.current_val -= self.step;
            if self.current_val < self.min_val {
                self.current_val = self.min_val
            }
            self.update_text();
        } else if is_key_pressed(KeyCode::Right) {
            self.current_val += self.step;
            if self.current_val > self.max_val {
                self.current_val = self.max_val
            }
            self.update_text();
        }
    }
    fn get_value(&self) -> GMValue {
        GMValue::GMF32(self.current_val)
    }
}

pub struct GMMenuItemEnum {
    pub(crate) base: GMMenuItemStatic,
    pub(crate) prefix: String,
    pub(crate) items: Vec<String>,
    pub(crate) current_item: usize,
}

impl GMMenuItemEnum {
    pub fn new(inactive_text: Box<dyn GMTextT>, active_text: Box<dyn GMTextT>, prefix: &str, items: Vec<String>, current_item: usize) -> Self {
        let mut base = GMMenuItemStatic::new(inactive_text, active_text);
        let text = format!("{}: {}", prefix, items[current_item]);
        base.set_text(&text);

        Self {
            base,
            prefix: prefix.to_string(),
            items,
            current_item,
        }
    }
    pub fn update_text(&mut self) {
        let text = format!("{}: {}", self.prefix, self.items[self.current_item]);
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
    fn event(&mut self) {
        let first = 0;
        let last = self.items.len() - 1;

        if is_key_pressed(KeyCode::Left) {
            if self.current_item > first {
                self.current_item -= 1;
            } else {
                self.current_item = last;
            }
            self.update_text();
        } else if is_key_pressed(KeyCode::Right) {
            if self.current_item < last {
                self.current_item += 1;
            } else {
                self.current_item = first;
            }
            self.update_text();
        }
    }
    fn get_value(&self) -> GMValue {
        GMValue::GMUSize(self.current_item)
    }
}

pub struct GMMenu {
    pub(crate) title: Box<dyn GMTextT>,
    pub(crate) items: Vec<Box<dyn GMMenuItemT>>,
    pub(crate) selected: usize,
    // TODO: Maybe fancy border ?
}

impl GMMenu {
    pub fn new(title: Box<dyn GMTextT>, mut items: Vec<Box<dyn GMMenuItemT>>) -> Self {
        items[0].set_active(true);

        Self {
            title,
            items,
            selected: 0,
        }
    }
    pub fn draw(&self) {
        self.title.draw();
        for item in self.items.iter() {
            item.draw();
        }
    }
    pub fn update(&mut self) {
        self.title.update();
        for item in self.items.iter_mut() {
            item.update();
        }
    }
    pub fn event(&mut self) -> Option<usize>{
        for item in self.items.iter_mut() {
            item.event();
        }

        let first: usize = 0;
        let last: usize = self.items.len() - 1;

        if is_key_pressed(KeyCode::Up) {
            self.items[self.selected].set_active(false);

            if self.selected > first {
                self.selected -= 1;
            } else {
                self.selected = last;
            }

            self.items[self.selected].set_active(true);
        } else if is_key_pressed(KeyCode::Down) {
            self.items[self.selected].set_active(false);

            if self.selected < last {
                self.selected += 1;
            } else {
                self.selected = first;
            }

            self.items[self.selected].set_active(true);
        }

        if is_key_pressed(KeyCode::Enter) {
            Some(self.selected)
        } else {
            None
        }
    }
    pub fn get_values(&self) -> Vec<GMValue> {
        self.items.iter().map(|item| item.get_value()).collect::<Vec<GMValue>>()
    }
}
