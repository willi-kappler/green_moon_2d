
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
    CustomM(String, Vec<GMValue>),
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
    U16(u16),
    U32(u32),
    U64(u64),
    U8(u8),
    USize(usize),
    Vec2D(GMVec2D),
}

impl GMValue {
    pub fn to_vec(self) -> Vec<GMValue> {
        match self {
            Self::Multiple(values) => {
                values
            }
            _ => {
                vec![self]
            }
        }
    }

    pub fn chain(self, value: GMValue) -> GMValue {
        match self {
            Self::Multiple(mut left_values) => {
                match value {
                    Self::Multiple(right_values) => {
                        left_values.extend(right_values);
                        left_values.into()
                    }
                    _ => {
                        left_values.push(value);
                        left_values.into()
                    }
                }
            }
            _ => {
                match value {
                    Self::Multiple(right_values) => {
                        let mut left_values = vec![self];
                        left_values.extend(right_values);
                        left_values.into()
                    }
                    _ => {
                        vec![self, value].into()
                    }
                }
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
        vec![GMValue::Bool(v1), v2.into()].into()
    }
}

impl From<(f32, f32)> for GMValue {
    fn from((v1, v2): (f32, f32)) -> Self {
        vec![GMValue::F32(v1), v2.into()].into()
    }
}

impl From<(f32, f32, f32)> for GMValue {
    fn from((v1, v2, v3): (f32, f32, f32)) -> Self {
        vec![GMValue::F32(v1), v2.into(), v3.into()].into()
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
        vec![v1, v2].into()
    }
}

impl From<(GMValue, GMValue, GMValue)> for GMValue {
    fn from((v1, v2, v3): (GMValue, GMValue, GMValue)) -> Self {
        vec![v1, v2, v3].into()
    }
}

impl From<(GMValue, GMValue, GMValue, GMValue)> for GMValue {
    fn from((v1, v2, v3, v4): (GMValue, GMValue, GMValue, GMValue)) -> Self {
        vec![v1, v2, v3, v4].into()
    }
}

impl From<Vec<GMVec2D>> for GMValue {
    fn from(values: Vec<GMVec2D>) -> Self {
        let values: Vec<GMValue> = values.iter().map(|v| GMValue::Vec2D(*v)).collect();
        Self::Multiple(values)
    }
}

impl From<Vec<f32>> for GMValue {
    fn from(values: Vec<f32>) -> Self {
        let values: Vec<GMValue> = values.iter().map(|v| GMValue::F32(*v)).collect();
        Self::Multiple(values)
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
        Self::CustomM(name.to_string(), vec![value1, value2])
    }
}

impl From<(&str, GMValue, GMValue, GMValue)> for GMValue {
    fn from((name, value1, value2, value3): (&str, GMValue, GMValue, GMValue)) -> Self {
        Self::CustomM(name.to_string(), vec![value1, value2, value3])
    }
}

impl From<(&str, GMValue, GMValue, GMValue, GMValue)> for GMValue {
    fn from((name, value1, value2, value3, value4): (&str, GMValue, GMValue, GMValue, GMValue)) -> Self {
        Self::CustomM(name.to_string(), vec![value1, value2, value3, value4])
    }
}
