

// Local modules
use crate::misc::runtime::{GM_Runtime};
use crate::misc::position::{GM_Position};

pub struct GM_Menu {
    position: GM_Position,
    selected_item: u8,
}

impl GM_Menu {
    pub fn new() -> GM_Menu {
        GM_Menu {
            position: GM_Position::new(0, 0),
            selected_item: 0,
        }
    }

    pub fn load(path: &str) -> GM_Menu {
        GM_Menu::new()
    }

    pub fn set_position(&mut self, position: &GM_Position) {
        self.position.set_position(position);
    }

    pub fn get_selected_item(&self) -> u8 {
        self.selected_item
    }

    pub fn process(&mut self, runtime: &mut GM_Runtime) {
    }

    pub fn update(&mut self, runtime: &mut GM_Runtime) {
    }

    pub fn draw(&mut self, runtime: &mut GM_Runtime) {
    }

}
