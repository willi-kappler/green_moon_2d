

use std::collections::VecDeque;

use sdl2::video::{self, Window, WindowContext};
use sdl2::render::{TextureCreator, Canvas, Texture};
use sdl2::pixels;
use sdl2::rect::Rect;

use log::debug;

use crate::GMSceneT;
// use crate::animation::GMAnimationT;
//use crate::object::GMObjectT;
// use crate::math::GMVec2D;
use crate::resources::GMResources;
use crate::input::GMInput;
use crate::message::{GMEngineMessage, GMSceneManagerMessage, GMObjectManagerMessage};
//use crate::message::{GMSender, GMReceiver, GMMessage, GMMessageData};
// use crate::scene::GMSceneT;

pub struct GMUpdateContext {
    engine_messages: VecDeque<GMEngineMessage>,
    scene_messages: VecDeque<GMSceneManagerMessage>,
    object_messages: VecDeque<GMObjectManagerMessage>,
    mode: GMContextMode,
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
            mode: GMContextMode::Scene("".to_string()),
            input,
            resources,
        }
    }

    pub(crate) fn set_mode_scene(&mut self, name: &str) {
        self.mode = GMContextMode::Scene(name.to_string());
    }

    pub(crate) fn set_mode_object(&mut self, name: &str) {
        self.mode = GMContextMode::Object(name.to_string());
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

    /*
    pub fn change_resolution(&mut self, width: u32, height: u32) {
        self.message_to_engine(GMMessageData::ChangeResolution(width, height));
    }

    pub fn change_title(&mut self, title: &str) {
        self.message_to_engine(GMMessageData::ChangeTitle(title.to_string()));
    }
*/


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



    // Object manager messages:
    pub(crate) fn next_object_message(&mut self) -> Option<GMObjectManagerMessage> {
        self.object_messages.pop_front()
    }

    /*
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

    */







    /*
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
    */


    // Update context
    pub(crate) fn update(&mut self) {
        self.input.update();
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

enum GMContextMode {
    Scene(String),
    Object(String),
}
