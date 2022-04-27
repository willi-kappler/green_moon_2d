
// use std::fs::File;
// use std::io::Read;
// use std::rc::Rc;

use std::collections::VecDeque;

use sdl2::video::{self, Window, WindowContext};
use sdl2::render::{TextureCreator, Canvas, Texture};
use sdl2::pixels;
use sdl2::rect::Rect;

//use sdl2::surface::Surface;

use log::debug;

use crate::configuration::GMConfiguration;
use crate::engine::GMEngineMessage;
use crate::error::GMError;
use crate::resources::GMResources;
use crate::scene::{GMSceneT, GMSceneMessage};
use crate::input::GMInput;

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
    engine_messages: VecDeque<GMEngineMessage>,
    scene_messages: VecDeque<GMSceneMessage>,
    pub input: GMInput,
    pub resources: GMResources,
}

impl GMUpdateContext {
    pub(crate) fn new (texture_creator: TextureCreator<WindowContext>, event_pump: sdl2::EventPump) -> Self {
        let input = GMInput::new(event_pump);
        let resources = GMResources::new(texture_creator);

        Self {
            engine_messages: VecDeque::new(),
            scene_messages: VecDeque::new(),
            input,
            resources,
        }
    }

    // Scene messages:
    pub fn add_scene<S: 'static + GMSceneT>(&mut self, scene: S) {
        debug!("GMUpdateContext::add_scene(), name: '{}'", scene.get_name());

        self.scene_messages.push_back(GMSceneMessage::AddScene(Box::new(scene)));
    }

    pub fn add_scene_box(&mut self, scene: Box<dyn GMSceneT>) {
        debug!("GMUpdateContext::add_scene_box(), name: '{}'", scene.get_name());

        self.scene_messages.push_back(GMSceneMessage::AddScene(scene));
    }

    pub fn remove_scene(&mut self, name: &str) {
        debug!("GMUpdateContext::remove_scene(), name: '{}'", name);

        self.scene_messages.push_back(GMSceneMessage::RemoveScene(name.to_string()));
    }

    pub fn change_scene(&mut self, name: &str) {
        debug!("GMUpdateContext::change_scene(), name: '{}'", name);

        self.scene_messages.push_back(GMSceneMessage::ChangeToScene(name.to_string()));
    }

    pub fn replace_scene<S: 'static + GMSceneT>(&mut self, scene: S) {
        debug!("GMUpdateContext::replace_scene(), name: '{}'", scene.get_name());

        self.scene_messages.push_back(GMSceneMessage::ReplaceScene(Box::new(scene)));
    }

    pub fn replace_scene_box(&mut self, scene: Box<dyn GMSceneT>) {
        debug!("GMUpdateContext::replace_scene_box(), name: '{}'", scene.get_name());

        self.scene_messages.push_back(GMSceneMessage::ReplaceScene(scene));
    }

    pub fn push_scene(&mut self) {
        debug!("GMUpdateContext::push_scene()");

        self.scene_messages.push_back(GMSceneMessage::Push);
    }

    pub fn pop_scene(&mut self) {
        debug!("GMUpdateContext::pop_scene()");

        self.scene_messages.push_back(GMSceneMessage::Pop);
    }

    pub(crate) fn next_scene_message(&mut self) -> Option<GMSceneMessage> {
        self.scene_messages.pop_front()
    }

    // Engine messages:
    pub fn quit(&mut self) {
        debug!("GMUpdateContext::quit()");

        self.engine_messages.push_back(GMEngineMessage::Quit);
    }

    pub fn change_fps(&mut self, new_fps: u32) {
        debug!("GMUpdateContext::change_fps(), new_fps: '{}'", new_fps);

        self.engine_messages.push_back(GMEngineMessage::ChangeFPS(new_fps));
    }

    pub(crate) fn next_engine_message(&mut self) -> Option<GMEngineMessage> {
        self.engine_messages.pop_front()
    }

    pub(crate) fn update(&mut self) -> Result<(), GMError> {
        self.input.update();

        Ok(())
    }
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
        self.clear(pixels::Color::BLACK);
    }

    pub fn clear(&mut self, color: pixels::Color) {
        self.canvas.set_draw_color(color);
        self.canvas.clear();
    }

    pub fn set_fullscreen(&mut self, fullscreen: bool) {
        debug!("GMDrawContext::set_fullscreen(), fullscreen: '{}'", fullscreen);

        // TODO: Map SDL2 error
        if fullscreen {
            self.canvas.window_mut().set_fullscreen(video::FullscreenType::True).ok();
        } else {
            self.canvas.window_mut().set_fullscreen(video::FullscreenType::Off).ok();
        }
    }

    pub fn draw_ex(&mut self, texture: &Texture, src_rect: Rect, dst_rect: Rect, angle: f64, flip_x: bool, flip_y: bool) {
        self.canvas.copy_ex(texture, src_rect, dst_rect, angle, None, flip_x, flip_y).unwrap();
    }
}

