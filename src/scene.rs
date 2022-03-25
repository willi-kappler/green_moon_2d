
use crate::error::GMError;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum GMSceneState {
    Enter,
    Run,
    Leave,
    NewScene(String),
    Quit,
}

pub trait GMScene {
    fn enter(&mut self) -> Result<GMSceneState, GMError>;
    fn run(&mut self) -> Result<GMSceneState, GMError>;
    fn leave(&mut self) -> Result<GMSceneState, GMError>;
}
