
use std::any::Any;

use std::fmt::{self, Debug};
use crate::math::GMVec2D;


pub struct GMObjectMessage {
    pub sender: GMSender,
    pub receiver: GMReceiver,
    pub data: GMObjectMessageData,
}

impl GMObjectMessage {
    pub fn new(sender: GMSender, receiver: GMReceiver, data: GMObjectMessageData) -> Self {
        Self {
            sender,
            receiver,
            data,
        }
    }
}

impl Debug for GMObjectMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GMObjectMessage")
            .field("sender", &self.sender)
            .field("receiver", &self.receiver)
            .field("data", &self.data).finish()
    }
}

#[derive(Clone, Debug)]
pub enum GMSender {
    Object(String),
    Scene(String),
}

#[derive(Clone, Debug)]
pub enum GMReceiver {
    Object(String),
    Group(String),
    Scene(String),
}

pub enum GMObjectMessageData {
    SetPosition(GMVec2D),
    AddPosition(GMVec2D),
    GetPosition,
    Position(GMVec2D),

    SetActive(bool),
    GetActive,
    Active(bool),

    Custom(Box<dyn Any>),
}

impl Debug for GMObjectMessageData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SetPosition(position) => f.debug_tuple("SetPosition").field(position).finish(),
            Self::AddPosition(position) => f.debug_tuple("AddPosition").field(position).finish(),
            Self::GetPosition => write!(f, "GetPosition"),
            Self::Position(position) => f.debug_tuple("Position").field(position).finish(),
            Self::SetActive(active) => f.debug_tuple("SetActive").field(active).finish(),
            Self::GetActive => write!(f, "GetActive"),
            Self::Active(active) => f.debug_tuple("Active").field(active).finish(),
            Self::Custom(custom) => f.debug_tuple("Custom").field(custom).finish(),
        }
    }
}
