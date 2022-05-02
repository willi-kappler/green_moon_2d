
// use std::any::Any;

use crate::GMUpdateContext;
use crate::math::GMVec2D;
// use crate::object::GMObjectT;
use crate::scene::GMSceneT;

#[derive(Clone, Debug)]
pub struct GMMessage {
    pub sender: GMSender,
    pub receiver: GMReceiver,
    pub data: GMMessageData,
}

impl GMMessage {
    pub fn new(sender: GMSender, receiver: GMReceiver, data: GMMessageData) -> Self {
        Self {
            sender,
            receiver,
            data,
        }
    }

    pub fn new_to_scene_manager(data: GMMessageData) -> Self {
        Self {
            sender: GMSender::Unknown,
            receiver: GMReceiver::SceneManager,
            data,
        }
    }

    pub fn new_to_engine(data: GMMessageData) -> Self {
        Self {
            sender: GMSender::Unknown,
            receiver: GMReceiver::Engine,
            data,
        }
    }
}

pub struct GMMessageFactory {
    sender: GMSender,
    receiver: GMReceiver,
}

impl GMMessageFactory {
    pub fn new(sender: GMSender, receiver: GMReceiver) -> Self {
        Self {
            sender,
            receiver,
        }
    }

    pub fn send(&self, data: GMMessageData, context: &mut GMUpdateContext) {
        context.send_message(self.create(data));
    }

    pub fn create(&self, data: GMMessageData) -> GMMessage {
        GMMessage::new(
            self.sender.clone(),
            self.receiver.clone(),
            data,
        )
    }

    pub fn send_to(&self, receiver: GMReceiver, data: GMMessageData, context: &mut GMUpdateContext) {
        context.send_message(self.create_to(receiver, data))
    }

    pub fn create_to(&self, receiver: GMReceiver, data: GMMessageData) -> GMMessage {
        GMMessage::new(
            self.sender.clone(),
            receiver,
            data,
        )
    }
}

#[derive(Clone, Debug)]
pub enum GMSender {
    Unknown,

    Engine,

    SceneModifier(String),
    Scene(String),
    CurrentScene,

    ObjectModifier(String),
    Object(String),
}

#[derive(Clone, Debug)]
pub enum GMReceiver {
    Engine,

    CurrentScene,
    Scene(String),
    SceneGroup(String),
    SceneModifier(String),
    SceneManager,

    Object(String),
    ObjectGroup(String),
    ObjectModifier(String),
    ObjectManager,
}

#[derive(Clone, Debug)]
pub enum GMMessageData {
    // Engine messages
    Quit,
    ChangeFPS(u32),

    // Scene messages
    InitScene,
    ExitScene,
    AddScene(Box<dyn GMSceneT>),
    RemoveScene(String),
    ChangeToScene(String),
    ReplaceScene(Box<dyn GMSceneT>),
    PushCurrentScene,
    PopCurrentScene,

    // Other messages
    SetActive(bool),
    GetActive,
    Active(bool),

    SetZIndex(i32),
    GetZIndex,
    ZIndex(i32),

    SetPosition(GMVec2D),
    AddPosition(GMVec2D),
    GetPosition,
    Position(GMVec2D),

    // Custom(Box<dyn Any>),
}
