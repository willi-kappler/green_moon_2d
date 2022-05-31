

use std::rc::Rc;

use crate::bitmap_text::GMBitmapFont;
use crate::math::GMVec2D;
use crate::property::GMValue;
use crate::scene::GMSceneT;
use crate::object::{GMObjectT};
use crate::texture::GMTextureConfig;

#[derive(Debug)]
pub(crate) enum GMEngineMessage {
    ChangeFPS(u32),
    Quit,
}

#[derive(Debug)]
pub(crate) enum GMSceneManagerMessage {
    AddScene(String, Box<dyn GMSceneT>),
    ChangeToScene(String),
    MessageToCurrentScene(GMSceneMessage),
    MessageToScene(String, GMSceneMessage),
    PopAndChangeScene,
    PushAndChangeScene(String),
    RemoveScene(String),
    ReplaceScene(String, Box<dyn GMSceneT>),
}

#[derive(Clone, Debug)]
pub enum GMSceneMessage {
    ClonedFrom(String, Box<dyn GMObjectT>),
    Enter,
    Leave,
    Update,
}

#[derive(Clone, Debug)]
pub enum GMSceneReply {
    Empty,
}

#[derive(Clone, Debug)]
pub(crate) enum GMObjectManagerMessage {
    AddObject(String, Box<dyn GMObjectT>),
    Clear,
    GetClone(String, GMMessageReplyTo), // object to clone
    MessageToObject(String, GMObjectMessage, GMMessageReplyTo), // receiver
    RemoveObject(String),
    ReplaceObject(String, Box<dyn GMObjectT>),
    SetParent(String, Box<dyn GMObjectT>),
}


#[derive(Clone, Debug)]
pub enum GMObjectMessage {
    AddProperty(String, GMValue),
    ClonedFrom(String, Box<dyn GMObjectT>),
    GetPosition,
    GetProperty(String),
    ReplyFrom(String, GMObjectReply),
    SetChild(Box<dyn GMObjectT>),
    SetFont(Rc<GMBitmapFont>),
    SetHorizontal(bool),
    SetPosition(GMVec2D),
    SetSpacingX(f32),
    SetSpacingY(f32),
    Update,
}

#[derive(Clone, Debug)]
pub enum GMObjectReply {
    ClonedObject(Box<dyn GMObjectT>),
    Empty,
    Property(String, GMValue),
    Position(GMVec2D),
}

#[derive(Clone, Debug)]
pub enum GMMessageReplyTo {
    Object(String),
    CurrentScene,
}

#[derive(Clone, Debug)]
pub(crate) enum GMDrawMessage {
    DrawTexture(GMTextureConfig),
}
