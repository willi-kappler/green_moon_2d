
use std::fs::File;
use std::io::Read;

use sdl2::video;
use sdl2::render;
use sdl2::keyboard::Keycode;
use sdl2::event::Event;
use sdl2::pixels;

use log::debug;

use crate::assets::GMAssets;
use crate::configuration::GMConfiguration;
use crate::error::GMError;


#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum GMSceneState {
    Enter,
    Run,
    Leave,
    ChangeToScene(String),
    Quit,
}

pub struct GMContext {
    configuration: GMConfiguration,
    pub(crate) new_fps: u32,
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
        let canvas = window.into_canvas()
            .accelerated()
            .present_vsync()
            .build().unwrap();
        let event_pump = sdl_context.event_pump().unwrap();

        Self {
            configuration,
            new_fps: 0,
            scene_state: GMSceneState::Enter,
            canvas,
            event_pump,
            key_esc_down_: false,
            key_esc_up_: false,
        }
    }

    pub fn set_fps(&mut self, new_fps: u32) {
        self.new_fps = new_fps;
    }

    pub fn load_assets(&mut self, assets_file: &str) -> Result<(), GMError> {
        debug!("GMContext::load_assets(), from file: '{}'", assets_file);

        let mut file = File::open(assets_file)?;
        let mut data = Vec::new();

        file.read_to_end(&mut data)?;

        let all_assets: GMAssets = serde_json::from_slice(&data)?;

        for texture in all_assets.textures.iter() {
            self.add_texture(&texture.name, &texture.file, texture.rows, texture.cols);
        }

        for animation in all_assets.animations.iter() {
            self.add_animation(&animation.name, &animation.frames, animation.animation_type);
        }

        for font in all_assets.fonts.iter() {
            self.add_font(&font.name, &font.texture, &font.mapping);
        }

        Ok(())
    }

    pub fn add_texture(&mut self, name: &str, file: &str, rows: u32, cols: u32) {
        debug!("GMContext::add_texture(), name: '{}', path: '{}'", name, file);

        todo!();
    }

    pub fn add_animation(&mut self, name: &str, frames: &[(usize, f32)], animation_type: u8) {
        debug!("GMContext::add_animation(), name: '{}'", name);

        todo!();
    }

    pub fn add_font(&mut self, name: &str, texture: &str, mapping: &str) {
        debug!("GMContext::add_font(), name: '{}', texture: '{}', mapping: '{}'", name, texture, mapping);

        todo!();
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

    pub fn clear_black(&mut self) {
        let black = pixels::Color::BLACK;
        self.canvas.set_draw_color(black);
        self.canvas.clear();
    }

    pub fn set_fullscreen(&mut self, fullscreen: bool) {
        // TODO: Map SDL2 error
        if fullscreen {
            self.canvas.window_mut().set_fullscreen(video::FullscreenType::True).ok();
        } else {
            self.canvas.window_mut().set_fullscreen(video::FullscreenType::Off).ok();
        }
    }

    pub fn key_esc_down(&self) -> bool {
        self.key_esc_down_
    }

    pub fn key_esc_up(&self) -> bool {
        self.key_esc_up_
    }

}
