

use crate::math::{GMVec2D, GMSize};
use crate::value::GMValue;
use crate::object_manager::GMObjectInfo;
use crate::object::GMObjectT;
use crate::target::GMTarget;


#[derive(Clone, Debug)]
pub enum GMMessage {
    AddPosition(GMVec2D),
    AddX(f32),
    AddY(f32),
    Custom0(String),
    Custom1(String, GMValue),
    Custom2(String, GMValue, GMValue),
    Custom3(String, GMValue, GMValue, GMValue),
    Custom4(String, GMValue, GMValue, GMValue, GMValue),
    Forward(Box<GMMessage>),
    GetChild(usize),
    GetChildCount,
    GetMessage,
    GetPosition,
    GetSize,
    GetTarget,
    GetX,
    GetY,
    Keep(Box<GMMessage>),
    Multiple(Vec<GMMessage>),
    OMAddCustomObject(String, GMObjectInfo),
    OMAddDrawObject(String, Box<dyn GMObjectT>, i32, i32),
    OMAddGroup(String, String),
    OMAddNormalObject(String, Box<dyn GMObjectT>, i32),
    OMClearCustomProperties(String),
    OMClearGroups(String),
    OMRemoveCustomProperty(String, String),
    OMRemoveGroup(String, String),
    OMRemoveObject(String),
    OMReplaceObject(String, Box<dyn GMObjectT>),
    OMSetActive(String, bool),
    OMSetCustomProperty(String, String, GMValue),
    OMSetDrawIndex(String, i32),
    OMSetUpdateIndex(String, i32),
    OMSetVisible(String, bool),
    OMToggleActive(String),
    OMToggleVisible(String),
    SetChild(usize, Box<dyn GMObjectT>),
    SetMessage(Box<GMMessage>),
    SetPosition(GMVec2D),
    SetSize(GMSize),
    SetTarget(GMTarget),
    SetX(f32),
    SetY(f32),
    ToChild(usize, Box<GMMessage>),
    ToAllChildren(Box<GMMessage>),
    Tuple2(Box<GMMessage>, Box<GMMessage>),
    Tuple3(Box<GMMessage>, Box<GMMessage>, Box<GMMessage>),
    Tuple4(Box<GMMessage>, Box<GMMessage>, Box<GMMessage>, Box<GMMessage>),
}

impl From<Vec<GMMessage>> for GMMessage {
    fn from(messages: Vec<GMMessage>) -> Self {
        Self::Multiple(messages)
    }
}

impl From<(&str, GMValue)> for GMMessage {
    fn from((name, value): (&str, GMValue)) -> Self {
        Self::Custom1(name.to_string(), value)
    }
}

impl From<(GMMessage, GMMessage)> for GMMessage {
    fn from((m1, m2): (GMMessage, GMMessage)) -> Self {
        Self::Tuple2(Box::new(m1), Box::new(m2))
    }
}

impl From<(GMMessage, GMMessage, GMMessage)> for GMMessage {
    fn from((m1, m2, m3): (GMMessage, GMMessage, GMMessage)) -> Self {
        Self::Tuple3(Box::new(m1), Box::new(m2), Box::new(m3))
    }
}

impl From<(GMMessage, GMMessage, GMMessage, GMMessage)> for GMMessage {
    fn from((m1, m2, m3, m4): (GMMessage, GMMessage, GMMessage, GMMessage)) -> Self {
        Self::Tuple4(Box::new(m1), Box::new(m2), Box::new(m3), Box::new(m4))
    }
}
