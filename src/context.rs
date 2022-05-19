

use std::collections::VecDeque;

use sdl2::video::{self, Window, WindowContext};
use sdl2::render::{TextureCreator, Canvas, Texture};
use sdl2::pixels;
use sdl2::rect::Rect;

use log::debug;

// use crate::animation::GMAnimationT;
// use crate::math::GMVec2D;
use crate::object::GMObjectT;
use crate::resources::GMResources;
use crate::input::GMInput;
use crate::message::{GMEngineMessage, GMSceneManagerMessage, GMSceneMessage, GMObjectManagerMessage, GMObjectMessage, GMMessageReplyTo};
use crate::scene::GMSceneT;

pub struct GMContext {
    engine_messages: VecDeque<GMEngineMessage>,
    scene_messages: VecDeque<GMSceneManagerMessage>,
    object_messages: VecDeque<GMObjectManagerMessage>,
    message_reply: GMMessageReplyTo,
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
            object_messages: VecDeque::new(),
            canvas,
            message_reply: GMMessageReplyTo::Scene,
            input,
            resources,
        }
    }

    pub(crate) fn reply_to_current_scene(&mut self) {
        self.message_reply = GMMessageReplyTo::Scene;
    }

    pub(crate) fn reply_to_object(&mut self, name: &str) {
        self.message_reply = GMMessageReplyTo::Object(name.to_string());
    }

    pub fn get_reply_to(&self) -> &GMMessageReplyTo {
        &self.message_reply
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

    pub fn message_to_current_scene(&mut self, message: GMSceneMessage) {
        self.scene_messages.push_back(GMSceneManagerMessage::MessageToCurrentScene(message));
    }

    pub fn message_to_scene(&mut self, name: &str, message: GMSceneMessage) {
        self.scene_messages.push_back(GMSceneManagerMessage::MessageToScene(name.to_string(), message));
    }


    // Object manager messages:
    pub(crate) fn next_object_message(&mut self) -> Option<GMObjectManagerMessage> {
        self.object_messages.pop_front()
    }

    pub fn add_object(&mut self, name: &str, object: Box<dyn GMObjectT>) {
        self.object_messages.push_back(GMObjectManagerMessage::AddObject(name.to_string(), object));
    }

    pub fn remove_object(&mut self, name: &str) {
        self.object_messages.push_back(GMObjectManagerMessage::RemoveObject(name.to_string()));
    }

    pub fn replace_object(&mut self, name: &str, object: Box<dyn GMObjectT>) {
        self.object_messages.push_back(GMObjectManagerMessage::ReplaceObject(name.to_string(), object));
    }

    pub fn set_object_parent(&mut self, name: &str, object: Box<dyn GMObjectT>) {
        self.object_messages.push_back(GMObjectManagerMessage::SetParent(name.to_string(), object));
    }

    pub fn get_object_clone(&mut self, name: &str) {
        self.object_messages.push_back(GMObjectManagerMessage::GetClone(name.to_string(), self.message_reply.clone()));
    }

    pub fn message_to_object(&mut self, name: &str, message: GMObjectMessage) {
        self.object_messages.push_back(GMObjectManagerMessage::MessageToObject(name.to_string(), message));
    }


    // Update context
    pub(crate) fn update(&mut self) {
        self.input.update();
    }


    // Draw methods:
    pub(crate) fn draw(&mut self) {

        self.canvas.present();

        todo!();
    }

    fn draw_ex(&mut self, texture: &Texture, src_rect: Rect, dst_rect: Rect, angle: f64, flip_x: bool, flip_y: bool) {
        self.canvas.copy_ex(texture, src_rect, dst_rect, angle, None, flip_x, flip_y).unwrap();
    }

    fn clear_black(&mut self) {
        self.clear(pixels::Color::BLACK);
    }

    fn clear(&mut self, color: pixels::Color) {
        self.canvas.set_draw_color(color);
        self.canvas.clear();
    }

    fn set_fullscreen(&mut self, fullscreen: bool) {
        debug!("GMContext::set_fullscreen(), fullscreen: '{}'", fullscreen);

        // TODO: Map SDL2 error
        if fullscreen {
            self.canvas.window_mut().set_fullscreen(video::FullscreenType::True)
                .expect("Could not set fullscreen on");
        } else {
            self.canvas.window_mut().set_fullscreen(video::FullscreenType::Off)
                .expect("Could not set fullscreen off");
        }
    }
}
