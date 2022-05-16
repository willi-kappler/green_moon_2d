
use std::rc::Rc;
use std::convert::From;

use crate::animation::GMAnimationT;
// use crate::context::GMUpdateContext;
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
    pub message_data: GMMessageData, // TODO: Vec<GMMessageData>
}

impl GMMessage {
    pub fn new<S: Into<GMSender>, R: Into<GMReceiver>>(sender: S, receiver: R, message_data: GMMessageData) -> Self {
        Self {
            sender: sender.into(),
            receiver: receiver.into(),
            message_data,
        }
    }

    pub fn new_reply<S: Into<GMSender>>(sender: S, message: &GMMessage, message_data: GMMessageData) -> Self {
        Self::new(sender, &message.sender, message_data)
    }

    pub fn as_reply(&self) -> GMReceiver {
        self.sender.as_receiver()
    }
}

#[derive(Clone, Debug)]
pub enum GMSender {
    Engine,

    CurrentScene,
    Scene(String),
    SceneManager,

    Object(String),
    ObjectManager,
}

impl GMSender {
    pub fn as_receiver(&self) -> GMReceiver {
        match self {
            GMSender::Engine => {
                GMReceiver::Engine
            }
            GMSender::CurrentScene => {
                GMReceiver::CurrentScene
            }
            GMSender::Scene(name) => {
                GMReceiver::Scene(name.to_string())
            }
            GMSender::SceneManager => {
                GMReceiver::SceneManager
            }
            GMSender::Object(name) => {
                GMReceiver::Object(name.to_string())
            }
            GMSender::ObjectManager => {
                GMReceiver::ObjectManager
            }
        }
    }
}

impl From<&Box<dyn GMObjectT>> for GMSender {
    fn from(object: &Box<dyn GMObjectT>) -> Self {
        object.as_sender()
    }
}

#[derive(Clone, Debug)]
pub enum GMReceiver {
    Engine,

    CurrentScene,
    Scene(String),
    SceneWithProperty(String),
    SceneManager,

    Object(String),
    ObjectWithProperty(String),
    ObjectManager,
}

impl From<&Box<dyn GMObjectT>> for GMReceiver {
    fn from(object: &Box<dyn GMObjectT>) -> Self {
        object.as_receiver()
    }
}

impl From<&GMSender> for GMReceiver {
    fn from(sender: &GMSender) -> Self {
        sender.as_receiver()
    }
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
    MessageToChild(Box<GMMessageData>),

    SetZIndex(i32),
    GetZIndex,
    ZIndex(i32),

    SetPosition(GMVec2D),
    AddPosition(GMVec2D),
    GetPosition,
    Position(GMVec2D),
    GetNextPosition,
    NextPosition(GMVec2D),

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

    SetDuration(f32),
    GetDuration,
    Duration(f32),

    AddProperty(String, GMValue),
    AddTag(String),
    GetProperty(String),
    Property(String, GMValue),
    HasProperty(String),
    PropertyFound(String),
    PropertyNotFound(String),
    RemoveProperty(String),
}
