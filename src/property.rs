use std::collections::HashMap;
use std::any::Any;

#[derive(Clone, Debug)]
pub enum GMValue {
    None,
    U8(u8),
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
}
