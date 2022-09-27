
use std::collections::{VecDeque, HashMap, HashSet};

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
use crate::data::GMData;

#[derive(Clone, Debug)]
pub enum GMObjectMessage {
    Base(String, GMData),
    Effect(usize, String, GMData),
}

pub struct GMContext {
    engine_messages: VecDeque<GMEngineMessage>,
    scene_messages: VecDeque<GMSceneManagerMessage>,
    object_messages: HashMap<String, VecDeque<GMObjectMessage>>,
    message_store_index: usize,
    message_get_index: usize,
    canvas: Canvas<Window>,
    input: GMInput,
    resources: GMResources,
    tags: HashMap<String, GMData>,
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
            object_messages: HashMap::new(),
            message_store_index: 0,
            message_get_index: 1,
            canvas,
            input,
            resources,
            tags: HashMap::new(),
            window_width: configuration.screen_width as f32,
            window_height: configuration.screen_height as f32,
        }
    }

    // Resources:
    pub fn resources(&self) -> &GMResources {
        &self.resources
    }

    pub fn resources_mut(&mut self) -> &mut GMResources {
        &mut self.resources
    }

    // Tags:
    pub fn get_tag(&self, name: &str) -> Option<&GMData> {
        self.tags.get(name)
    }

    pub fn set_tag<T: Into<String>>(&mut self, name: T, value: GMData) {
        self.tags.insert(name.into(), value);
    }

    pub fn remove_tag(&mut self, name: &str) {
        self.tags.remove(name);
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

    pub fn change_title<T: Into<String>>(&mut self, title: T) {
        todo!("change_title: '{}'", title.into());
    }


    // Scene messages:
    pub(crate) fn next_scene_message(&mut self) -> Option<GMSceneManagerMessage> {
        self.scene_messages.pop_front()
    }

    pub fn add_scene<T: 'static + GMSceneT, S: Into<String>>(&mut self, name: S, scene: T) {
        self.add_scene2(name.into(), Box::new(scene));
    }

    pub fn add_scene2<S: Into<String>>(&mut self, name: S, scene: Box<dyn GMSceneT>) {
        self.scene_messages.push_back(GMSceneManagerMessage::AddScene(name.into(), scene));
    }

    pub fn remove_scene<S: Into<String>>(&mut self, name: S) {
        self.scene_messages.push_back(GMSceneManagerMessage::RemoveScene(name.into()));
    }

    pub fn change_to_scene<S: Into<String>>(&mut self, name: S) {
        self.scene_messages.push_back(GMSceneManagerMessage::ChangeToScene(name.into()));
    }

    pub fn replace_scene<T: 'static + GMSceneT, S: Into<String>>(&mut self, name: S, scene: T) {
        self.replace_scene2(name.into(), Box::new(scene));
    }

    pub fn replace_scene2<S: Into<String>>(&mut self, name: S, scene: Box<dyn GMSceneT>) {
        self.scene_messages.push_back(GMSceneManagerMessage::ReplaceScene(name.into(), scene));
    }

    pub fn push_and_change_scene<S: Into<String>>(&mut self, name: S) {
        self.scene_messages.push_back(GMSceneManagerMessage::PushAndChangeScene(name.into()));
    }

    pub fn pop_and_change_scene(&mut self) {
        self.scene_messages.push_back(GMSceneManagerMessage::PopAndChangeScene);
    }

    pub fn send_scene_message<S: Into<String>, T: Into<String>>(&mut self, scene: S, message: T) {
        self.scene_messages.push_back(GMSceneManagerMessage::SendMessage(scene.into(), message.into()));
    }

    pub fn send_scene_message_data<S: Into<String>, T: Into<String>>(&mut self, scene: S, message: T, data: GMData) {
        self.scene_messages.push_back(GMSceneManagerMessage::SendMessageData(scene.into(), message.into(), data));
    }

    // Object messages
    pub fn get_object_messages(&mut self, name: &str) -> VecDeque<GMObjectMessage> {
        match self.object_messages.remove(name) {
            Some(message) => {
                message
            }
            None => {
                VecDeque::new()
            }
        }
    }

    pub fn get_group_messages(&mut self, groups: &HashSet<String>) -> VecDeque<GMObjectMessage> {
        if groups.len() == 0 {
            VecDeque::new()
        } else {
            // TODO: filter messaages by group
            todo!();
        }
    }

    pub fn send_object_message<S: Into<String>>(&mut self, receiver: S, message: GMObjectMessage) {
        let receiver = receiver.into();

        match self.object_messages.get_mut(&receiver) {
            Some(messages) => {
                messages.push_back(message)
            }
            None => {
                let mut messages = VecDeque::new();
                messages.push_back(message);
                self.object_messages.insert(receiver, messages);
            }
        }
    }

    pub fn send_group_message<S: Into<String>>(&mut self, receiver: S, message: GMObjectMessage) {
        let receiver = receiver.into();
        // TODO: use a second hashmap
        todo!();
    }

    pub fn send_om_base<S: Into<String>, T: Into<String>>(&mut self, receiver: S, message: T, data: GMData) {
        self.send_object_message(receiver, GMObjectMessage::Base(message.into(), data));
    }

    pub fn send_om_base2<S: Into<String>, T: Into<String>>(&mut self, receiver: S, message: T) {
        self.send_object_message(receiver, GMObjectMessage::Base(message.into(), GMData::None));
    }

    pub fn send_om_group<S: Into<String>, T: Into<String>>(&mut self, receiver: S, message: T, data: GMData) {
        self.send_group_message(receiver, GMObjectMessage::Base(message.into(), data));
    }

    pub fn send_om_group2<S: Into<String>, T: Into<String>>(&mut self, receiver: S, message: T) {
        self.send_group_message(receiver, GMObjectMessage::Base(message.into(), GMData::None));
    }

    pub fn send_om_effect<S: Into<String>, T: Into<String>>(&mut self, receiver: S, index: usize, message: T, data: GMData) {
        self.send_object_message(receiver, GMObjectMessage::Effect(index, message.into(), data));
    }

    pub fn send_om_effect2<S: Into<String>, T: Into<String>>(&mut self, receiver: S, index: usize, message: T) {
        self.send_object_message(receiver, GMObjectMessage::Effect(index, message.into(), GMData::None));
    }

    pub fn send_om_group_effect<S: Into<String>, T: Into<String>>(&mut self, receiver: S, index: usize, message: T, data: GMData) {
        self.send_group_message(receiver, GMObjectMessage::Effect(index, message.into(), data));
    }

    pub fn send_om_group_effect2<S: Into<String>, T: Into<String>>(&mut self, receiver: S, index: usize, message: T) {
        self.send_group_message(receiver, GMObjectMessage::Effect(index, message.into(), GMData::None));
    }

    // Update context, called by engine
    pub(crate) fn update(&mut self) {
        self.input.update();

        // Swap store and get index for messages:
        self.message_store_index = 1 - self.message_store_index;
        // Get index must be different than store index.
        self.message_get_index = 1 - self.message_store_index;
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
