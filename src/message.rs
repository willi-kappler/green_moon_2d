
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

pub fn msg1<V: Into<GMValue>>(method: &str, value: V) -> GMMessage {
    GMMessage {
        tag: "".to_string(),
        method: method.to_string(),
        value: value.into(),
    }
}

pub fn msg2<V1: Into<GMValue>, V2: Into<GMValue>>(method: &str, value1: V1, value2: V2) -> GMMessage {
    GMMessage {
        tag: "".to_string(),
        method: method.to_string(),
        value: value1.into().chain(value2.into()),
    }
}

pub fn msg3<V1: Into<GMValue>, V2: Into<GMValue>, V3: Into<GMValue>>(method: &str, value1: V1, value2: V2, value3: V3) -> GMMessage {
    GMMessage {
        tag: "".to_string(),
        method: method.to_string(),
        value: value1.into().chain(value2.into()).chain(value3.into()),
    }
}

pub fn msgt0(tag: &str, method: &str) -> GMMessage {
    GMMessage {
        tag: tag.to_string(),
        method: method.to_string(),
        value: GMValue::None,
    }
}

pub fn msgt1<V: Into<GMValue>>(tag: &str, method: &str, value: V) -> GMMessage {
    GMMessage {
        tag: tag.to_string(),
        method: method.to_string(),
        value: value.into(),
    }
}

pub fn msgt2<V1: Into<GMValue>, V2: Into<GMValue>>(tag: &str, method: &str, value1: V1, value2: V2) -> GMMessage {
    GMMessage {
        tag: tag.to_string(),
        method: method.to_string(),
        value: value1.into().chain(value2.into()),
    }
}

pub fn msgt3<V1: Into<GMValue>, V2: Into<GMValue>, V3: Into<GMValue>>(tag: &str, method: &str, value1: V1, value2: V2, value3: V3) -> GMMessage {
    GMMessage {
        tag: tag.to_string(),
        method: method.to_string(),
        value: value1.into().chain(value2.into()).chain(value3.into()),
    }
}
