

// Rust modules
use std::rc::Rc;

// Local modules
use crate::misc::{GM_Position};
use crate::misc::{GM_Runtime};
use crate::misc::{GM_Dimension};
use crate::gfx::{GM_BitmapFont};
use crate::gfx::{GM_Canvas};

pub trait GM_Text_T {
    fn set_text(&mut self, text: &str);
    fn get_text(&self) -> String;
    fn set_pos(&mut self, position: GM_Position);
    fn get_pos(&self) -> GM_Position;
    fn set_font(&mut self, font: &Rc<GM_BitmapFont>);
    fn set_base(&mut self, base: Box<dyn GM_Text_T>);
    fn get_dimensions(&self) -> GM_Dimension;
    fn update(&mut self, runtime: &GM_Runtime);
    fn draw(&self, canvas: &mut GM_Canvas);
}

pub trait GM_SelectableText_T: GM_Text_T {
    fn set_selected(&mut self, selected: bool);
    fn get_selected(&self) -> bool;
}

pub struct GM_StaticText {
    font: Rc<GM_BitmapFont>,
    text: String,
    position: GM_Position,
}

impl GM_StaticText {
    pub fn new(font: &Rc<GM_BitmapFont>, text: &str) -> GM_StaticText {
        GM_StaticText {
            font: font.clone(),
            text: text.to_string(),
            position: GM_Position::new(0, 0),
        }
    }

    pub fn new_pos(text: &str, font: Rc<GM_BitmapFont>, position: GM_Position) -> GM_StaticText {
        GM_StaticText {
            font,
            text: text.to_string(),
            position,
        }
    }
}

impl GM_Text_T for GM_StaticText { 
    fn set_text(&mut self, text: &str) {
        self.text = text.to_string();
    }

    fn get_text(&self) -> String {
        self.text.clone()
    }

    fn set_pos(&mut self, position: GM_Position) {
        self.position = position;
    }

    fn get_pos(&self) -> GM_Position {
        self.position.clone()
    }

    fn set_font(&mut self, font: &Rc<GM_BitmapFont>) {
        self.font = font.clone();
    }

    fn update(&mut self, runtime: &GM_Runtime) {

    }

    fn set_base(&mut self, base: Box<dyn GM_Text_T>) {
        // Nothing to do for static text
        // It already is the most basic text struct
    }

    fn get_dimensions(&self) -> GM_Dimension {
        // TODO: implement
        GM_Dimension::new(0, 0)
    }

    fn draw(&self, canvas: &mut GM_Canvas) {

    }
}

pub struct GM_WaveText {
    base: Box<dyn GM_Text_T>,
}

impl GM_WaveText {
    pub fn new(font: &Rc<GM_BitmapFont>, text: &str) -> GM_WaveText {
        let base = Box::new(GM_StaticText::new(font, text));

        // TODO: implement it
        GM_WaveText {
            base,
        }
    }
}

impl GM_Text_T for GM_WaveText {
    fn set_text(&mut self, text: &str) {
        self.base.set_text(text);
    }

    fn get_text(&self) -> String {
        self.base.get_text()
    }

    fn set_pos(&mut self, position: GM_Position) {
        self.base.set_pos(position);
    }

    fn get_pos(&self) -> GM_Position {
        self.base.get_pos()
    }

    fn set_font(&mut self, font: &Rc<GM_BitmapFont>) {
        self.base.set_font(font)
    }

    fn update(&mut self, runtime: &GM_Runtime) {

    }

    fn set_base(&mut self, base: Box<dyn GM_Text_T>) {
        self.base = base;
    }

    fn get_dimensions(&self) -> GM_Dimension {
        // TODO: implement
        GM_Dimension::new(0, 0)
    }

    fn draw(&self, canvas: &mut GM_Canvas) {

    }
}

pub struct GM_SelectableText {
    base: Box<dyn GM_Text_T>,
    font_unselected: Rc<GM_BitmapFont>,
    font_selected: Rc<GM_BitmapFont>,
    selected: bool,
}

impl GM_SelectableText {
    pub fn new(font_unselected: &Rc<GM_BitmapFont>, font_selected: &Rc<GM_BitmapFont>, text: &str) -> GM_SelectableText {
        let base = Box::new(GM_StaticText::new(font_unselected, text));

        GM_SelectableText {
            base,
            font_selected: font_selected.clone(),
            font_unselected: font_unselected.clone(),
            selected: false,
        }
    }
}

impl GM_Text_T for GM_SelectableText {
    fn set_text(&mut self, text: &str) {
        self.base.set_text(text);
    }

    fn get_text(&self) -> String {
        self.base.get_text()
    }

    fn set_pos(&mut self, position: GM_Position) {
        self.base.set_pos(position);
    }

    fn get_pos(&self) -> GM_Position {
        self.base.get_pos()
    }

    fn set_font(&mut self, font: &Rc<GM_BitmapFont>) {
        self.base.set_font(font)
    }

    fn update(&mut self, runtime: &GM_Runtime) {

    }

    fn set_base(&mut self, base: Box<dyn GM_Text_T>) {
        self.base = base;
    }

    fn get_dimensions(&self) -> GM_Dimension {
        // TODO: implement
        GM_Dimension::new(0, 0)
    }

    fn draw(&self, canvas: &mut GM_Canvas) {

    }
}

impl GM_SelectableText_T for GM_SelectableText {
    fn get_selected(&self) -> bool {
        self.selected
    }

    fn set_selected(&mut self, selected: bool) {
        self.selected = selected
    }
}
