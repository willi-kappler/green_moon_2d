

use std::collections::VecDeque;

use sdl2::video::{self, Window, WindowContext};
use sdl2::render::{TextureCreator, Canvas, Texture};
use sdl2::pixels;
use sdl2::rect::Rect;

use log::debug;

use crate::error::GMError;
use crate::resources::GMResources;
use crate::input::GMInput;
use crate::message::{GMReceiver, GMMessage, GMMessageData};
use crate::scene::GMSceneT;

pub struct GMUpdateContext {
    engine_messages: VecDeque<GMMessage>,
    scene_messages: VecDeque<GMMessage>,
    object_messages: VecDeque<GMMessage>,
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
            input,
            resources,
        }
    }

    // Scene messages:
    pub fn add_scene<S: 'static + GMSceneT>(&mut self, scene: S) {
        debug!("GMUpdateContext::add_scene(), name: '{}'", scene.get_name());

        let message = GMMessage::new_to_scene_manager(GMMessageData::AddScene(Box::new(scene)));
        self.scene_messages.push_back(message);
    }

    pub fn add_scene_box(&mut self, scene: Box<dyn GMSceneT>) {
        debug!("GMUpdateContext::add_scene_box(), name: '{}'", scene.get_name());

        let message = GMMessage::new_to_scene_manager(GMMessageData::AddScene(scene));
        self.scene_messages.push_back(message);
    }

    pub fn remove_scene(&mut self, name: &str) {
        debug!("GMUpdateContext::remove_scene(), name: '{}'", name);

        let message = GMMessage::new_to_scene_manager(GMMessageData::RemoveScene(name.to_string()));
        self.scene_messages.push_back(message);
    }

    pub fn change_scene(&mut self, name: &str) {
        debug!("GMUpdateContext::change_scene(), name: '{}'", name);

        let message = GMMessage::new_to_scene_manager(GMMessageData::ChangeToScene(name.to_string()));
        self.scene_messages.push_back(message);
    }

    pub fn replace_scene<S: 'static + GMSceneT>(&mut self, scene: S) {
        debug!("GMUpdateContext::replace_scene(), name: '{}'", scene.get_name());

        let message = GMMessage::new_to_scene_manager(GMMessageData::ReplaceScene(Box::new(scene)));
        self.scene_messages.push_back(message);
    }

    pub fn replace_scene_box(&mut self, scene: Box<dyn GMSceneT>) {
        debug!("GMUpdateContext::replace_scene_box(), name: '{}'", scene.get_name());

        let message = GMMessage::new_to_scene_manager(GMMessageData::ReplaceScene(scene));
        self.scene_messages.push_back(message);
    }

    pub fn push_scene(&mut self) {
        debug!("GMUpdateContext::push_scene()");

        let message = GMMessage::new_to_scene_manager(GMMessageData::PushCurrentScene);
        self.scene_messages.push_back(message);
    }

    pub fn pop_scene(&mut self) {
        debug!("GMUpdateContext::pop_scene()");

        let message = GMMessage::new_to_scene_manager(GMMessageData::PopCurrentScene);
        self.scene_messages.push_back(message);
    }

    pub(crate) fn next_scene_message(&mut self) -> Option<GMMessage> {
        debug!("GMUpdateContext::next_scene_message()");

        self.scene_messages.pop_front()
    }

    // Engine messages:
    pub fn quit(&mut self) {
        debug!("GMUpdateContext::quit()");

        let message = GMMessage::new_to_engine(GMMessageData::Quit);
        self.engine_messages.push_back(message);
    }

    pub fn change_fps(&mut self, new_fps: u32) {
        debug!("GMUpdateContext::change_fps(), new_fps: '{}'", new_fps);

        let message = GMMessage::new_to_engine(GMMessageData::ChangeFPS(new_fps));
        self.engine_messages.push_back(message);
    }

    pub(crate) fn next_engine_message(&mut self) -> Option<GMMessage> {
        debug!("GMUpdateContext::next_engine_message()");

        self.engine_messages.pop_front()
    }

    // Object messages:

    // TODO:
    // set_z_index, set_z_index_group, set_position, add_position, set_velocity, add_velocity, set_acceleration, add_acceleration, ...

    pub(crate) fn next_object_message(&mut self) -> Option<GMMessage> {
        self.object_messages.pop_front()
    }

    // Messages to anything:

    pub fn send_message(&mut self, message: GMMessage) {
        use GMReceiver::*;

        let receiver = message.receiver.clone();

        match receiver {
            Engine => {
                self.engine_messages.push_back(message);
            }
            CurrentScene | Scene(_) | SceneGroup(_) | SceneManager => {
                self.scene_messages.push_back(message);
            }
            Object(_) | ObjectGroup(_) | ObjectManager => {
                self.object_messages.push_back(message);
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

