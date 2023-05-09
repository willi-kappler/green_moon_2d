
use std::collections::HashMap;

use crate::value::GMValue;

#[derive(Clone, Debug)]
pub struct GMState {
    state: HashMap<String, GMValue>,
}

impl GMState {
    pub fn new() -> Self {
        Self {
            state: HashMap::new(),
        }
    }

    pub fn set_property(&mut self, name: &str, value: GMValue) {
        self.state.insert(name.to_string(), value);
    }

    pub fn get_property(&self, name: &str) -> &GMValue {
        if let Some(value) = self.state.get(name) {
            value
        } else {
            &GMValue::None
        }
    }

    pub fn remove_property(&mut self, name: &str) {
        self.state.remove(name);
    }

    pub fn clear(&mut self) {
        self.state.clear();
    }
}
