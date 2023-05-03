

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
    CustomM(String, Vec<GMValue>),
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
}

impl GMMessage {
    pub fn to_vec(self) -> Vec<GMMessage> {
        match self {
            Self::Multiple(messages) => {
                messages
            }
            _ => {
                vec![self]
            }
        }
    }

    pub fn chain(self, message: GMMessage) -> GMMessage {
        match self {
            Self::Multiple(mut left_messages) => {
                match message {
                    Self::Multiple(right_messages) => {
                        left_messages.extend(right_messages);
                        left_messages.into()
                    }
                    _ => {
                        left_messages.push(message);
                        left_messages.into()
                    }
                }
            }
            _ => {
                match message {
                    Self::Multiple(right_messages) => {
                        let mut left_messages = vec![self];
                        left_messages.extend(right_messages);
                        left_messages.into()
                    }
                    _ => {
                        vec![self, message].into()
                    }
                }
            }
        }
    }
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

impl From<(&str, GMValue, GMValue)> for GMMessage {
    fn from((name, value1, value2): (&str, GMValue, GMValue)) -> Self {
        Self::CustomM(name.to_string(), vec![value1, value2])
    }
}

impl From<(&str, GMValue, GMValue, GMValue)> for GMMessage {
    fn from((name, value1, value2, value3): (&str, GMValue, GMValue, GMValue)) -> Self {
        Self::CustomM(name.to_string(), vec![value1, value2, value3])
    }
}

impl From<(&str, GMValue, GMValue, GMValue, GMValue)> for GMMessage {
    fn from((name, value1, value2, value3, value4): (&str, GMValue, GMValue, GMValue, GMValue)) -> Self {
        Self::CustomM(name.to_string(), vec![value1, value2, value3, value4])
    }
}

impl From<(GMMessage, GMMessage)> for GMMessage {
    fn from((m1, m2): (GMMessage, GMMessage)) -> Self {
        vec![m1, m2].into()
    }
}

impl From<(GMMessage, GMMessage, GMMessage)> for GMMessage {
    fn from((m1, m2, m3): (GMMessage, GMMessage, GMMessage)) -> Self {
        vec![m1, m2, m3].into()
    }
}

impl From<(GMMessage, GMMessage, GMMessage, GMMessage)> for GMMessage {
    fn from((m1, m2, m3, m4): (GMMessage, GMMessage, GMMessage, GMMessage)) -> Self {
        vec![m1, m2, m3, m4].into()
    }
}