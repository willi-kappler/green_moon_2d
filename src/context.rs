

use std::collections::VecDeque;
use std::any::Any;

use sdl2::video::{self, Window, WindowContext};
use sdl2::render::{Texture, TextureCreator, Canvas};
use sdl2::pixels;
use sdl2::rect::Rect;

use log::debug;

use crate::resources::GMResources;
use crate::input::GMInput;
use crate::scene::{GMSceneT, GMSceneManagerMessage};
use crate::engine::GMEngineMessage;


pub struct GMContext {
    engine_messages: VecDeque<GMEngineMessage>,
    scene_messages: VecDeque<GMSceneManagerMessage>,
    canvas: Canvas<Window>,
    pub input: GMInput,
    pub resources: GMResources,
}

impl GMContext {
    pub(crate) fn new (texture_creator: TextureCreator<WindowContext>,
            event_pump: sdl2::EventPump, canvas: Canvas<Window>) -> Self {
        let input = GMInput::new(event_pump);
        let resources = GMResources::new(texture_creator);

        Self {
            engine_messages: VecDeque::new(),
            scene_messages: VecDeque::new(),
            canvas,
            input,
            resources,
        }
    }

    // Engine messages:
    pub(crate) fn next_engine_message(&mut self) -> Option<GMEngineMessage> {
        self.engine_messages.pop_front()
    }

    pub fn quit(&mut self) {
        self.engine_messages.push_back(GMEngineMessage::Quit);
    }

    pub fn change_fps(&mut self, fps: u32) {
        self.engine_messages.push_back(GMEngineMessage::ChangeFPS(fps));
    }

    pub fn change_resolution(&mut self, width: u32, height: u32) {
        todo!("change_resolution: '{}', '{}'", width, height);
    }

    pub fn change_title(&mut self, title: &str) {
        todo!("change_title: '{}'", title);
    }


    // Scene messages:
    pub(crate) fn next_scene_message(&mut self) -> Option<GMSceneManagerMessage> {
        self.scene_messages.pop_front()
    }

    pub fn add_scene(&mut self, scene: Box<dyn GMSceneT>) {
        self.scene_messages.push_back(GMSceneManagerMessage::AddScene(scene));
    }

    pub fn remove_scene(&mut self, name: &str) {
        self.scene_messages.push_back(GMSceneManagerMessage::RemoveScene(name.to_string()));
    }

    pub fn change_to_scene(&mut self, name: &str) {
        self.scene_messages.push_back(GMSceneManagerMessage::ChangeToScene(name.to_string()));
    }

    pub fn replace_scene(&mut self, name: &str, scene: Box<dyn GMSceneT>) {
        self.scene_messages.push_back(GMSceneManagerMessage::ReplaceScene(name.to_string(), scene));
    }

    pub fn push_and_change_scene(&mut self, name: &str) {
        self.scene_messages.push_back(GMSceneManagerMessage::PushAndChangeScene(name.to_string()));
    }

    pub fn pop_and_change_scene(&mut self) {
        self.scene_messages.push_back(GMSceneManagerMessage::PopAndChangeScene);
    }

    pub fn send_message(&mut self, scene: &str, message: &str, data: Option<Box<dyn Any>>) {
        self.scene_messages.push_back(GMSceneManagerMessage::SendMessage(scene.to_string(), message.to_string(), data));
    }

    // Update context
    pub(crate) fn update(&mut self) {
        self.input.update();
    }

    // Draw methods:
    pub fn draw_texture_opt(&mut self, texture: &Texture, src_rect: Rect, dst_rect: Rect, angle: f64, flip_x: bool, flip_y: bool) {
        panic!("TODO: call SDL2 draw texture");
    }

    pub fn present(&mut self) {
        self.canvas.present();
    }

    pub fn clear_black(&mut self) {
        self.clear(pixels::Color::BLACK);
    }

    pub fn clear(&mut self, clear_color: pixels::Color) {
        panic!("TODO: call SDL2 clear screen");
    }

    pub fn set_fullscreen(&mut self, fullscreen: bool) {
        debug!("GMContext::set_fullscreen(), fullscreen: '{}'", fullscreen);

        if fullscreen {
            self.canvas.window_mut().set_fullscreen(video::FullscreenType::True)
                .expect("Could not set fullscreen on");
        } else {
            self.canvas.window_mut().set_fullscreen(video::FullscreenType::Off)
                .expect("Could not set fullscreen off");
        }
    }
}
