
use crate::value::GMValue;

#[derive(Clone, Debug)]
pub struct GMMessage {
    pub tag: String,
    pub method: String,
    pub value: GMValue,
}

pub fn msg0(method: &str) -> GMMessage {
    GMMessage {
        tag: "".to_string(),
        method: method.to_string(),
        value: GMValue::None,
    }
}

pub fn msg1(method: &str, value: GMValue) -> GMMessage {
    GMMessage {
        tag: "".to_string(),
        method: method.to_string(),
        value,
    }
}

pub fn msg2(method: &str, value1: GMValue, value2: GMValue) -> GMMessage {
    GMMessage {
        tag: "".to_string(),
        method: method.to_string(),
        value: value1.chain(value2),
    }
}

pub fn msg3(method: &str, value1: GMValue, value2: GMValue, value3: GMValue) -> GMMessage {
    GMMessage {
        tag: "".to_string(),
        method: method.to_string(),
        value: value1.chain(value2).chain(value3),
    }
}

pub fn msgt0(tag: &str, method: &str) -> GMMessage {
    GMMessage {
        tag: tag.to_string(),
        method: method.to_string(),
        value: GMValue::None,
    }
}

pub fn msgt1(tag: &str, method: &str, value: GMValue) -> GMMessage {
    GMMessage {
        tag: tag.to_string(),
        method: method.to_string(),
        value,
    }
}

pub fn msgt2(tag: &str, method: &str, value1: GMValue, value2: GMValue) -> GMMessage {
    GMMessage {
        tag: tag.to_string(),
        method: method.to_string(),
        value: value1.chain(value2),
    }
}

pub fn msgt3(tag: &str, method: &str, value1: GMValue, value2: GMValue, value3: GMValue) -> GMMessage {
    GMMessage {
        tag: tag.to_string(),
        method: method.to_string(),
        value: value1.chain(value2).chain(value3),
    }
}
