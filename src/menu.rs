
use crate::text::{GMText};
use crate::scene::{GMScene, GMSceneState};

pub struct GMMenuItem {
    text: GMText,
    selected: bool,
}

pub struct GMMenu {
    pub(crate) title: GMText,
    pub(crate) items: Vec<GMMenuItem>,
    pub(crate) item_effect: Box<dyn GMMenuItemEffect>,
}

pub trait GMMenuItemEffect {
    fn draw(&self, text: &GMText);

    fn update(&mut self, text: &GMText) {

    }
}

impl GMScene for GMMenu {
    fn enter(&mut self) {

    }

    fn update(&mut self) -> GMSceneState {
        GMSceneState::Stay
    }

    fn draw(&self) {

    }

    fn event(&mut self) {

    }
}
