
use std::collections::VecDeque;
use std::mem;

use crate::util::{error_panic, send_message_str};
use crate::value::GMValue;

#[derive(Clone, Debug)]
pub struct GMMessage {
    pub tags: VecDeque<String>,
    pub method: String,
    pub value: GMValue,
}

impl GMMessage {
    pub fn new(method: &str) -> Self {
        Self {
            tags: VecDeque::new(),
            method: method.to_string(),
            value: GMValue::None,
        }
    }

    pub fn new2<V: Into<GMValue>>(method: &str, value: V) -> Self {
        Self {
            tags: VecDeque::new(),
            method: method.to_string(),
            value: value.into(),
        }
    }

    pub fn new3<T: Into<GMTags>>(tag: T, method: &str) -> Self {
        Self {
            tags: tag.into().tags,
            method: method.to_string(),
            value: GMValue::None,
        }
    }

    pub fn new4<T: Into<GMTags>, V: Into<GMValue>>(tag: T, method: &str, value: V) -> Self {
        Self {
            tags: tag.into().tags,
            method: method.to_string(),
            value: value.into(),
        }
    }

    pub fn next_tag(&mut self) -> String {
        if self.tags.is_empty() {
            String::new()
        } else {
            self.tags.pop_front().unwrap()
        }
    }

    pub fn take_value(&mut self) -> GMValue {
        mem::take(&mut self.value)
    }

    pub fn send_message(&mut self, mut message: GMMessage) -> GMValue {
        let tag = message.next_tag();
        let method = message.method.as_str();
        let value = message.value;

        match tag.as_str() {
            "tags" => {
                match method {
                    "set" => {
                        let (index, new_tag) = value.into_generic::<(usize, String)>();
                        self.tags[index] = new_tag;
                    }
                    "get" => {
                        let index = value.into_usize();
                        return self.tags[index].clone().into();
                    }
                    "push_back" => {
                        let new_tag = value.into_string();
                        self.tags.push_back(new_tag);
                    }
                    "push_front" => {
                        let new_tag = value.into_string();
                        self.tags.push_front(new_tag);
                    }
                    _ => {
                        error_panic(&format!("GMMessage::send_message, tag: 'tags' unknown method: '{}'", method));
                    }
                }
            }
            "method" => {
                return send_message_str(&mut self.method, method, value);
            }
            "value" => {
                return self.value.send_message(method, value);
            }
            _ => {
                error_panic(&format!("GMMessage::send_message, unknown tag: '{}'", tag));
            }
        }

        GMValue::None
    }
}

pub struct GMTags {
    pub tags: VecDeque<String>,
}

impl From<&str> for GMTags {
    fn from(tag: &str) -> Self {
        Self {
            tags: VecDeque::from([tag.to_string()]),
        }
    }
}

impl From<(&str, &str)> for GMTags {
    fn from((tag1, tag2): (&str, &str)) -> Self {
        Self {
            tags: VecDeque::from([tag1.to_string(), tag2.to_string()]),
        }
    }
}

impl From<(&str, &str, &str)> for GMTags {
    fn from((tag1, tag2, tag3): (&str, &str, &str)) -> Self {
        Self {
            tags: VecDeque::from([tag1.to_string(), tag2.to_string(), tag3.to_string()]),
        }
    }
}

impl From<&[&str]> for GMTags {
    fn from(tags: &[&str]) -> Self {
        Self {
            tags: VecDeque::from(tags.iter().map(|s| s.to_string()).collect::<Vec<String>>()),
        }
    }
}

impl From<String> for GMTags {
    fn from(tag: String) -> Self {
        Self {
            tags: VecDeque::from([tag]),
        }
    }
}

pub fn msg0v(method: &str) -> GMMessage {
    GMMessage::new(method)
}

pub fn msg1v<V: Into<GMValue>>(method: &str, value: V) -> GMMessage {
    GMMessage::new2(method, value)
}

pub fn msg2v<V1: Into<GMValue>, V2: Into<GMValue>>(method: &str, value1: V1, value2: V2) -> GMMessage {
    GMMessage::new2(method, value1.into().chain(value2.into()))
}

pub fn msg3v<V1: Into<GMValue>, V2: Into<GMValue>, V3: Into<GMValue>>(method: &str, value1: V1, value2: V2, value3: V3) -> GMMessage {
    GMMessage::new2(method, value1.into().chain(value2.into()).chain(value3.into()))
}

pub fn msgt0v<T: Into<GMTags>>(tag: T, method: &str) -> GMMessage {
    GMMessage::new3(tag, method)
}

pub fn msgt1v<T: Into<GMTags>, V: Into<GMValue>>(tag: T, method: &str, value: V) -> GMMessage {
    GMMessage::new4(tag, method, value)
}

pub fn msgt2v<T: Into<GMTags>, V1: Into<GMValue>, V2: Into<GMValue>>(tag: T, method: &str, value1: V1, value2: V2) -> GMMessage {
    GMMessage::new4(tag, method, value1.into().chain(value2.into()))
}

pub fn msgt3v<T: Into<GMTags>, V1: Into<GMValue>, V2: Into<GMValue>, V3: Into<GMValue>>(tag: T, method: &str, value1: V1, value2: V2, value3: V3) -> GMMessage {
    GMMessage::new4(tag, method, value1.into().chain(value2.into()).chain(value3.into()))
}
