


use crate::property::GMValue;
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

    ClonedFrom(String, Box<dyn GMObjectT>),
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
    Clear,
    SetParent(String, Box<dyn GMObjectT>),
    GetClone(String, GMMessageReplyTo), // object to clone

    MessageToObject(String, GMObjectMessage), // receiver
}


#[derive(Clone, Debug)]
pub enum GMObjectMessage {
    Update,

    AddProperty(String, GMValue),
    GetProperty(String, Option<GMMessageReplyTo>),
    SetChild(Box<dyn GMObjectT>),
    ClonedFrom(String, Box<dyn GMObjectT>),
}

#[derive(Clone, Debug)]
pub enum GMObjectReply {
    Empty,
    Property(String, GMValue),
    ClonedObject(Box<dyn GMObjectT>),
}

#[derive(Clone, Debug)]
pub enum GMMessageReplyTo {
    Object(String),
    Scene,
}

#[derive(Clone, Debug)]
pub(crate) enum GMDrawMessage {
    DrawTexture(GMTextureConfig),
}
