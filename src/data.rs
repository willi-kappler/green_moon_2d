
use std::any::Any;
use std::fmt::Debug;
use std::rc::Rc;

use crate::math::GMVec2D;
use crate::animation::GMAnimation;
use crate::texture::GMTexture;
use crate::util::{error_panic, GMRepetition};

use crate::{create_from_type_for_gmdata, create_from_gmdata_for_type};

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

#[allow(non_camel_case_types)]
#[derive(Debug, Clone)]
pub enum GMData {
    None,
    Bool(bool),
    Bool_2(bool, bool),
    Bool_3(bool, bool, bool),
    U8(u8),
    U8_2(u8, u8),
    U8_3(u8, u8, u8),
    I8(i8),
    I8_2(i8, i8),
    I8_3(i8, i8, i8),
    U16(u16),
    U16_2(u16, u16),
    U16_3(u16, u16, u16),
    I16(i16),
    I16_2(i16, i16),
    I16_3(i16, i16, i16),
    U32(u32),
    U32_2(u32, u32),
    U32_3(u32, u32, u32),
    I32(i32),
    I32_2(i32, i32),
    I32_3(i32, i32, i32),
    U64(u64),
    U64_2(u64, u64),
    U64_3(u64, u64, u64),
    I64(i64),
    I64_2(i64, i64),
    I64_3(i64, i64, i64),
    USize(usize),
    USize_2(usize, usize),
    USize_3(usize, usize, usize),
    F32(f32),
    F32_2(f32, f32),
    F32_3(f32, f32, f32),
    F64(f64),
    F64_2(f64, f64),
    F64_3(f64, f64, f64),
    Char(char),
    Char_2(char, char),
    Char_3(char, char, char),
    String(String),
    Vec2D(GMVec2D),
    Vec2D_2(GMVec2D, GMVec2D),
    Vec2D_3(GMVec2D, GMVec2D, GMVec2D),
    Repetition(GMRepetition),
    Texture(Rc<GMTexture>),
    Animation(GMAnimation),
    Tuple(Box<GMData>, Box<GMData>),
    Vec(Vec<Box<GMData>>),
    Custom(Box<dyn GMAnyT>),
}

impl From<(Box<GMData>, Box<GMData>)> for GMData {
    fn from(data: (Box<GMData>, Box<GMData>)) -> Self {
        GMData::Tuple(data.0, data.1)
    }
}

create_from_type_for_gmdata!(bool, Bool, Bool_2, Bool_3);
create_from_type_for_gmdata!(u8, U8, U8_2, U8_3);
create_from_type_for_gmdata!(i8, I8, I8_2, I8_3);
create_from_type_for_gmdata!(u16, U16, U16_2, U16_3);
create_from_type_for_gmdata!(i16, I16, I16_2, I16_3);
create_from_type_for_gmdata!(u32, U32, U32_2, U32_3);
create_from_type_for_gmdata!(i32, I32, I32_2, I32_3);
create_from_type_for_gmdata!(u64, U64, U64_2, U64_3);
create_from_type_for_gmdata!(i64, I64, I64_2, I64_3);
create_from_type_for_gmdata!(usize, USize, USize_2, USize_3);
create_from_type_for_gmdata!(f32, F32, F32_2, F32_3);
create_from_type_for_gmdata!(f64, F64, F64_2, F64_3);
create_from_type_for_gmdata!(char, Char, Char_2, Char_3);
create_from_type_for_gmdata!(GMVec2D, Vec2D, Vec2D_2, Vec2D_3);
create_from_type_for_gmdata!(String, String);
create_from_type_for_gmdata!(GMRepetition, Repetition);
create_from_type_for_gmdata!(Rc<GMTexture>, Texture);
create_from_type_for_gmdata!(GMAnimation, Animation);
create_from_type_for_gmdata!(Vec<Box<GMData>>, Vec);
create_from_type_for_gmdata!(Box<dyn GMAnyT>, Custom);


impl From<GMData> for (Box<GMData>, Box<GMData>) {
    fn from(data: GMData) -> Self {
        if let GMData::Tuple(data1, data2) = data {
            (data1, data2)
        } else {
            error_panic(&format!("Expected Tuple, got {:?}", data))
        }
    }
}        

create_from_gmdata_for_type!(bool, Bool, Bool_2, Bool_3);
create_from_gmdata_for_type!(u8, U8, U8_2, U8_3);
create_from_gmdata_for_type!(i8, I8, I8_2, I8_3);
create_from_gmdata_for_type!(u16, U16, U16_2, U16_3);
create_from_gmdata_for_type!(i16, I16, I16_2, I16_3);
create_from_gmdata_for_type!(u32, U32, U32_2, U32_3);
create_from_gmdata_for_type!(i32, I32, I32_2, I32_3);
create_from_gmdata_for_type!(u64, U64, U64_2, U64_3);
create_from_gmdata_for_type!(i64, I64, I64_2, I64_3);
create_from_gmdata_for_type!(usize, USize, USize_2, USize_3);
create_from_gmdata_for_type!(f32, F32, F32_2, F32_3);
create_from_gmdata_for_type!(f64, F64, F64_2, F64_3);
create_from_gmdata_for_type!(char, Char, Char_2, Char_3);
create_from_gmdata_for_type!(GMVec2D, Vec2D, Vec2D_2, Vec2D_3);
create_from_gmdata_for_type!(String, String);
create_from_gmdata_for_type!(GMRepetition, Repetition);
create_from_gmdata_for_type!(Rc<GMTexture>, Texture);
create_from_gmdata_for_type!(GMAnimation, Animation);
create_from_gmdata_for_type!(Vec<Box<GMData>>, Vec);
create_from_gmdata_for_type!(Box<dyn GMAnyT>, Custom);
