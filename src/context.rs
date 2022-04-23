
// use std::fs::File;
// use std::io::Read;
// use std::rc::Rc;

use std::collections::VecDeque;

use sdl2::video::{self, Window, WindowContext};
use sdl2::render::{TextureCreator, Canvas};
use sdl2::keyboard::Keycode;
use sdl2::event::Event;
use sdl2::pixels;
//use sdl2::surface::Surface;

use log::debug;

// use crate::animation::GMAnimationT;
// use crate::assets::GMAssets;
use crate::configuration::GMConfiguration;
use crate::engine::GMEngineMessage;
use crate::error::GMError;
// use crate::font::GMFontT;
use crate::scene::{GMSceneT, GMSceneMessage};
//use crate::texture::GMTexture;


pub(crate) fn create_context(configuration: &GMConfiguration) -> (GMUpdateContext, GMDrawContext) {
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

    let update_context = GMUpdateContext::new(texture_creator, event_pump);
    let draw_context = GMDrawContext::new(canvas);

    (update_context, draw_context)
}

pub struct GMUpdateContext {
    texture_creator: TextureCreator<WindowContext>,
    event_pump: sdl2::EventPump,
    engine_messages: VecDeque<GMEngineMessage>,
    scene_messages: VecDeque<GMSceneMessage>,
}

impl GMUpdateContext {
    pub(crate) fn new (texture_creator: TextureCreator<WindowContext>, event_pump: sdl2::EventPump) -> Self {
        Self {
            texture_creator,
            event_pump,
            engine_messages: VecDeque::new(),
            scene_messages: VecDeque::new(),

        }
    }

    pub fn add_scene<S: 'static + GMSceneT>(&mut self, scene: S) {
        debug!("GMContext::add_scene(), name: '{}'", scene.get_name());

        self.scene_messages.push_back(GMSceneMessage::AddScene(Box::new(scene)));
    }

    pub fn remove_scene(&mut self, name: &str) {
        debug!("GMContext::remove_scene(), name: '{}'", name);

        self.scene_messages.push_back(GMSceneMessage::RemoveScene(name.to_string()));
    }

    pub fn change_scene(&mut self, name: &str) {
        debug!("GMContext::change_scene(), name: '{}'", name);

        self.scene_messages.push_back(GMSceneMessage::ChangeToScene(name.to_string()));
    }

    pub fn replace_scene<S: 'static + GMSceneT>(&mut self, scene: S) {
        debug!("GMContext::replace_scene(), name: '{}'", scene.get_name());

        self.scene_messages.push_back(GMSceneMessage::ReplaceScene(Box::new(scene)));
    }

    pub(crate) fn next_scene_message(&mut self) -> Option<GMSceneMessage> {
        self.scene_messages.pop_front()
    }

    pub fn quit(&mut self) {
        debug!("GMContext::quit()");

        self.engine_messages.push_back(GMEngineMessage::Quit);
    }

    pub fn change_fps(&mut self, new_fps: u32) {
        debug!("GMContext::change_fps(), new_fps: '{}'", new_fps);

        self.engine_messages.push_back(GMEngineMessage::ChangeFPS(new_fps));
    }

    pub(crate) fn next_engine_message(&mut self) -> Option<GMEngineMessage> {
        self.engine_messages.pop_front()
    }


    pub fn update(&mut self) -> Result<(), GMError> {
/*
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
*/
        Ok(())
    }

/*
    pub fn key_esc_down(&self) -> bool {
        self.key_esc_down_
    }

    pub fn key_esc_up(&self) -> bool {
        self.key_esc_up_
    }
*/
}

pub struct GMDrawContext {
    canvas: Canvas<Window>,

}

impl GMDrawContext {
    pub(crate) fn new(canvas: Canvas<Window>) -> Self {
        Self {
            canvas,
        }
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
        debug!("GMContext::set_fullscreen(), fullscreen: '{}'", fullscreen);

        // TODO: Map SDL2 error
        if fullscreen {
            self.canvas.window_mut().set_fullscreen(video::FullscreenType::True).ok();
        } else {
            self.canvas.window_mut().set_fullscreen(video::FullscreenType::Off).ok();
        }
    }
}

