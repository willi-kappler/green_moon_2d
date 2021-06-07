

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum GMSceneState {
    Stay,
    Switch(String),
}

pub trait GMSceneT {
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

pub struct GMEmptyScene {

}

impl GMSceneT for GMEmptyScene {

}
