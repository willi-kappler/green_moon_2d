
use crate::text::{GMText};
use crate::text_effect::{GMRotZ, GMSelected1, GMStaticText, GMTextEffect, GMTextEffectT};
use crate::font::{GMBitmapFont};
use crate::scene::{GMScene, GMSceneState};

use macroquad::window::{screen_width};
use macroquad::input::{is_key_pressed, KeyCode};

use std::rc::Rc;

pub struct GMMenu {
    pub(crate) title: GMText,
    pub(crate) items: Vec<GMText>,
    pub(crate) highlighted: usize,
    pub(crate) effect_title: GMTextEffect,
    pub(crate) effect_highlight: GMTextEffect,
    pub(crate) effect_normal: GMTextEffect,
    pub(crate) font: Rc<GMBitmapFont>,
}

impl GMMenu {
    pub fn new(title: &str, menu_items: &[&str], font: Rc<GMBitmapFont>, y: f32) -> GMMenu {
        let mut ym = y;

        let center_x = screen_width() / 2.0;

        let mut title = GMText::new(title, center_x, ym);
        // TODO: title.set_alignment();

        let effect_title = Box::new(GMRotZ::new(1.0, &title, &font));
        let effect_normal = Box::new(GMStaticText::new());
        let effect_highlight = Box::new(GMSelected1::new()); // TODO:

        let (_, ye) = effect_title.get_extend(&title, &font);

        ym += 2.0 * ye;

        let mut items = Vec::new();

        for item in menu_items.iter() {
            let text = GMText::new(*item, 0.0, ym);
            let (_, ye) = effect_normal.get_extend(&text, &font);
            ym += ye;
            items.push(text)
        }

        GMMenu {
            title,
            items,
            highlighted: 0,
            effect_title,
            effect_highlight,
            effect_normal,
            font,
        }
    }

    pub fn set_item(&mut self, i: usize, text: &str) {
        self.items[i].set_text(text)
    }

    pub fn get_highlighted(&self) -> usize {
        self.highlighted
    }

    pub fn update(&mut self) {
        self.effect_title.update();
        self.effect_normal.update();
        self.effect_highlight.update();
    }

    pub fn draw(&self) {
        self.effect_title.draw(&self.title, &self.font);

        for i in 0..self.items.len() {
            let item = &self.items[i];
            if i == self.highlighted {
                self.effect_highlight.draw(item, &self.font);
            } else {
                self.effect_normal.draw(item, &self.font);
            }
        }
    }

    pub fn event(&mut self) {
        if is_key_pressed(KeyCode::Up) {
            if self.highlighted == 0 {
                self.highlighted = self.items.len() - 1;
            } else {
                self.highlighted -= 1;
            }
        } else if is_key_pressed(KeyCode::Down) {
            if self.highlighted == self.items.len() - 1 {
                self.highlighted = 0;
            } else {
                self.highlighted += 1;
            }
        }
    }
}
