
use std::rc::Rc;
use std::any::Any;
use std::cell::RefCell;
use std::collections::VecDeque;

use crate::animation::GMAnimation;
use crate::bitmap_text::GMBitmapFont;
use crate::math::{GMVec2D, GMSize, GMFlipXY};
use crate::object_manager::{GMObjectInfo};
use crate::object::GMObjectT;
use crate::sprite::GMSprite;
use crate::target::GMTarget;
use crate::texture::GMTexture;
use crate::util::{GMRepetition, GMAlign, error_panic};
use crate::line::GMLineMode;


#[derive(Clone, Debug)]
pub enum GMValue {
    Align(GMAlign),
    Any(Rc<dyn Any>),
    Binary(Vec<u8>),
    Bool(bool),
    Custom0(String),
    Custom1(String, Box<GMValue>),
    CustomM(String, Vec<GMValue>),
    F32(f32),
    F64(f64),
    FlipXY(GMFlipXY),
    I16(i16),
    I32(i32),
    I64(i64),
    I8(i8),
    Message(String, String, Box<GMValue>),
    Multiple(VecDeque<GMValue>),
    None,
    Object(Box<dyn GMObjectT>),
    ObjectInfo(GMObjectInfo),
    Repetition(GMRepetition),
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
    pub fn is_none(&self) -> bool {
        if let Self::None = self {
            true
        } else {
            false
        }
    }

    pub fn to_vec_deque(self) -> VecDeque<GMValue> {
        match self {
            Self::Multiple(values) => {
                values
            }
            _ => {
                let mut vec_deque = VecDeque::new();
                vec_deque.push_back(self);
                vec_deque
            }
        }
    }

    pub fn chain(self, other: GMValue) -> GMValue {
        match self {
            Self::Multiple(mut left_values) => {
                match other {
                    Self::Multiple(right_values) => {
                        left_values.extend(right_values);
                        left_values.into()
                    }
                    _ => {
                        left_values.push_back(other);
                        left_values.into()
                    }
                }
            }
            _ => {
                match other {
                    Self::Multiple(right_values) => {
                        let mut left_values = VecDeque::new();
                        left_values.push_back(self);
                        left_values.extend(right_values);
                        left_values.into()
                    }
                    _ => {
                        let mut left_values = VecDeque::new();
                        left_values.push_back(self);
                        left_values.push_back(other);
                        left_values.into()
                    }
                }
            }
        }
    }

    pub fn into_bool(self) -> bool {
        if let Self::Bool(value) = self {
            return value
        }

        error_panic(&format!("GMValue::into_bool, not a boolean variant: '{:?}'", self));
    }

    pub fn into_i32(self) -> i32 {
        if let Self::I32(value) = self {
            return value
        }

        error_panic(&format!("GMValue::into_i32, not an i32variant: '{:?}'", self));
    }

    pub fn into_usize(self) -> usize {
        if let Self::USize(value) = self {
            return value
        }

        error_panic(&format!("GMValue::into_usize, not an usize variant: '{:?}'", self));
    }

    pub fn into_f32(self) -> f32 {
        if let Self::F32(value) = self {
            return value
        }

        error_panic(&format!("GMValue::into_f32, not a f32 variant: '{:?}'", self));
    }

    pub fn into_string(self) -> String {
        if let Self::String(value) = self {
            return value
        }

        error_panic(&format!("GMValue::into_string, not a string variant: '{:?}'", self));
    }

    pub fn into_target(self) -> GMTarget {
        if let Self::Target(value) = self {
            return value
        }

        error_panic(&format!("GMValue::into_target, not a target variant: '{:?}'", self));
    }

    pub fn into_vec2d(self) -> GMVec2D {
        if let Self::Vec2D(value) = self {
            return value
        }

        error_panic(&format!("GMValue::into_vec2d, not a vec2d variant: '{:?}'", self));
    }

    pub fn into_size(self) -> GMSize {
        if let Self::Size(value) = self {
            return value
        }

        error_panic(&format!("GMValue::into_size, not a size variant: '{:?}'", self));
    }

    pub fn into_flipxy(self) -> GMFlipXY {
        if let Self::FlipXY(value) = self {
            return value
        }

        error_panic(&format!("GMValue::into_flipxy, not a flipxy variant: '{:?}'", self));
    }

    pub fn into_repetition(self) -> GMRepetition {
        if let Self::Repetition(value) = self {
            return value
        }

        error_panic(&format!("GMValue::into_repetition, not a repetition variant: '{:?}'", self));
    }

    pub fn into_align(self) -> GMAlign {
        if let Self::Align(value) = self {
            return value
        }

        error_panic(&format!("GMValue::into_align, not an align variant: '{:?}'", self));
    }

    pub fn into_line_mode(self) -> GMLineMode {
        if let Self::Any(value) = self {
            return value.downcast_ref::<GMLineMode>().unwrap().clone();
        }

        error_panic(&format!("GMValue::into_line_mode, not an any variant: '{:?}'", self));
    }

    pub fn into_animation(self) -> GMAnimation {
        if let GMValue::Any(value) = self {
            return value.downcast_ref::<GMAnimation>().unwrap().clone();
        }

        error_panic(&format!("GMValue::into_animation, not an any variant: '{:?}'", self));
    }

    pub fn into_texture(self) -> Rc<GMTexture> {
        if let Self::Any(value) = self {
            return value.downcast_ref::<Rc<GMTexture>>().unwrap().clone();
        }

        error_panic(&format!("GMValue::into_texture, not an any variant: '{:?}'", self));
    }

    pub fn into_font(self) -> Rc<GMBitmapFont> {
        if let Self::Any(value) = self {
            return value.downcast_ref::<Rc<GMBitmapFont>>().unwrap().clone();
        }

        error_panic(&format!("GMValue::into_font, not an any variant: '{:?}'", self));
    }

    pub fn into_sprite(self) -> GMSprite {
        if let Self::Any(value) = self {
            return value.downcast_ref::<GMSprite>().unwrap().clone();
        }

        error_panic(&format!("GMValue::into_sprite, not an any variant: '{:?}'", self));
    }

    pub fn into_object_info(self) -> GMObjectInfo {
        if let Self::ObjectInfo(value) = self {
            return value
        }

        error_panic(&format!("GMValue::into_object_info, not an object info variant: '{:?}'", self));
    }

    pub fn into_object(self) -> Box<dyn GMObjectT> {
        if let Self::Object(value) = self {
            return value
        }

        error_panic(&format!("GMValue::into_object, not an object variant: '{:?}'", self));
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

impl From<GMSize> for GMValue {
    fn from(value: GMSize) -> Self {
        Self::Size(value)
    }
}

impl From<GMFlipXY> for GMValue {
    fn from(value: GMFlipXY) -> Self {
        Self::FlipXY(value)
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
        let mut vec_deque = VecDeque::new();
        vec_deque.push_back(GMValue::Bool(v1));
        vec_deque.push_back(GMValue::Bool(v2));
        vec_deque.into()
    }
}

impl From<(f32, f32)> for GMValue {
    fn from((v1, v2): (f32, f32)) -> Self {
        let mut vec_deque = VecDeque::new();
        vec_deque.push_back(GMValue::F32(v1));
        vec_deque.push_back(GMValue::F32(v2));
        vec_deque.into()
    }
}

impl From<(f32, f32, f32)> for GMValue {
    fn from((v1, v2, v3): (f32, f32, f32)) -> Self {
        let mut vec_deque = VecDeque::new();
        vec_deque.push_back(GMValue::F32(v1));
        vec_deque.push_back(GMValue::F32(v2));
        vec_deque.push_back(GMValue::F32(v3));
        vec_deque.into()
    }
}

impl From<(String, String, GMValue)> for GMValue {
    fn from((tag, message, value): (String, String, GMValue)) -> Self {
        Self::Message(tag, message, Box::new(value))
    }
}

impl From<GMTarget> for GMValue {
    fn from(value: GMTarget) -> Self {
        Self::Target(value)
    }
}

impl From<(GMValue, GMValue)> for GMValue {
    fn from((v1, v2): (GMValue, GMValue)) -> Self {
        let mut vec_deque = VecDeque::new();
        vec_deque.push_back(v1);
        vec_deque.push_back(v2);
        vec_deque.into()
    }
}

impl From<(GMValue, GMValue, GMValue)> for GMValue {
    fn from((v1, v2, v3): (GMValue, GMValue, GMValue)) -> Self {
        let mut vec_deque = VecDeque::new();
        vec_deque.push_back(v1);
        vec_deque.push_back(v2);
        vec_deque.push_back(v3);
        vec_deque.into()
    }
}

impl From<(GMValue, GMValue, GMValue, GMValue)> for GMValue {
    fn from((v1, v2, v3, v4): (GMValue, GMValue, GMValue, GMValue)) -> Self {
        let mut vec_deque = VecDeque::new();
        vec_deque.push_back(v1);
        vec_deque.push_back(v2);
        vec_deque.push_back(v3);
        vec_deque.push_back(v4);
        vec_deque.into()
    }
}

impl From<VecDeque<GMVec2D>> for GMValue {
    fn from(values: VecDeque<GMVec2D>) -> Self {
        let values: VecDeque<GMValue> = values.iter().map(|v| GMValue::Vec2D(*v)).collect();
        Self::Multiple(values)
    }
}

impl From<VecDeque<f32>> for GMValue {
    fn from(values: VecDeque<f32>) -> Self {
        let values: VecDeque<GMValue> = values.iter().map(|v| GMValue::F32(*v)).collect();
        Self::Multiple(values)
    }
}

impl From<VecDeque<GMTarget>> for GMValue {
    fn from(mut values: VecDeque<GMTarget>) -> Self {
        let values: VecDeque<GMValue> = values.drain(0..).map(|v| GMValue::Target(v)).collect();
        Self::Multiple(values)
    }
}

impl From<VecDeque<GMValue>> for GMValue {
    fn from(value: VecDeque<GMValue>) -> Self {
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

impl From<GMRepetition> for GMValue {
    fn from(value: GMRepetition) -> Self {
        Self::Repetition(value)
    }
}

impl From<GMAlign> for GMValue {
    fn from(value: GMAlign) -> Self {
        Self::Align(value)
    }
}

impl From<GMLineMode> for GMValue {
    fn from(value: GMLineMode) -> Self {
        Self::Any(Rc::new(value))
    }
}

impl From<GMAnimation> for GMValue {
    fn from(value: GMAnimation) -> Self {
        Self::Any(Rc::new(value))
    }
}

impl From<GMSprite> for GMValue {
    fn from(value: GMSprite) -> Self {
        Self::Any(Rc::new(value))
    }
}

impl From<Rc<GMTexture>> for GMValue {
    fn from(value: Rc<GMTexture>) -> Self {
        Self::Any(value)
    }
}

impl From<Rc<GMBitmapFont>> for GMValue {
    fn from(value: Rc<GMBitmapFont>) -> Self {
        Self::Any(value)
    }
}

impl From<Box<dyn GMObjectT>> for GMValue {
    fn from(value: Box<dyn GMObjectT>) -> Self {
        Self::Object(value)
    }
}
