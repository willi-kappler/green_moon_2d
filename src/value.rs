
use std::rc::Rc;
use std::any::Any;
use std::cell::RefCell;

use crate::message::GMMessage;
use crate::object::GMObjectT;
use crate::math::{GMVec2D, GMSize};
use crate::target::GMTarget;


#[derive(Clone, Debug)]
pub enum GMValue {
    Any(Rc<dyn Any>),
    Binary(Vec<u8>),
    Bool(bool),
    Custom0(String),
    Custom1(String, Box<GMValue>),
    Custom2(String, Box<GMValue>, Box<GMValue>),
    Custom3(String, Box<GMValue>, Box<GMValue>, Box<GMValue>),
    Custom4(String, Box<GMValue>, Box<GMValue>, Box<GMValue>, Box<GMValue>),
    F32(f32),
    F64(f64),
    I16(i16),
    I32(i32),
    I64(i64),
    I8(i8),
    Message(Box<GMMessage>),
    Multiple(Vec<GMValue>),
    None,
    Object(Box<dyn GMObjectT>),
    Position(GMVec2D),
    Shared(Rc<GMValue>),
    SharedCell(Rc<RefCell<GMValue>>),
    Size(GMSize),
    String(String),
    Target(GMTarget),
    Tuple2(Box<GMValue>, Box<GMValue>),
    Tuple3(Box<GMValue>, Box<GMValue>, Box<GMValue>),
    Tuple4(Box<GMValue>, Box<GMValue>, Box<GMValue>, Box<GMValue>),
    U16(u16),
    U32(u32),
    U64(u64),
    U8(u8),
    USize(usize),
    Vec2D(GMVec2D),
}

impl GMValue {
    pub fn chain(self, value: GMValue) -> GMValue {
        // TODO: match also on value
        match self {
            Self::Tuple2(v1, v2) => {
                Self::Tuple3(v1, v2, Box::new(value))
            }
            Self::Tuple3(v1, v2, v3) => {
                Self::Tuple4(v1, v2, v3, Box::new(value))
            }
            Self::Tuple4(v1, v2, v3, v4) => {
                let mut values = Vec::new();
                values.push(v1);
                values.push(v2);
                values.push(v3);
                values.push(v4);
                values.push(value);
                Self::Multiple(values)
            }
            Self::Multiple(mut values) => {
                values.push(value);
                Self::Multiple(values)
            }
            _ => {
                Self::Tuple2(Box::new(self), Box::new(value))
            }
        }
    }
}

impl From<()> for GMValue {
    fn from(_value: ()) -> Self {
        Self::None
    }
}

impl From<bool> for GMValue {
    fn from(value: bool) -> Self {
        Self::Bool(value)
    }
}

impl From<u8> for GMValue {
    fn from(value: u8) -> Self {
        Self::U8(value)
    }
}

impl From<u16> for GMValue {
    fn from(value: u16) -> Self {
        Self::U16(value)
    }
}

impl From<u32> for GMValue {
    fn from(value: u32) -> Self {
        Self::U32(value)
    }
}

impl From<u64> for GMValue {
    fn from(value: u64) -> Self {
        Self::U64(value)
    }
}

impl From<i8> for GMValue {
    fn from(value: i8) -> Self {
        Self::I8(value)
    }
}

impl From<i16> for GMValue {
    fn from(value: i16) -> Self {
        Self::I16(value)
    }
}

impl From<i32> for GMValue {
    fn from(value: i32) -> Self {
        Self::I32(value)
    }
}

impl From<i64> for GMValue {
    fn from(value: i64) -> Self {
        Self::I64(value)
    }
}

impl From<usize> for GMValue {
    fn from(value: usize) -> Self {
        Self::USize(value)
    }
}

impl From<f32> for GMValue {
    fn from(value: f32) -> Self {
        Self::F32(value)
    }
}

impl From<GMVec2D> for GMValue {
    fn from(value: GMVec2D) -> Self {
        Self::Vec2D(value)
    }
}

impl From<&str> for GMValue {
    fn from(value: &str) -> Self {
        Self::String(value.to_string())
    }
}

impl From<String> for GMValue {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

impl From<(bool, bool)> for GMValue {
    fn from((v1, v2): (bool, bool)) -> Self {
        Self::Tuple2(Box::new(GMValue::Bool(v1)), Box::new(GMValue::Bool(v2)))
    }
}

impl From<(f32, f32)> for GMValue {
    fn from((v1, v2): (f32, f32)) -> Self {
        Self::Tuple2(Box::new(GMValue::F32(v1)), Box::new(GMValue::F32(v2)))
    }
}

impl From<(f32, f32, f32)> for GMValue {
    fn from((v1, v2, v3): (f32, f32, f32)) -> Self {
        Self::Tuple3(Box::new(GMValue::F32(v1)), Box::new(GMValue::F32(v2)), Box::new(GMValue::F32(v3)))
    }
}

impl From<GMMessage> for GMValue {
    fn from(value: GMMessage) -> Self {
        Self::Message(Box::new(value))
    }
}

impl From<GMTarget> for GMValue {
    fn from(value: GMTarget) -> Self {
        Self::Target(value)
    }
}

impl From<(GMValue, GMValue)> for GMValue {
    fn from((v1, v2): (GMValue, GMValue)) -> Self {
        Self::Tuple2(Box::new(v1), Box::new(v2))
    }
}

impl From<(GMValue, GMValue, GMValue)> for GMValue {
    fn from((v1, v2, v3): (GMValue, GMValue, GMValue)) -> Self {
        Self::Tuple3(Box::new(v1), Box::new(v2), Box::new(v3))
    }
}

impl From<(GMValue, GMValue, GMValue, GMValue)> for GMValue {
    fn from((v1, v2, v3, v4): (GMValue, GMValue, GMValue, GMValue)) -> Self {
        Self::Tuple4(Box::new(v1), Box::new(v2), Box::new(v3), Box::new(v4))
    }
}

impl From<Vec<GMValue>> for GMValue {
    fn from(value: Vec<GMValue>) -> Self {
        Self::Multiple(value)
    }
}

impl From<(&str, GMValue)> for GMValue {
    fn from((name, value): (&str, GMValue)) -> Self {
        Self::Custom1(name.to_string(), Box::new(value))
    }
}

impl From<(&str, GMValue, GMValue)> for GMValue {
    fn from((name, value1, value2): (&str, GMValue, GMValue)) -> Self {
        Self::Custom2(name.to_string(), Box::new(value1), Box::new(value2))
    }
}

impl From<(&str, GMValue, GMValue, GMValue)> for GMValue {
    fn from((name, value1, value2, value3): (&str, GMValue, GMValue, GMValue)) -> Self {
        Self::Custom3(name.to_string(), Box::new(value1), Box::new(value2), Box::new(value3))
    }
}

impl From<(&str, GMValue, GMValue, GMValue, GMValue)> for GMValue {
    fn from((name, value1, value2, value3, value4): (&str, GMValue, GMValue, GMValue, GMValue)) -> Self {
        Self::Custom4(name.to_string(), Box::new(value1), Box::new(value2), Box::new(value3), Box::new(value4))
    }
}
