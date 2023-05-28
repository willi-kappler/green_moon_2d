

use std::time::Instant;

use crate::message::GMMessage;
use crate::value::GMValue;
use crate::util::{error_panic, send_message_f32, send_message_bool};

#[derive(Clone, Debug)]
pub struct GMTimer {
    pub active: bool,
    pub duration: f32,
    instant: Instant,
}

impl GMTimer {
    pub fn new(duration: f32) -> Self {
        Self {
            active: true,
            duration,
            instant: Instant::now(),
        }
    }

    pub fn finished(&mut self) -> bool {
        if self.active {
            if self.instant.elapsed().as_secs_f32() >= self.duration {
                self.active = false;
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    pub fn start(&mut self) {
        self.instant = Instant::now();
        self.active = true;
    }

    pub fn send_message(&mut self, mut message: GMMessage) -> GMValue {
        let tag = message.next_tag();
        let method = message.method.as_str();
        let value = message.value.clone();

        match tag.as_str() {
            "" => {
                match method {
                    "finished" => {
                        return self.finished().into();
                    }
                    "start" => {
                        self.start();
                    }
                    _ => {
                        error_panic(&format!("GMtimer::send_message, unknown method: '{}', no tag", method));
                    }
                }
            }
            "active" => {
                return send_message_bool(&mut self.active, method, value);
            }
            "duration" => {
                return send_message_f32(&mut self.duration, method, value);
            }
            _ => {
                error_panic(&format!("GMtimer::send_message, unknown tag: '{}'", tag));
            }
        }

        GMValue::None
    }
}
