

use std::collections::VecDeque;

use sdl2::video::{self, Window, WindowContext};
use sdl2::render::{TextureCreator, Canvas, Texture};
use sdl2::pixels;
use sdl2::rect::Rect;

use log::debug;

use crate::animation::GMAnimationT;
use crate::error::GMError;
use crate::math::GMVec2D;
use crate::resources::GMResources;
use crate::input::GMInput;
use crate::message::{GMSender, GMReceiver, GMMessage, GMMessageData};
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

    // TODO: Properties


    pub(crate) fn next_scene_message(&mut self) -> Option<GMMessage> {
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
        self.engine_messages.pop_front()
    }

    // Object manager messages:




    // Object messages:


    pub fn unknown_to_object(&mut self, name: &str, data: GMMessageData) {
        let sender = GMSender::Unknown;
        let receiver = GMReceiver::Object(name.to_string());
        let message = GMMessage::new(sender, receiver, data);
        self.object_messages.push_back(message);
    }

    pub fn unknown_to_object_group(&mut self, group: &str, data: GMMessageData) {
        let sender = GMSender::Unknown;
        let receiver = GMReceiver::ObjectGroup(group.to_string());
        let message = GMMessage::new(sender, receiver, data);
        self.object_messages.push_back(message);
    }

    pub fn unknown_to_object_manager(&mut self, data: GMMessageData) {
        let sender = GMSender::Unknown;
        let receiver = GMReceiver::ObjectManager;
        let message = GMMessage::new(sender, receiver, data);
        self.object_messages.push_back(message);
    }

    pub fn set_z_index(&mut self, name: &str, z_index: i32) {
        let data = GMMessageData::SetZIndex(z_index);
        self.unknown_to_object(name, data);
    }

    pub fn set_z_index_group(&mut self, group: &str, z_index: i32) {
        let data = GMMessageData::SetZIndex(z_index);
        self.unknown_to_object_group(group, data);
    }

    pub fn set_active(&mut self, name: &str, active: bool) {
        let data = GMMessageData::SetActive(active);
        self.unknown_to_object(name, data);
    }

    pub fn set_position(&mut self, name: &str, position: GMVec2D) {
        let data = GMMessageData::SetPosition(position);
        self.unknown_to_object(name, data);
    }

    pub fn set_position_group(&mut self, group: &str, position: GMVec2D) {
        let data = GMMessageData::SetPosition(position);
        self.unknown_to_object_group(group, data);
    }

    pub fn add_position(&mut self, name: &str, position: GMVec2D) {
        let data = GMMessageData::AddPosition(position);
        self.unknown_to_object(name, data);
    }

    pub fn add_position_group(&mut self, group: &str, position: GMVec2D) {
        let data = GMMessageData::AddPosition(position);
        self.unknown_to_object_group(group, data);
    }

    pub fn set_velocity(&mut self, name: &str, velocity: GMVec2D) {
        let data = GMMessageData::SetVelocity(velocity);
        self.unknown_to_object(name, data);
    }

    pub fn set_velocity_group(&mut self, group: &str, velocity: GMVec2D) {
        let data = GMMessageData::SetVelocity(velocity);
        self.unknown_to_object_group(group, data);
    }

    pub fn add_velocity(&mut self, name: &str, velocity: GMVec2D) {
        let data = GMMessageData::AddVelocity(velocity);
        self.unknown_to_object(name, data);
    }

    pub fn add_velocity_group(&mut self, group: &str, velocity: GMVec2D) {
        let data = GMMessageData::AddVelocity(velocity);
        self.unknown_to_object_group(group, data);
    }

    pub fn set_acceleration(&mut self, name: &str, acceleration: GMVec2D) {
        let data = GMMessageData::SetAcceleration(acceleration);
        self.unknown_to_object(name, data);
    }

    pub fn set_acceleration_group(&mut self, group: &str, acceleration: GMVec2D) {
        let data = GMMessageData::SetAcceleration(acceleration);
        self.unknown_to_object_group(group, data);
    }

    pub fn add_acceleration(&mut self, name: &str, acceleration: GMVec2D) {
        let data = GMMessageData::AddAcceleration(acceleration);
        self.unknown_to_object(name, data);
    }

    pub fn add_acceleration_group(&mut self, group: &str, acceleration: GMVec2D) {
        let data = GMMessageData::AddAcceleration(acceleration);
        self.unknown_to_object_group(group, data);
    }

    pub fn set_animation(&mut self, name: &str, animation: Box<dyn GMAnimationT>) {
        let data = GMMessageData::SetAnimation(animation);
        self.unknown_to_object(name, data);
    }

    pub fn set_animation_group(&mut self, group: &str, animation: Box<dyn GMAnimationT>) {
        let data = GMMessageData::SetAnimation(animation);
        self.unknown_to_object(group, data);
    }






    pub(crate) fn next_object_message(&mut self) -> Option<GMMessage> {
        self.object_messages.pop_front()
    }

    // General messages:

    pub fn send_message(&mut self, message: GMMessage) {
        use GMReceiver::*;

        let receiver = message.receiver.clone();

        match receiver {
            Engine => {
                self.engine_messages.push_back(message);
            }
            CurrentScene | Scene(_) | SceneGroup(_) | SceneModifier(_) | SceneManager => {
                self.scene_messages.push_back(message);
            }
            Object(_) | ObjectGroup(_) | ObjectModifier(_) | ObjectManager => {
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

