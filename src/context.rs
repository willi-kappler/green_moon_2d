


use sdl2::video;
use sdl2::render;
use sdl2::keyboard::Keycode;
use sdl2::event::Event;

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
    pub frame_time: f32,
    configuration: GMConfiguration,
    scene_state: GMSceneState,
    canvas: render::Canvas<video::Window>,
    event_pump: sdl2::EventPump,
    key_esc_down_: bool,
    key_esc_up_: bool,
}

impl GMContext {
    pub fn new(configuration: GMConfiguration) -> Self {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
        let window = video_subsystem.window(
            &configuration.window_title,
            configuration.screen_width,
            configuration.screen_height)
            .position_centered()
            .build()
            .unwrap();
        let canvas = window.into_canvas().build().unwrap();
        let event_pump = sdl_context.event_pump().unwrap();

        let frame_time = 1.0 / configuration.fps;

        Self {
            frame_time,
            configuration,
            scene_state: GMSceneState::Enter,
            canvas,
            event_pump,
            key_esc_down_: false,
            key_esc_up_: false,
        }
    }

    pub fn set_fps(&mut self, fps: f32) {
        self.frame_time = 1.0 / fps;
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
        self.key_esc_down_ = false;
        self.key_esc_up_ = false;

        for event in self.event_pump.poll_iter() {
            match event {
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    self.key_esc_down_ = true;
                }
                Event::KeyUp { keycode: Some(Keycode::Escape), .. } => {
                    self.key_esc_up_ = true;
                }
                _ => {

                }
            }
        }

        Ok(())
    }

    pub(crate) fn draw(&mut self) -> Result<(), GMError> {
        Ok(())
    }

    pub(crate) fn present(&mut self) {
        self.canvas.present();
    }

    pub fn key_esc_down(&self) -> bool {
        self.key_esc_down_
    }

    pub fn key_esc_up(&self) -> bool {
        self.key_esc_up_
    }

}
