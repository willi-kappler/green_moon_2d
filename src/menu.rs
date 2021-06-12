
use crate::font::GMFontT;
use crate::text::{GMTextT, GMStaticText, GMArrowText};
use crate::value::GMValue;
use crate::sound::GMSound;
use crate::menu_item::{GMMenuItemT, GMMenuItemStatic};

// use macroquad::window::{screen_width};
use macroquad::input::{is_key_pressed, KeyCode};

use std::rc::Rc;

pub struct GMMenu {
    title: Box<dyn GMTextT>,
    items: Vec<Box<dyn GMMenuItemT>>,
    selected: usize,
    change_sound: Rc<GMSound>,
    enter_sound: Rc<GMSound>,
    // TODO: Maybe fancy border ?
}

impl GMMenu {
    pub fn new(title: Box<dyn GMTextT>, mut items: Vec<Box<dyn GMMenuItemT>>, change_sound: &Rc<GMSound>, enter_sound: &Rc<GMSound>) -> Self {
        items[0].set_active(true);

        Self {
            title,
            items,
            selected: 0,
            change_sound: change_sound.clone(),
            enter_sound: enter_sound.clone(),
        }
    }
    pub fn new_simple(x: f32, y: f32, title: &str, items: &[&str], font: &Rc<dyn GMFontT>, change_sound: &Rc<GMSound>, enter_sound: &Rc<GMSound>) -> Self {
        let mut current_y = y;

        let title = GMStaticText::new_box(title, x, y, &font);
        let mut menu_items = Vec::new();
        let (_, font_height) = font.get_extend('A');
        current_y += font_height * 2.0;

        for item in items.iter() {
            let inactive = GMStaticText::new_box(item, x, current_y, &font);
            let active = GMStaticText::new_box(item, x, current_y, &font);
            let active = GMArrowText::new_box(active);
            let menu_item = GMMenuItemStatic::new_box(inactive, active);

            menu_items.push(menu_item);

            current_y += font_height + 4.0;
        }

        GMMenu::new(title, menu_items, change_sound, enter_sound)
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
            self.change_sound.stop();
            self.change_sound.play();

            if self.selected > first {
                self.selected -= 1;
            } else {
                self.selected = last;
            }

            self.items[self.selected].set_active(true);
        } else if is_key_pressed(KeyCode::Down) {
            self.items[self.selected].set_active(false);
            self.change_sound.stop();
            self.change_sound.play();

            if self.selected < last {
                self.selected += 1;
            } else {
                self.selected = first;
            }

            self.items[self.selected].set_active(true);
        }

        if is_key_pressed(KeyCode::Enter) {
            self.change_sound.stop();
            self.enter_sound.play();
            Some(self.selected)
        } else {
            None
        }
    }
    pub fn get_values(&self) -> Vec<GMValue> {
        self.items.iter().map(|item| item.get_value()).collect::<Vec<GMValue>>()
    }
}
