
use crate::text::{GMText};
use crate::text_effect::{GMTextEffect, GMStaticText, GMRotZ, GMSelected1};
use crate::font::{GMBitmapFont};
use crate::scene::{GMScene, GMSceneState};

use macroquad::window::screen_width;

use std::rc::Rc;

pub struct GMMenu {
    pub(crate) title: GMText,
    pub(crate) items: Vec<GMText>,
    pub(crate) selected: usize,
    pub(crate) effect_title: GMTextEffect,
    pub(crate) effect_selected: GMTextEffect,
    pub(crate) effect_normal: GMTextEffect,
    pub(crate) font: Rc<GMBitmapFont>,
}

impl GMMenu {
    pub fn new(title: &str, menu_items: &[&str], font: Rc<GMBitmapFont>, y: f32) -> GMMenu {
        let y1 = y;

        let x1 = screen_width() / 2.0;

        let title = GMText::new(title, x1, y1);

        let effect_title = Box::new(GMRotZ::new(1.0, &title, &font));
        let effect_normal = Box::new(GMStaticText::new());
        let effect_selected = Box::new(GMSelected1::new()); // TODO:

        let mut items = Vec::new();

        for item in menu_items.iter() {
            let text = GMText::new(*item, x1, y1);
            items.push(text)
        }

        GMMenu {
            title,
            items,
            selected: 0,
            effect_title,
            effect_selected,
            effect_normal,
            font,
        }
    }
}

impl GMScene for GMMenu {
    fn enter(&mut self) {

    }

    fn update(&mut self) -> GMSceneState {
        self.effect_title.update(&self.title);
        GMSceneState::Stay
    }

    fn draw(&self) {
        self.effect_title.draw(&self.title, &self.font);

        for i in 0..self.items.len() {
            let item = &self.items[i];
            if i == self.selected {
                self.effect_selected.draw(item, &self.font);
            } else {
                self.effect_normal.draw(item, &self.font);
            }
        }
    }

    fn event(&mut self) {

    }
}
