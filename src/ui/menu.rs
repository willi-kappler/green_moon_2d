

// Local modules
use crate::misc::{GM_Runtime};
use crate::misc::{GM_Position};
use crate::gfx::{GM_Canvas};
use super::text::{GM_Text_T, GM_SelectableText_T};


pub struct GM_Menu {
    title: Box<dyn GM_Text_T>,
    items: Vec<Box<dyn GM_SelectableText_T>>,
    position: GM_Position,
    selected_item: u8,
}

impl GM_Menu {
    pub fn new(title: Box<dyn GM_Text_T>, items: Vec<Box<dyn GM_SelectableText_T>>) -> GM_Menu {
        GM_Menu {
            title,
            items, 
            position: GM_Position::new(0, 0),
            selected_item: 0,
        }
    }

    pub fn add_item(&mut self, item: Box<dyn GM_SelectableText_T>) {
        self.items.push(item);
    }

    pub fn set_position(&mut self, position: &GM_Position) {
        self.position.set_position(position);
    }

    pub fn get_selected_item(&self) -> u8 {
        self.selected_item
    }

    pub fn process(&mut self, runtime: &GM_Runtime) {
    }

    pub fn update(&mut self, runtime: &GM_Runtime) {
    }

    pub fn draw(&self, canvas: &mut GM_Canvas) {
    }

}
