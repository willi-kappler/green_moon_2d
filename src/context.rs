

use std::collections::VecDeque;

use sdl2::video::{self, Window, WindowContext};
use sdl2::render::{TextureCreator, Canvas, Texture};
use sdl2::pixels;
use sdl2::rect::Rect;

use log::debug;

use crate::GMSceneT;
// use crate::animation::GMAnimationT;
use crate::error::GMError;
use crate::object::GMObjectT;
// use crate::math::GMVec2D;
use crate::resources::GMResources;
use crate::input::GMInput;
use crate::message::{GMSender, GMReceiver, GMMessage, GMMessageData};
// use crate::scene::GMSceneT;

pub struct GMUpdateContext {
    engine_messages: VecDeque<GMMessage>,
    scene_messages: VecDeque<GMMessage>,
    object_messages: VecDeque<GMMessage>,
    current_sender: GMSender,
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
            object_messages: VecDeque::new(),
            current_sender: GMSender::Engine,
            input,
            resources,
        }
    }

    pub(crate) fn set_current_sender(&mut self, sender: GMSender) {
        self.current_sender = sender;
    }

    pub(crate) fn get_current_sender(&self) -> GMSender {
        self.current_sender.clone()
    }


    // Engine messages:
    pub(crate) fn next_engine_message(&mut self) -> Option<GMMessage> {
        self.engine_messages.pop_front()
    }

    fn message_to_engine(&mut self, message_data: GMMessageData) {
        self.engine_messages.push_back(GMMessage {
            sender: self.current_sender.clone(),
            receiver: GMReceiver::Engine,
            message_data
        });
    }

    pub fn quit(&mut self) {
        self.message_to_engine(GMMessageData::Quit);
    }

    pub fn change_fps(&mut self, fps: u32) {
        self.message_to_engine(GMMessageData::ChangeFPS(fps));
    }

    pub fn change_resolution(&mut self, width: u32, height: u32) {
        self.message_to_engine(GMMessageData::ChangeResolution(width, height));
    }

    pub fn change_title(&mut self, title: &str) {
        self.message_to_engine(GMMessageData::ChangeTitle(title.to_string()));
    }


    // Scene messages:
    pub(crate) fn next_scene_message(&mut self) -> Option<GMMessage> {
        self.scene_messages.pop_front()
    }

    fn message_to_scene_manager(&mut self, message_data: GMMessageData) {
        self.scene_messages.push_back(GMMessage {
            sender: self.current_sender.clone(),
            receiver: GMReceiver::SceneManager,
            message_data
        });
    }

    pub fn add_scene(&mut self, name: &str, scene: Box<dyn GMSceneT>) {
        self.message_to_scene_manager(GMMessageData::AddScene(name.to_string(), scene));
    }

    pub fn remove_scene(&mut self, name: &str) {
        self.message_to_scene_manager(GMMessageData::RemoveScene(name.to_string()));
    }

    pub fn change_to_scene(&mut self, name: &str) {
        self.message_to_scene_manager(GMMessageData::ChangeToScene(name.to_string()));
    }

    pub fn replace_scene(&mut self, name: &str, scene: Box<dyn GMSceneT>) {
        self.message_to_scene_manager(GMMessageData::ReplaceScene(name.to_string(), scene));
    }

    pub fn push_current_scene(&mut self) {
        self.message_to_scene_manager(GMMessageData::PushCurrentScene);
    }

    pub fn pop_current_scene(&mut self) {
        self.message_to_scene_manager(GMMessageData::PopCurrentScene);
    }

    // TODO: SetSceneParent, RemoveSceneParent, SetSceneChild, RemoveSceneChild, TakeSceneChild






    // Object manager messages:
    pub(crate) fn next_object_message(&mut self) -> Option<GMMessage> {
        self.object_messages.pop_front()
    }

    fn message_to_object_manager(&mut self, message_data: GMMessageData) {
        self.object_messages.push_back(GMMessage {
            sender: self.current_sender.clone(),
            receiver: GMReceiver::ObjectManager,
            message_data
        });
    }

    pub fn add_object(&mut self, name: &str, object: Box<dyn GMObjectT>) {
        self.message_to_object_manager(GMMessageData::AddObject(name.to_string(), object));
    }

    pub fn replace_object(&mut self, name: &str, object: Box<dyn GMObjectT>) {
        self.message_to_object_manager(GMMessageData::ReplaceObject(name.to_string(), object));
    }

    pub fn remove_object(&mut self, name: &str) {
        self.message_to_object_manager(GMMessageData::RemoveObject(name.to_string()));
    }

    pub fn take_object(&mut self, name: &str) {
        self.message_to_object_manager(GMMessageData::TakeObject(name.to_string()));
    }

    // TODO: SetObjectParent, RemoveObjectParent, SetObjectChild, RemoveObjectChild, TakeObjectChild








    // General messages:
    pub fn send_message(&mut self, mut message: GMMessage) {
        use GMReceiver::*;

        let receiver = &message.receiver;
        let sender = &message.sender;

        // Change sender if message comes from current scene or current object
        match sender {
            GMSender::CurrentScene => {
                if matches!(self.current_sender, GMSender::Scene(_)) {
                    message.sender = self.current_sender.clone();
                } else {
                    panic!("GMUpdateContext::send_message(), sender for current scene does not match: {:?}", self.current_sender);
                }
            }
            GMSender::CurrentObject => {
                if matches!(self.current_sender, GMSender::Object(_)) {
                    message.sender = self.current_sender.clone();
                } else {
                    panic!("GMUpdateContext::send_message(), sender for current object does not match: {:?}", self.current_sender);
                }
            }
            _ => {
                // Nothing to do for other senders...
            }
        }

        match receiver {
            Engine => {
                self.engine_messages.push_back(message);
            }
            CurrentScene | Scene(_) | SceneWithProperty(_) | SceneManager => {
                self.scene_messages.push_back(message);
            }
            Object(_) | ObjectWithProperty(_) | ObjectManager => {
                self.object_messages.push_back(message);
            }
            _ => {
                panic!("GMUpdateContext::send_message(), unknown receiver in context: {:?}", receiver);
            }
        }
    }

    // Update context
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

