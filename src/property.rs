
use std::fmt::Debug;
use std::any::Any;

use std::collections::HashMap;

pub trait GMCustomT : Debug {
    fn clone_box(&self) -> Box<dyn GMCustomT>;

    fn to_any(&self) -> Box<dyn Any>;
}

impl Clone for Box<dyn GMCustomT> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}

#[derive(Clone, Debug)]
pub enum GMValue {
    None,

    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),

    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),

    F32(f32),
    F64(f64),

    Char(char),
    String(String),

    Tuple(Box<GMValue>, Box<GMValue>),
    Vector(Vec<GMValue>),

    Custom(Box<dyn GMCustomT>),
}

#[derive(Clone, Debug)]
pub struct GMPropertyManager {
    properties: HashMap<String, GMValue>,
}

impl GMPropertyManager {
    pub fn new() -> Self {
        Self {
            properties: HashMap::new(),
        }
    }

    pub fn add_property(&mut self, name: &str, value: GMValue) {
        self.properties.insert(name.to_string(), value);
    }

    pub fn add_tag(&mut self, name: &str) {
        self.properties.insert(name.to_string(), GMValue::None);
    }

    pub fn remove_property(&mut self, name: &str) {
        self.properties.remove(name);
    }

    pub fn get_property(&self, name: &str) -> Option<&GMValue> {
        self.properties.get(name)
    }

    pub fn has_property(&self, name: &str) -> bool {
        self.properties.contains_key(name)
    }
}
