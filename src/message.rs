

use crate::math::{GMVec2D, GMSize};
use crate::object_manager::GMObjectInfo;
use crate::object::GMObjectT;
use crate::target::GMTarget;
use crate::value::GMValue;


#[derive(Clone, Debug)]
pub enum GMMessage {
    AddMultiPosition(Vec<GMVec2D>),
    AddPosition(GMVec2D),
    AddX(f32),
    AddY(f32),
    Custom0(String),
    Custom1(String, GMValue),
    Custom2(String, GMValue, GMValue),
    Custom3(String, GMValue, GMValue, GMValue),
    CustomM(String, Vec<GMValue>),
    Forward(Box<GMMessage>),
    GetMessage,
    GetMultiPosition,
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
    OMClearGroups(String),
    OMRemoveGroup(String, String),
    OMRemoveObject(String),
    OMReplaceObject(String, Box<dyn GMObjectT>),
    OMSetActive(String, bool),
    OMSetDrawIndex(String, i32),
    OMSetUpdateIndex(String, i32),
    OMSetVisible(String, bool),
    OMToggleActive(String),
    OMToggleVisible(String),
    SetMessage(Box<GMMessage>),
    SetMultiPosition(Vec<GMVec2D>),
    SetPosition(GMVec2D),
    SetSize(GMSize),
    SetTarget(GMTarget),
    SetX(f32),
    SetY(f32),
    Update,
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

    pub fn chain(self, other: GMMessage) -> GMMessage {
        match self {
            Self::Multiple(mut left_messages) => {
                match other {
                    Self::Multiple(right_messages) => {
                        left_messages.extend(right_messages);
                        left_messages.into()
                    }
                    _ => {
                        left_messages.push(other);
                        left_messages.into()
                    }
                }
            }
            _ => {
                match other {
                    Self::Multiple(right_messages) => {
                        let mut left_messages = vec![self];
                        left_messages.extend(right_messages);
                        left_messages.into()
                    }
                    _ => {
                        vec![self, other].into()
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

impl From<&str> for GMMessage {
    fn from(name: &str) -> Self {
        Self::Custom0(name.to_string())
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
