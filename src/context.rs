

use std::collections::VecDeque;
use std::any::Any;

use sdl2::video::{self, Window, WindowContext};
use sdl2::render::{Texture, TextureCreator, Canvas};
use sdl2::pixels;
use sdl2::rect::Rect;

use log::debug;

use crate::resources::GMResources;
use crate::input::{GMInput, GMEventCode};
use crate::scene::{GMSceneT, GMSceneManagerMessage};
use crate::engine::GMEngineMessage;
use crate::configuration::GMConfiguration;


pub struct GMContext {
    engine_messages: VecDeque<GMEngineMessage>,
    scene_messages: VecDeque<GMSceneManagerMessage>,
    canvas: Canvas<Window>,
    input: GMInput,
    resources: GMResources,
    window_width: f32,
    window_height: f32,
}

impl GMContext {
    pub(crate) fn new (texture_creator: TextureCreator<WindowContext>,
            event_pump: sdl2::EventPump, canvas: Canvas<Window>, configuration: &GMConfiguration) -> Self {
        let input = GMInput::new(event_pump);
        let resources = GMResources::new(texture_creator);

        Self {
            engine_messages: VecDeque::new(),
            scene_messages: VecDeque::new(),
            canvas,
            input,
            resources,
            window_width: configuration.screen_width as f32,
            window_height: configuration.screen_height as f32,
        }
    }

    pub fn get_resources(&self) -> &GMResources {
        &self.resources
    }

    pub fn get_mut_resources(&mut self) -> &mut GMResources {
        &mut self.resources
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

    pub fn add_scene(&mut self, name: &str, scene: Box<dyn GMSceneT>) {
        self.scene_messages.push_back(GMSceneManagerMessage::AddScene(name.to_string(), scene));
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

    // Update context, called by engine
    pub(crate) fn update(&mut self) {
        self.input.update();
    }

    // Events, called by user code
    pub fn event(&self, event_code: GMEventCode) -> bool {
        self.input.event(event_code)
    }

    // Draw methods:
    pub fn draw_texture_opt(&mut self, texture: &Texture, src_rect: Rect, dst_rect: Rect, angle: f64, flip_x: bool, flip_y: bool) {
        self.canvas.copy_ex(texture, src_rect, dst_rect, angle, None, flip_x, flip_y)
            .expect("GMContext::draw_texture_opt(), error when drawing texture!");
    }

    // Called by engine
    pub(crate) fn present(&mut self) {
        self.canvas.present();
    }

    pub fn clear_black(&mut self) {
        self.clear(pixels::Color::BLACK);
    }

    pub fn clear(&mut self, clear_color: pixels::Color) {
        self.canvas.set_draw_color(clear_color);
        self.canvas.clear();
    }

    pub fn set_fullscreen(&mut self, fullscreen: bool) {
        debug!("GMContext::set_fullscreen(), fullscreen: '{}'", fullscreen);

        if fullscreen {
            self.canvas.window_mut().set_fullscreen(video::FullscreenType::True)
                .expect("GMContext::set_fullscreen(), could not set fullscreen on");
        } else {
            self.canvas.window_mut().set_fullscreen(video::FullscreenType::Off)
                .expect("GMContext::set_fullscreen(), could not set fullscreen off");
        }
    }

    pub fn window_width(&self) -> f32 {
        self.window_width
    }

    pub fn window_height(&self) -> f32 {
        self.window_height
    }
}
