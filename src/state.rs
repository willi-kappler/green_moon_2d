
use std::collections::HashMap;

use crate::value::GMValue;

#[derive(Clone, Debug)]
pub struct GMState {
    pub state: HashMap<String, GMValue>,
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

    pub fn get_property_mut(&mut self, name: &str) -> &mut GMValue {
        self.state.get_mut(name).unwrap()
    }

    pub fn remove_property(&mut self, name: &str) {
        self.state.remove(name);
    }

    pub fn clear(&mut self) {
        self.state.clear();
    }
}

impl Into<GMValue> for GMState {
    fn into(self) -> GMValue {
        GMValue::State(self)
    }
}
