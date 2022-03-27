


use crate::error::GMError;
use crate::configuration::GMConfiguration;


#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum GMSceneState {
    Enter,
    Run,
    Leave,
    ChangeToScene(String),
    Quit,
}

pub struct GMContext {
    scene_state: GMSceneState,
    pub frame_time: f32,
}

impl GMContext {
    pub fn new() -> Self {
        Self {
            scene_state: GMSceneState::Enter,
            frame_time: 1.0 / 60.0 // 60 FPS
        }
    }

    pub fn get_scene_state(&self) -> &GMSceneState {
        &self.scene_state
    }

    pub fn enter_scene(&mut self) {
        self.scene_state = GMSceneState::Enter;
    }

    pub fn run_scene(&mut self) {
        self.scene_state = GMSceneState::Run;
    }

    pub fn leave_scene(&mut self) {
        self.scene_state = GMSceneState::Leave;
    }

    pub fn quit_app(&mut self) {
        self.scene_state = GMSceneState::Quit;
    }

    pub fn change_to_scene(&mut self, name: &str) {
        self.scene_state = GMSceneState::ChangeToScene(name.to_string());
    }

    pub(crate) fn update(&mut self) -> Result<(), GMError> {
        Ok(())
    }

    pub(crate) fn draw(&mut self) -> Result<(), GMError> {
        Ok(())
    }

    pub fn set_fps(&mut self, fps: f32) {
        self.frame_time = 1.0 / fps;
    }

    pub(crate) fn set_configuration(&mut self, _configuration: &GMConfiguration) {
        todo!();
    }
}
