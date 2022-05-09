
use std::rc::Rc;

use log::{debug};

use crate::animation::GMAnimationT;
use crate::context::GMUpdateContext;
use crate::math::GMVec2D;
use crate::object::GMObjectT;
use crate::scene::GMSceneT;
use crate::texture::GMTexture;
use crate::property::GMValue;
use crate::font::GMFontT;
// use crate::error::GMError;


#[derive(Clone, Debug)]
pub struct GMMessage {
    pub sender: GMSender,
    pub receiver: GMReceiver,
    pub message_data: GMMessageData,
}

impl GMMessage {
    pub fn new(sender: GMSender, receiver: GMReceiver, data: GMMessageData) -> Self {
        Self {
            sender,
            receiver,
            message_data: data,
        }
    }

    pub fn sender2receiver(sender: &GMSender) -> GMReceiver {
        match sender {
            GMSender::Engine => {
                GMReceiver::Engine
            }
            GMSender::CurrentScene => {
                GMReceiver::CurrentScene
            }
            GMSender::Scene(name) => {
                GMReceiver::Scene(name.to_string())
            }
            /*
            GMSender::SceneModifier(name) => {
                GMReceiver::SceneModifier(name.to_string())
            }
            */
            GMSender::SceneManager => {
                GMReceiver::SceneManager
            }
            GMSender::Object(name) => {
                GMReceiver::Object(name.to_string())
            }
            /*
            GMSender::ObjectModifier(name) => {
                GMReceiver::ObjectModifier(name.to_string())
            }
            */
            GMSender::ObjectManager => {
                GMReceiver::ObjectManager
            }

        }
    }
}

#[derive(Debug, Clone)]
pub struct GMMessageFactory {
    sender: GMSender,
    receiver: Option<GMReceiver>,
}

impl GMMessageFactory {
    pub fn new(sender: GMSender) -> Self {
        Self {
            sender,
            receiver: None,
        }
    }

    pub fn set_receiver(&mut self, receiver: GMReceiver) {
        self.receiver = Some(receiver);
    }

    pub fn sender_from_object(&mut self, sender: &Box<dyn GMObjectT>) {
        let name = sender.get_name();
        self.sender = GMSender::Object(name.to_string());
    }

    pub fn send(&self, message_data: GMMessageData, context: &mut GMUpdateContext) {
        context.send_message(self.create(message_data));
    }

    pub fn send_to(&self, receiver: GMReceiver, message_data: GMMessageData, context: &mut GMUpdateContext) {
        context.send_message(self.create_to(receiver, message_data))
    }

    pub fn reply(&self, message: &GMMessage, message_data: GMMessageData, context: &mut GMUpdateContext) {
        let receiver = GMMessage::sender2receiver(&message.sender);
        self.send_to(receiver, message_data, context)
    }

    pub fn create(&self, message_data: GMMessageData) -> GMMessage {
        GMMessage::new(
            self.sender.clone(),
            self.receiver.clone().unwrap(),
            message_data,
        )
    }

    pub fn create_to(&self, receiver: GMReceiver, message_data: GMMessageData) -> GMMessage {
        GMMessage::new(
            self.sender.clone(),
            receiver,
            message_data,
        )
    }

    pub fn create_reply(&self, message: &GMMessage, message_data: GMMessageData) -> GMMessage {
        let receiver = GMMessage::sender2receiver(&message.sender);
        self.create_to(receiver, message_data)
    }

    pub fn create_to_engine(&self, message_data: GMMessageData) -> GMMessage {
        self.create_to(GMReceiver::Engine, message_data)
    }

    pub fn create_to_scene_manager(&self, message_data: GMMessageData) -> GMMessage {
        self.create_to(GMReceiver::SceneManager, message_data)
    }

    pub fn create_to_scene(&self, scene: &str, message_data: GMMessageData) -> GMMessage {
        self.create_to(GMReceiver::Scene(scene.to_string()), message_data)
    }

    pub fn create_to_current_scene(&self, message_data: GMMessageData) -> GMMessage {
        self.create_to(GMReceiver::CurrentScene, message_data)
    }

    pub fn create_to_object_manager(&self, message_data: GMMessageData) -> GMMessage {
        self.create_to(GMReceiver::ObjectManager, message_data)
    }

    pub fn create_to_object(&self, object: &str, message_data: GMMessageData) -> GMMessage {
        self.create_to(GMReceiver::Object(object.to_string()), message_data)
    }


    // Engine messages:

    pub fn engine_quit(&self, context: &mut GMUpdateContext) {
        debug!("GMMessageFactory::engine_quit()");
        context.send_message(self.create_to_engine(GMMessageData::Quit));
    }

    pub fn engine_change_fps(&self, new_fps: u32, context: &mut GMUpdateContext) {
        debug!("GMMessageFactory::engine_change_fps(), new_fps: '{}'", new_fps);
        context.send_message(self.create_to_engine(GMMessageData::ChangeFPS(new_fps)));
    }


    // Scene manager messages:

    pub fn scene_add<S: 'static + GMSceneT>(&self, scene: S, context: &mut GMUpdateContext) {
        debug!("GMMessageFactory::add_scene(), name: '{}'", scene.get_name());
        self.scene_add_box(Box::new(scene), context);
    }

    pub fn scene_add_box(&self, scene: Box<dyn GMSceneT>, context: &mut GMUpdateContext) {
        debug!("GMMessageFactory::add_scene_box(), name: '{}'", scene.get_name());
        context.send_message(self.create_to_scene_manager(GMMessageData::AddScene(scene)));
    }

    pub fn scene_remove(&self, name: &str, context: &mut GMUpdateContext) {
        debug!("GMMessageFactory::remove_scene(), name: '{}'", name);
        context.send_message(self.create_to_scene_manager(GMMessageData::RemoveScene(name.to_string())));
    }

    pub fn scene_change(&self, name: &str, context: &mut GMUpdateContext) {
        debug!("GMMessageFactory::change_scene(), name: '{}'", name);
        context.send_message(self.create_to_scene_manager(GMMessageData::ChangeToScene(name.to_string())));
    }

    pub fn replace_replace<S: 'static + GMSceneT>(&self, scene: S, context: &mut GMUpdateContext) {
        debug!("GMMessageFactory::replace_scene(), name: '{}'", scene.get_name());
        self.scene_replace_box(Box::new(scene), context);
    }

    pub fn scene_replace_box(&self, scene: Box<dyn GMSceneT>, context: &mut GMUpdateContext) {
        debug!("GMMessageFactory::replace_scene_box(), name: '{}'", scene.get_name());
        context.send_message(self.create_to_scene_manager(GMMessageData::ReplaceScene(scene)));
    }

    pub fn scene_push(&self, context: &mut GMUpdateContext) {
        debug!("GMMessageFactory::push_scene()");
        context.send_message(self.create_to_scene_manager(GMMessageData::PushCurrentScene));
    }

    pub fn scene_pop(&self, context: &mut GMUpdateContext) {
        debug!("GMMessageFactory::pop_scene()");
        context.send_message(self.create_to_scene_manager(GMMessageData::PopCurrentScene));
    }




}




#[derive(Clone, Debug)]
pub enum GMSender {
    Engine,

    CurrentScene,
    Scene(String),
    // SceneModifier(String), // TODO: maybe add later
    SceneManager,

    Object(String),
    // ObjectModifier(String), // TODO: maybe add later
    ObjectManager,
}

#[derive(Clone, Debug)]
pub enum GMReceiver {
    Engine,

    CurrentScene,
    Scene(String),
    SceneWithProperty(String),
    // SceneModifier(String), // TODO: maybe add later
    SceneManager,

    Object(String),
    ObjectWithProperty(String),
    // ObjectModifier(String), // TODO: maybe add later
    ObjectManager,
}

#[derive(Clone, Debug)]
pub enum GMMessageData {
    // Engine messages
    Quit,
    ChangeFPS(u32),
    ChangeResolution(u32, u32),
    ChangeTitle(String),

    // Scene messages
    InitScene,
    ExitScene,
    AddScene(Box<dyn GMSceneT>),
    RemoveScene(String),
    // TODO: Maybe add TakeScene message
    ChangeToScene(String),
    ReplaceScene(Box<dyn GMSceneT>),
    PushCurrentScene,
    PopCurrentScene,
    SetSceneParent(String, Box<dyn GMSceneT>),
    RemoveSceneParent(String),
    SetSceneChild(String, Box<dyn GMSceneT>),
    RemoveSceneChild(String),
    TakeSceneChild(String),

    // Object manager
    AddObject(Box<dyn GMObjectT>),
    ReplaceObject(Box<dyn GMObjectT>),
    RemoveObject(String),
    TakeObject(String),
    Object(Box<dyn GMObjectT>),
    SetObjectParent(String, Box<dyn GMObjectT>),
    RemoveObjectParent(String),
    SetObjectChild(String, Box<dyn GMObjectT>),
    RemoveObjectChild(String),
    TakeObjectChild(String),

    // Other messages
    SetActive(bool),
    GetActive,
    Active(bool),

    SetChild(Box<dyn GMObjectT>),
    GetChildClone,
    Child(Option<Box<dyn GMObjectT>>),

    SetZIndex(i32),
    GetZIndex,
    ZIndex(i32),

    SetPosition(GMVec2D),
    AddPosition(GMVec2D),
    GetPosition,
    Position(GMVec2D),

    SetVelocity(GMVec2D),
    AddVelocity(GMVec2D),
    GetVelocity,
    Velocity(GMVec2D),

    SetAcceleration(GMVec2D),
    AddAcceleration(GMVec2D),
    GetAcceleration,
    Acceleration(GMVec2D),

    SetRadius(f32),
    GetRadius,
    Radius(f32),

    SetAngle(f32),
    GetAngle,
    Angle(f32),

    SetAnimation(Box<dyn GMAnimationT>),
    SetAnimationName(String),
    SetTexture(Rc<GMTexture>),
    SetTextureName(String),
    AnimationDone,

    SetText(String),
    SetFont(Rc<dyn GMFontT>),
    SetFontName(String),

    Property(String, GMValue),
}
