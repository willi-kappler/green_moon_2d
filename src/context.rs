
use std::fs::File;
use std::io::Read;
use std::rc::Rc;

use sdl2::video::{self, Window, WindowContext};
use sdl2::render::{TextureCreator, Canvas};
use sdl2::keyboard::Keycode;
use sdl2::event::Event;
use sdl2::pixels;
//use sdl2::surface::Surface;

use log::debug;

use crate::animation::GMAnimationT;
use crate::assets::GMAssets;
use crate::configuration::GMConfiguration;
use crate::error::GMError;
use crate::font::GMFontT;
use crate::texture::GMTexture;


#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum GMSceneState {
    Run,
    ChangeToScene(String),
    Quit,
}

pub struct GMContext {
    pub configuration: GMConfiguration,
    pub new_fps: u32,
    pub scene_state: GMSceneState,
    pub canvas: Canvas<Window>,
    pub texture_creator: TextureCreator<WindowContext>,
    pub event_pump: sdl2::EventPump,

    // Name, Object
    pub animations: Vec<(String, Box<dyn GMAnimationT>)>,
    pub fonts: Vec<(String, Rc<dyn GMFontT>)>,
    pub textures: Vec<Rc<GMTexture>>,

    pub key_esc_down_: bool,
    pub key_esc_up_: bool,
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
        let texture_creator = canvas.texture_creator();
        let event_pump = sdl_context.event_pump().unwrap();

        Self {
            configuration,
            new_fps: 0,
            scene_state: GMSceneState::Run,
            canvas,
            texture_creator,
            event_pump,

            animations: Vec::new(),
            fonts: Vec::new(),
            textures: Vec::new(),

            key_esc_down_: false,
            key_esc_up_: false,
        }
    }

    pub fn load_assets(&mut self, assets_file: &str) -> Result<(), GMError> {
        debug!("GMContext::load_assets(), from file: '{}'", assets_file);

        let mut file = File::open(assets_file)?;
        let mut data = Vec::new();

        file.read_to_end(&mut data)?;

        let all_assets: GMAssets = serde_json::from_slice(&data)?;

        for texture in all_assets.textures.iter() {
            self.add_texture(&texture.name, &texture.file, texture.rows, texture.cols)?;
        }

        for animation in all_assets.animations.iter() {
            self.add_animation1(&animation.name, &animation.frames, animation.animation_type)?;
        }

        for font in all_assets.fonts.iter() {
            self.add_font(&font.name, &font.texture, &font.mapping)?;
        }

        Ok(())
    }

    pub fn has_animation(&self, name: &str) -> bool {
        debug!("GMContext::has_animation(), name: '{}'", name);

        self.animations.iter().any(|(a_name, _)| a_name == name)
    }

    pub fn add_animation1(&mut self, name: &str, _frames: &[(usize, f32)], _animation_type: u8) -> Result<(), GMError> {
        debug!("GMContext::add_animation1(), name: '{}'", name);

        if self.has_animation(name) {
            return Err(GMError::AnimationAlreadyExists(name.to_string()))
        }

        todo!();
    }

    pub fn add_animation2<A: 'static + GMAnimationT>(&mut self, name: &str, animation: A) -> Result<(), GMError> {
        debug!("GMContext::add_animation2(), name: '{}'", name);

        if self.has_animation(name) {
            return Err(GMError::AnimationAlreadyExists(name.to_string()))
        }

        self.animations.push((name.to_string(), Box::new(animation)));

        Ok(())
    }

    pub fn add_animation3(&mut self, name: &str, animation: Box<dyn GMAnimationT>) -> Result<(), GMError> {
        debug!("GMContext::add_animation3(), name: '{}'", name);

        if self.has_animation(name) {
            return Err(GMError::AnimationAlreadyExists(name.to_string()))
        }

        self.animations.push((name.to_string(), animation));

        Ok(())
    }

    pub fn get_animation_clone(&self, name: &str) -> Result<Box<dyn GMAnimationT>, GMError> {
        debug!("GMContext::get_animation_clone(), name: '{}'", name);

        for (a_name, animation) in self.animations.iter() {
            if a_name == name {
                return Ok(animation.box_clone())
            }
        }

        Err(GMError::AnimationNotFound(name.to_string()))
    }

    pub fn remove_animation(&mut self, name: &str) -> Result<(), GMError> {
        debug!("GMContext::remove_animation(), name: '{}'", name);

        match self.animations.iter().position(|(a_name, _)| a_name == name) {
            Some(index) => {
                self.animations.remove(index);
                Ok(())
            }
            None => {
                Err(GMError::AnimationNotFound(name.to_string()))
            }
        }
    }

    pub fn add_font(&mut self, name: &str, texture: &str, mapping: &str) -> Result<(), GMError> {
        debug!("GMContext::add_font(), name: '{}', texture: '{}', mapping: '{}'", name, texture, mapping);

        todo!();
    }

    pub fn get_font_rc(&self, name: &str) -> Result<Rc<dyn GMFontT>, GMError> {
        debug!("GMContext::get_font_rc(), name: '{}'", name);

        for (f_name, font) in self.fonts.iter() {
            if f_name == name {
                return Ok(font.clone())
            }
        }

        Err(GMError::FontNotFound(name.to_string()))
    }

    pub fn remove_font(&mut self, name: &str) -> Result<(), GMError> {
        debug!("GMContext::remove_font(), name: '{}'", name);

        match self.fonts.iter().position(|(f_name, _)| f_name == name) {
            Some(index) => {
                self.fonts.remove(index);
                Ok(())
            }
            None => {
                Err(GMError::FontNotFound(name.to_string()))
            }
        }
    }

    pub fn add_texture(&mut self, name: &str, file: &str, _rows: u32, _cols: u32) -> Result<(), GMError> {
        debug!("GMContext::add_texture(), name: '{}', path: '{}'", name, file);

        todo!();
    }

    pub fn get_texture_rc(&self, name: &str) -> Result<Rc<GMTexture>, GMError> {
        debug!("GMContext::get_texture_rc(), name: '{}'", name);

        todo!();
    }

    pub fn remove_texture(&mut self, name: &str) -> Result<(), GMError> {
        debug!("GMContext::remove_texture(), name: '{}'", name);

        todo!();
    }

    pub fn get_scene_state(&self) -> &GMSceneState {
        &self.scene_state
    }

    pub fn run_scene(&mut self) {
        self.scene_state = GMSceneState::Run;
    }

    pub fn quit_app(&mut self) {
        self.scene_state = GMSceneState::Quit;
    }

    pub fn change_to_scene(&mut self, name: &str) {
        self.scene_state = GMSceneState::ChangeToScene(name.to_string());
    }

    pub fn update(&mut self) -> Result<(), GMError> {
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

        for (_, animation) in self.animations.iter_mut() {
            animation.update();
        }

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
