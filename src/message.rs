
use std::any::Any;
use std::rc::Rc;

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
    SceneManager,

    Object(String),
    ObjectGroup(String),
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
    GetZindex,
    ZIndex(i32),

    SetPosition(GMVec2D),
    AddPosition(GMVec2D),
    GetPosition,
    Position(GMVec2D),

    Custom(Rc<dyn Any>),
}
