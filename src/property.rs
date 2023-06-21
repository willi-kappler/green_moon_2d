
use std::collections::HashMap;

use crate::value::GMValue;

#[derive(Clone, Debug)]
pub struct GMProperty {
    pub state: HashMap<String, GMValue>,
}

impl GMProperty {
    pub fn new() -> Self {
        Self {
            state: HashMap::new(),
        }
    }

    pub fn set_property<T: Into<GMValue>>(&mut self, property: &str, value: T) {
        self.state.insert(property.to_string(), value.into());
    }

    pub fn get_property(&self, property: &str) -> &GMValue {
        if let Some(value) = self.state.get(property) {
            value
        } else {
            &GMValue::None
        }
    }

    pub fn get_property_mut(&mut self, property: &str) -> &mut GMValue {
        self.state.get_mut(property).unwrap()
    }

    pub fn remove_property(&mut self, property: &str) {
        self.state.remove(property);
    }

    pub fn clear(&mut self) {
        self.state.clear();
    }
}
