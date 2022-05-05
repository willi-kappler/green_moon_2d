
use std::rc::Rc;

use crate::animation::GMAnimationT;
use crate::context::GMUpdateContext;
use crate::math::GMVec2D;
use crate::object::GMObjectT;
use crate::scene::GMSceneT;
use crate::texture::GMTexture;
use crate::property::GMValue;
use crate::font::GMFontT;
use crate::error::GMError;


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

    pub fn new_to_engine(data: GMMessageData) -> Self {
        Self {
            sender: GMSender::Unknown,
            receiver: GMReceiver::Engine,
            message_data: data,
        }
    }

    pub fn new_to_scene_manager(data: GMMessageData) -> Self {
        Self {
            sender: GMSender::Unknown,
            receiver: GMReceiver::SceneManager,
            message_data: data,
        }
    }

    pub fn new_to_object_manager(data: GMMessageData) -> Self {
        Self {
            sender: GMSender::Unknown,
            receiver: GMReceiver::ObjectManager,
            message_data: data,
        }
    }

    pub fn empty_message() -> Self {
        Self {
            sender: GMSender::Unknown,
            receiver: GMReceiver::Engine,
            message_data: GMMessageData::Empty,
        }
    }

    pub fn empty_message_ok() -> Result<Self, GMError> {
        Ok(Self::empty_message())
    }

    pub fn sender2receiver(sender: &GMSender) -> Option<GMReceiver> {
        match sender {
            GMSender::Unknown => {
                None
            }
            GMSender::Engine => {
                Some(GMReceiver::Engine)
            }
            GMSender::CurrentScene => {
                Some(GMReceiver::CurrentScene)
            }
            GMSender::Scene(name) => {
                Some(GMReceiver::Scene(name.to_string()))
            }
            GMSender::SceneModifier(name) => {
                Some(GMReceiver::SceneModifier(name.to_string()))
            }
            GMSender::SceneManager => {
                Some(GMReceiver::SceneManager)
            }
            GMSender::Object(name) => {
                Some(GMReceiver::Object(name.to_string()))
            }
            GMSender::ObjectModifier(name) => {
                Some(GMReceiver::ObjectModifier(name.to_string()))
            }
            GMSender::ObjectManager => {
                Some(GMReceiver::ObjectManager)
            }

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

    pub fn send(&self, message_data: GMMessageData, context: &mut GMUpdateContext) {
        context.send_message(self.create(message_data));
    }

    pub fn send_to(&self, receiver: GMReceiver, message_data: GMMessageData, context: &mut GMUpdateContext) {
        context.send_message(self.create_to(receiver, message_data))
    }

    pub fn create(&self, message_data: GMMessageData) -> GMMessage {
        GMMessage::new(
            self.sender.clone(),
            self.receiver.clone(),
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
}

#[derive(Clone, Debug)]
pub enum GMSender {
    Unknown,

    Engine,

    CurrentScene,
    Scene(String),
    SceneModifier(String),
    SceneManager,

    Object(String),
    ObjectModifier(String),
    ObjectManager,
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
    Empty,

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
