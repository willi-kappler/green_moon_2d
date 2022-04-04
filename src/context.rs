
use std::fs::File;
use std::io::Read;
use std::rc::Rc;

use sdl2::video;
use sdl2::render;
use sdl2::keyboard::Keycode;
use sdl2::event::Event;
use sdl2::pixels;

use log::debug;

use crate::animation::GMAnimationT;
use crate::assets::GMAssets;
use crate::configuration::GMConfiguration;
use crate::draw_object::GMDrawT;
use crate::error::GMError;
use crate::font::GMFontT;
use crate::sprite::GMSprite;
use crate::text::GMText;
use crate::texture::GMTexture;


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
    new_fps: u32,
    scene_state: GMSceneState,
    canvas: render::Canvas<video::Window>,
    event_pump: sdl2::EventPump,

    // Name, Object
    animations: Vec<(String, Box<dyn GMAnimationT>)>,
    draw_objects: Vec<(String, Box<dyn GMDrawT>)>,
    fonts: Vec<(String, Rc<dyn GMFontT>)>,
    textures: Vec<Rc<GMTexture>>,

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

            animations: Vec::new(),
            draw_objects: Vec::new(),
            fonts: Vec::new(),
            textures: Vec::new(),

            key_esc_down_: false,
            key_esc_up_: false,
        }
    }

    pub fn set_fps(&mut self, new_fps: u32) {
        self.new_fps = new_fps;
    }

    pub fn get_fps(&self) -> u32 {
        self.new_fps
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
            self.add_animation(&animation.name, &animation.frames, animation.animation_type)?;
        }

        for font in all_assets.fonts.iter() {
            self.add_font(&font.name, &font.texture, &font.mapping)?;
        }

        Ok(())
    }

    pub fn add_animation(&mut self, name: &str, _frames: &[(usize, f32)], _animation_type: u8) -> Result<(), GMError> {
        debug!("GMContext::add_animation(), name: '{}'", name);

        todo!();
    }

    pub fn get_animation_clone(&self, name: &str) -> Result<Box<dyn GMAnimationT>, GMError> {
        debug!("GMContext::get_animation_clone(), name: '{}'", name);

        todo!();
    }

    pub fn remove_animation(&mut self, name: &str) -> Result<(), GMError> {
        debug!("GMContext::remove_animation(), name: '{}'", name);

        todo!();
    }

    pub fn add_draw_object(&mut self, name: &str, object: Box<dyn GMDrawT>) -> Result<(), GMError> {
        debug!("GMContext::add_animation(), name: '{}'", name);

        for (o_name, _) in self.draw_objects.iter() {
            if o_name == name {
                return Err(GMError::ObjectAlreadyExists(name.to_string()))
            }
        }

        self.draw_objects.push((name.to_string(), object));

        Ok(())
    }

    pub fn get_draw_object(&self, name: &str) -> Result<&Box<dyn GMDrawT>, GMError> {
        debug!("GMContext::get_draw_object(), name: '{}'", name);

        for (o_name, object) in self.draw_objects.iter() {
            if o_name == name {
                return Ok(object)
            }
        }

        Err(GMError::DrawObjectNotFound(name.to_string()))
    }

    pub fn get_draw_object_mut(&mut self, name: &str) -> Result<&mut Box<dyn GMDrawT>, GMError> {
        debug!("GMContext::get_draw_object_mut(), name: '{}'", name);

        for (o_name, object) in self.draw_objects.iter_mut() {
            if o_name == name {
                return Ok(object)
            }
        }

        Err(GMError::DrawObjectNotFound(name.to_string()))
    }

    pub fn get_draw_object_clone(&self, name: &str) -> Result<Box<dyn GMDrawT>, GMError> {
        debug!("GMContext::get_draw_object_clone(), name: '{}'", name);

        for (o_name, object) in self.draw_objects.iter() {
            if o_name == name {
                return Ok(object.box_clone())
            }
        }

        Err(GMError::DrawObjectNotFound(name.to_string()))
    }

    pub fn remove_draw_object(&mut self, name: &str) -> Result<(), GMError> {
        debug!("GMContext::remove_draw_object(), name: '{}'", name);

        for (o_name, _) in self.draw_objects.iter() {
            if o_name == name {
                todo!();
            }
        }

        Err(GMError::DrawObjectNotFound(name.to_string()))
    }

    pub fn add_font(&mut self, name: &str, texture: &str, mapping: &str) -> Result<(), GMError> {
        debug!("GMContext::add_font(), name: '{}', texture: '{}', mapping: '{}'", name, texture, mapping);

        todo!();
    }

    pub fn get_font_rc(&self, name: &str) -> Result<Rc<dyn GMFontT>, GMError> {
        debug!("GMContext::get_font_rc(), name: '{}'", name);

        todo!();
    }

    pub fn remove_font(&mut self, name: &str) -> Result<(), GMError> {
        debug!("GMContext::remove_font(), name: '{}'", name);

        todo!();
    }

    pub fn add_sprite(&mut self, name: &str) -> Result<(), GMError> {
        debug!("GMContext::add_sprite(), name: '{}'", name);

        todo!();
    }

    pub fn get_sprite(&self, name: &str) -> Result<&GMSprite, GMError> {
        debug!("GMContext::get_sprite(), name: '{}'", name);

        todo!();
    }

    pub fn get_sprite_mut(&mut self, name: &str) -> Result<&mut GMSprite, GMError> {
        debug!("GMContext::get_sprite_mut(), name: '{}'", name);

        todo!();
    }

    pub fn get_sprite_clone(&self, name: &str) -> Result<GMSprite, GMError> {
        debug!("GMContext::get_sprite_clone(), name: '{}'", name);

        todo!();
    }

    pub fn remove_sprite(&mut self, name: &str) -> Result<(), GMError> {
        debug!("GMContext::remove_sprite(), name: '{}'", name);

        todo!();
    }

    pub fn set_sprite_animation(&mut self, sprite: &str, animation: &str) -> Result<(), GMError> {
        debug!("GMContext::set_sprite_animation(), sprite: '{}', animation: '{}'", sprite, animation);

        todo!();
    }

    pub fn add_text(&mut self, name: &str) -> Result<(), GMError> {
        debug!("GMContext::add_text(), name: '{}'", name);

        todo!();
    }

    pub fn get_text(&mut self, name: &str) -> Result<&GMText, GMError> {
        debug!("GMContext::get_text(), name: '{}'", name);

        todo!();
    }

    pub fn get_text_mut(&mut self, name: &str) -> Result<&mut GMText, GMError> {
        debug!("GMContext::get_text_mut(), name: '{}'", name);

        todo!();
    }

    pub fn get_text_clone(&mut self, name: &str) -> Result<GMText, GMError> {
        debug!("GMContext::get_text_clone(), name: '{}'", name);

        todo!();
    }

    pub fn remove_text(&mut self, name: &str) -> Result<(), GMError> {
        debug!("GMContext::remove_text(), name: '{}'", name);

        todo!();
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

        for (_, animation) in self.animations.iter_mut() {
            animation.update();
        }

        for (_, object) in self.draw_objects.iter_mut() {
            object.update();
        }

        Ok(())
    }

    pub(crate) fn draw(&mut self) -> Result<(), GMError> {
        // Sort all drawable objects by z order before drawing them
        self.draw_objects.sort_by_key(|(_, object)| object.get_z_index());

        for (_, object) in self.draw_objects.iter() {
            object.draw();
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
