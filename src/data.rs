
use std::any::Any;
use std::fmt::Debug;
use std::rc::Rc;

use crate::math::GMVec2D;
use crate::animation::GMAnimation;
use crate::texture::GMTexture;
use crate::util::{error_panic, GMRepetition};

pub trait GMAnyT: Debug {
    fn clone_box(&self) -> Box<dyn GMAnyT>;

    fn to_any(&self) -> Box<dyn Any>;
}

impl Clone for Box<dyn GMAnyT> {
    fn clone(&self) -> Box<dyn GMAnyT> {
        self.clone_box()
    }
}

// TODO: write a macro that creates the enum

#[derive(Debug, Clone)]
pub enum GMData {
    None,
    Bool(bool),
    U8(u8),
    U8U8(u8, u8),
    U8U8U8(u8, u8, u8),
    I8(i8),
    I8I8(i8, i8),
    I8I8I8(i8, i8, i8),
    U16(u16),
    U16U16(u16, u16),
    U16U16U16(u16, u16, u16),
    I16(i16),
    I16I16(i16, i16),
    I16I16I16(i16, i16, i16),
    U32(u32),
    U32U32(u32, u32),
    U32U32U32(u32, u32, u32),
    I32(i32),
    I32I32(i32, i32),
    I32I32I32(i32, i32, i32),
    U64(u64),
    U64U64(u64, u64),
    U64U64U64(u64, u64, u64),
    I64(i64),
    I64I64(i64, i64),
    I64I64I64(i64, i64, i64),
    USize(usize),
    USizeUSize(usize, usize),
    USizeUSizeUSize(usize, usize, usize),
    F32(f32),
    F32F32(f32, f32),
    F32F32F32(f32, f32, f32),
    F64(f64),
    F64F64(f64, f64),
    F64F64F64(f64, f64, f64),
    Char(char),
    CharChar(char, char),
    CharCharChar(char, char, char),
    String(String),
    Vec2D(GMVec2D),
    Vec2DVec2D(GMVec2D, GMVec2D),
    Vec2DVec2DVec2D(GMVec2D, GMVec2D, GMVec2D),
    Repetition(GMRepetition),
    Texture(Rc<GMTexture>),
    Animation(GMAnimation),
    Tuple(Box<GMData>, Box<GMData>),
    Vec(Box<GMData>),
    Custom(Box<dyn GMAnyT>),
}

// TODO: write a macro that creates all the conversion functions

impl From<bool> for GMData {
    fn from(data: bool) -> Self {
        GMData::Bool(data)
    }
}

impl From<u8> for GMData {
    fn from(data: u8) -> Self {
        GMData::U8(data)
    }
}

impl From<(u8, u8)> for GMData {
    fn from(data: (u8, u8)) -> Self {
        GMData::U8U8(data.0, data.1)
    }
}

impl From<(u8, u8, u8)> for GMData {
    fn from(data: (u8, u8, u8)) -> Self {
        GMData::U8U8U8(data.0, data.1, data.2)
    }
}

impl From<u32> for GMData {
    fn from(data: u32) -> Self {
        GMData::U32(data)
    }
}

impl From<(u32, u32)> for GMData {
    fn from(data: (u32, u32)) -> Self {
        GMData::U32U32(data.0, data.1)
    }
}

impl From<(u32, u32, u32)> for GMData {
    fn from(data: (u32, u32, u32)) -> Self {
        GMData::U32U32U32(data.0, data.1, data.2)
    }
}

impl From<usize> for GMData {
    fn from(data: usize) -> Self {
        GMData::USize(data)
    }
}

impl From<f32> for GMData {
    fn from(data: f32) -> Self {
        GMData::F32(data)
    }
}

impl From<(f32, f32)> for GMData {
    fn from(data: (f32, f32)) -> Self {
        GMData::F32F32(data.0, data.1)
    }
}

impl From<String> for GMData {
    fn from(data: String) -> Self {
        GMData::String(data)
    }
}

impl From<GMVec2D> for GMData {
    fn from(data: GMVec2D) -> Self {
        GMData::Vec2D(data)
    }
}

impl From<GMRepetition> for GMData {
    fn from(data: GMRepetition) -> Self {
        GMData::Repetition(data)
    }
}

impl From<Rc<GMTexture>> for GMData {
    fn from(data: Rc<GMTexture>) -> Self {
        GMData::Texture(data)
    }
}

impl From<GMAnimation> for GMData {
    fn from(data: GMAnimation) -> Self {
        GMData::Animation(data)
    }
}





impl From<GMData> for bool {
    fn from(data: GMData) -> Self {
        if let GMData::Bool(data) = data {
            data
        } else {
            error_panic(&format!("Expected Bool, got {:?}", data))
        }
    }
}

impl From<GMData> for u8 {
    fn from(data: GMData) -> Self {
        if let GMData::U8(data) = data {
            data
        } else {
            error_panic(&format!("Expected U8, got {:?}", data))
        }
    }
}

impl From<GMData> for (u8, u8) {
    fn from(data: GMData) -> Self {
        if let GMData::U8U8(data1, data2) = data {
            (data1, data2)
        } else {
            error_panic(&format!("Expected U8U8, got {:?}", data))
        }
    }
}

impl From<GMData> for (u8, u8, u8) {
    fn from(data: GMData) -> Self {
        if let GMData::U8U8U8(data1, data2, data3) = data {
            (data1, data2, data3)
        } else {
            error_panic(&format!("Expected U8U8U8, got {:?}", data))
        }
    }
}

impl From<GMData> for u32 {
    fn from(data: GMData) -> Self {
        if let GMData::U32(data) = data {
            data
        } else {
            error_panic(&format!("Expected U32, got {:?}", data))
        }
    }
}

impl From<GMData> for (u32, u32) {
    fn from(data: GMData) -> Self {
        if let GMData::U32U32(data1, data2) = data {
            (data1, data2)
        } else {
            error_panic(&format!("Expected U32U32, got {:?}", data))
        }
    }
}

impl From<GMData> for (u32, u32, u32) {
    fn from(data: GMData) -> Self {
        if let GMData::U32U32U32(data1, data2, data3) = data {
            (data1, data2, data3)
        } else {
            error_panic(&format!("Expected U32U32U32, got {:?}", data))
        }
    }
}

impl From<GMData> for usize {
    fn from(data: GMData) -> Self {
        if let GMData::USize(data) = data {
            data
        } else {
            error_panic(&format!("Expected U8, got {:?}", data))
        }
    }
}

impl From<GMData> for f32 {
    fn from(data: GMData) -> Self {
        if let GMData::F32(data) = data {
            data
        } else {
            error_panic(&format!("Expected F32, got {:?}", data))
        }
    }
}

impl From<GMData> for (f32, f32) {
    fn from(data: GMData) -> Self {
        if let GMData::F32F32(data1, data2) = data {
            (data1, data2)
        } else {
            error_panic(&format!("Expected F32F32, got {:?}", data))
        }
    }
}

impl From<GMData> for String {
    fn from(data: GMData) -> Self {
        if let GMData::String(data) = data {
            data
        } else {
            error_panic(&format!("Expected String, got {:?}", data))
        }
    }
}

impl From<GMData> for GMVec2D {
    fn from(data: GMData) -> Self {
        if let GMData::Vec2D(data) = data {
            data
        } else {
            error_panic(&format!("Expected Vec2D, got {:?}", data))
        }
    }
}

impl From<GMData> for GMRepetition {
    fn from(data: GMData) -> Self {
        if let GMData::Repetition(data) = data {
            data
        } else {
            error_panic(&format!("Expected Repetition, got {:?}", data))
        }
    }
}

impl From<GMData> for Rc<GMTexture> {
    fn from(data: GMData) -> Self {
        if let GMData::Texture(data) = data {
            data
        } else {
            error_panic(&format!("Expected Texture, got {:?}", data))
        }
    }
}

impl From<GMData> for GMAnimation {
    fn from(data: GMData) -> Self {
        if let GMData::Animation(data) = data {
            data
        } else {
            error_panic(&format!("Expected Animation, got {:?}", data))
        }
    }
}
