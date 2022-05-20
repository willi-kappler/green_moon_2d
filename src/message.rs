


use crate::scene::GMSceneT;
use crate::object::{GMObjectT};
use crate::texture::GMTextureConfig;

#[derive(Debug)]
pub(crate) enum GMEngineMessage {
    Quit,
    ChangeFPS(u32),
}

#[derive(Debug)]
pub(crate) enum GMSceneManagerMessage {
    AddScene(String, Box<dyn GMSceneT>),
    RemoveScene(String),
    ReplaceScene(String, Box<dyn GMSceneT>),
    PushAndChangeScene(String),
    PopAndChangeScene,
    ChangeToScene(String),

    MessageToCurrentScene(GMSceneMessage),
    MessageToScene(String, GMSceneMessage),
}

#[derive(Clone, Debug)]
pub enum GMSceneMessage {
    Update,
    Enter,
    Leave,
}

#[derive(Clone, Debug)]
pub enum GMSceneReply {
    Empty,
}

#[derive(Clone, Debug)]
pub(crate) enum GMObjectManagerMessage {
    AddObject(String, Box<dyn GMObjectT>),
    RemoveObject(String),
    ReplaceObject(String, Box<dyn GMObjectT>),
    SetParent(String, Box<dyn GMObjectT>),
    GetClone(String, GMMessageReplyTo), // object to clone, message sender

    MessageToObject(String, GMObjectMessage, GMMessageReplyTo), // receiver, message, sender
}


#[derive(Clone, Debug)]
pub enum GMObjectMessage {
    Update,

    SetChild(Box<dyn GMObjectT>),
}

#[derive(Clone, Debug)]
pub enum GMObjectReply {
    Empty,
    ClonedObject(Box<dyn GMObjectT>),
}

#[derive(Clone, Debug)]
pub enum GMMessageReplyTo {
    Object(String),
    Scene(String),
}

#[derive(Clone, Debug)]
pub(crate) enum GMDrawMessage {
    DrawTexture(GMTextureConfig),
}
