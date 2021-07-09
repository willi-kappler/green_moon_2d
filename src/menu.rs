
use crate::font::GMFontT;
use crate::text::{GMTextT, GMTextStatic};
use crate::sprite::GMSprite;
use crate::value::GMValue;
use crate::sound::GMSound;
use crate::menuitem::{GMMenuItemT, GMMenuItemStatic, GMMenuItemEvent};
use crate::behavior::GMKeyValue;

// use macroquad::window::{screen_width};

use std::rc::Rc;

pub struct GMMenu {
    title: Box<dyn GMTextT>,
    items: Vec<Box<dyn GMMenuItemT>>,
    highlighted: usize,
    change_sound: Rc<GMSound>,
    enter_sound: Rc<GMSound>,
    // TODO: Maybe add fancy border and background?
}

impl GMMenu {
    pub fn new(title: Box<dyn GMTextT>, mut items: Vec<Box<dyn GMMenuItemT>>, change_sound: &Rc<GMSound>, enter_sound: &Rc<GMSound>) -> Self {
        if items.len() > 0 {
            items[0].set_active(true);
        }

        Self {
            title,
            items,
            highlighted: 0,
            change_sound: change_sound.clone(),
            enter_sound: enter_sound.clone(),
        }
    }
    pub fn new_empty(title: Box<dyn GMTextT>, change_sound: &Rc<GMSound>, enter_sound: &Rc<GMSound>) -> Self {
        Self::new(title, Vec::new(), change_sound, enter_sound)
    }
    pub fn new_static_arrow(x: f32, y: f32, title: &str, items: &[&str], font: &Rc<dyn GMFontT>, change_sound: &Rc<GMSound>, enter_sound: &Rc<GMSound>) -> Self {
        let mut current_y = y;

        let title = GMTextStatic::new_box(title, x, y, font);
        let mut menu_items = Vec::new();
        let (_, font_height) = font.get_extend('A');
        current_y += font_height * 2.0;

        for item in items.iter() {
            let menu_item = GMMenuItemStatic::new_static_arrow(item, x, current_y, font);

            menu_items.push(menu_item);

            current_y += font_height + 4.0;
        }

        GMMenu::new(title, menu_items, change_sound, enter_sound)
    }
    pub fn new_static_sprite(x: f32, y: f32, title: &str, items: &[&str], font: &Rc<dyn GMFontT>, sprite: &GMSprite, change_sound: &Rc<GMSound>, enter_sound: &Rc<GMSound>) -> Self {
        let mut current_y = y;

        let title = GMTextStatic::new_box(title, x, y, font);
        let mut menu_items = Vec::new();
        let (_, font_height) = font.get_extend('A');
        current_y += font_height * 2.0;

        for item in items.iter() {
            let menu_item = GMMenuItemStatic::new_static_sprite(item, x, current_y, font, sprite);

            menu_items.push(menu_item);

            current_y += font_height + 4.0;
        }

        GMMenu::new(title, menu_items, change_sound, enter_sound)
    }
    pub fn add_item(&mut self, mut item: Box<dyn GMMenuItemT>, dx: f32, dy: f32) {
        if self.items.is_empty() {
            item.set_active(true);
            self.items.push(item);
        } else {
            let last = self.items.len() - 1;
            let px = self.items[last].get_x();
            let py = self.items[last].get_y();
            item.set_x(px + dx);
            item.set_y(py + dy);
            self.items.push(item);
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
    pub fn event(&mut self) -> Option<(usize, GMValue)> {
        let first = 0;
        let last = self.items.len() - 1;
        let mut new_highlighted: Option<(usize, bool)> = None;

        for (i, item) in self.items.iter_mut().enumerate() {
            match item.event() {
                Some(e) => {
                    use GMMenuItemEvent::*;
                    use GMValue::*;

                    match e {
                        SelectThisItem => {
                            self.change_sound.stop();
                            self.enter_sound.play();
                            return Some((i, None))
                        }
                        HighlightPrevItem => {
                            self.change_sound.stop();
                            self.change_sound.play();

                            if i == first {
                                self.highlighted = last;
                                new_highlighted = Some((last, true));
                            } else {
                                self.highlighted = i - 1;
                                new_highlighted = Some((i - 1, true));
                            }
                        }
                        HighlightThisItem => {
                            self.change_sound.stop();
                            self.change_sound.play();

                            new_highlighted = Some((self.highlighted, false));
                            self.highlighted = i;
                        }
                        HighlightNextItem => {
                            self.change_sound.stop();
                            self.change_sound.play();

                            if i == last {
                                self.highlighted = first;
                                new_highlighted = Some((first, true));
                            } else {
                                self.highlighted = i + 1;
                                new_highlighted = Some((i + 1, true));
                            }
                        }
                        NewValue(v) => {
                            self.change_sound.stop();
                            self.change_sound.play();
                            return Some((i, v))
                        }
                    }
                }
                None => {
                    // Nothing to do...
                }
            }
        }

        if let Some((i, v)) = new_highlighted {
            self.items[i].set_active(v);
        }

        None
    }
    pub fn set_title_font(&mut self, font: &Rc<dyn GMFontT>) {
        self.title.set_font(font);
    }
    pub fn set_item_font(&mut self, font: &Rc<dyn GMFontT>) {
        for item in self.items.iter_mut() {
            item.set_font(font);
        }
    }
    pub fn change_property_all(&mut self, data: &GMKeyValue) {
        for item in self.items.iter_mut() {
            item.set_property(data);
        }
    }
    pub fn change_property_one(&mut self, i: usize, data: &GMKeyValue) {
        self.items[i].set_property(data);
    }
    pub fn change_property_title(&mut self, data: &GMKeyValue) {
        self.title.set_property(data);
    }
}
