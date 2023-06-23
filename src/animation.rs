
use std::fmt::Debug;

use log::debug;

use crate::timer::GMTimer;
use crate::util::{GMRepetition, error_panic, send_message_bool, send_message_usize};
use crate::value::GMValue;
use crate::message::GMMessage;

#[derive(Clone, Debug)]
pub struct GMAnimation {
    pub active: bool,
    pub current_frame: usize,
    pub repetition: GMRepetition,
    frames: Vec<(u32, f32)>, // (index, duration in seconds)
    timer: GMTimer,
}

impl GMAnimation {
    pub fn new(frames: &[(u32, f32)], repetition: GMRepetition) -> Self {
        debug!("GMAnimation::new(), number of frames: {}, repetition: {:?}", frames.len(), repetition);

        Self {
            active: true,
            current_frame: 0,
            repetition,
            frames: frames.to_vec(),
            timer: GMTimer::new(frames[0].1),
        }
    }

    pub fn new2(frame: u32) -> Self {
        Self::new(&[(frame, 0.0)], GMRepetition::Fixed)
    }

    pub fn texture_index(&self) -> u32 {
        self.frames[self.current_frame].0
    }

    pub fn frame_at_start(&self) -> bool {
        self.current_frame == 0
    }

    pub fn frame_at_end(&self) -> bool {
        self.current_frame >= self.frames.len() - 1
    }

    fn set_new_timer_duration(&mut self) {
        self.timer.duration = self.frames[self.current_frame].1;
        self.timer.start();
    }

    pub fn finished(&self) -> bool {
        match self.repetition {
            GMRepetition::Fixed => {
                true
            }
            GMRepetition::OnceForward => {
                self.frame_at_end()
            }
            GMRepetition::OnceBackward => {
                self.frame_at_start()
            }
            _ => {
                false
            }
        }
    }

    pub fn update(&mut self) {
        if self.active && self.timer.finished() {
            match self.repetition {
                GMRepetition::Fixed =>{
                    // Nothing to do
                }
                GMRepetition::OnceForward => {
                    if self.frame_at_end() {
                        self.active = false;
                    } else {
                        self.current_frame += 1;
                        self.set_new_timer_duration();
                    }
                }
                GMRepetition::OnceBackward => {
                    if self.frame_at_start() {
                        self.active = false;
                    } else {
                        self.current_frame -= 1;
                        self.set_new_timer_duration();
                    }
                }
                GMRepetition::LoopForward => {
                    if self.frame_at_end() {
                        // Restart animation
                        self.current_frame = 0;
                    } else {
                        self.current_frame += 1;
                    }
                    self.set_new_timer_duration();
                }
                GMRepetition::LoopBackward => {
                    if self.frame_at_start() {
                        // Restart animation
                        self.current_frame = self.frames.len() - 1;
                    } else {
                        self.current_frame -= 1;
                    }
                    self.set_new_timer_duration();
                }
                GMRepetition::PingPongForward => {
                    if self.frame_at_end() {
                        self.repetition.reverse();
                    } else {
                        self.current_frame += 1;
                    }
                    self.set_new_timer_duration();
                }
                GMRepetition::PingPongBackward => {
                    if self.frame_at_start() {
                        self.repetition.reverse();
                    } else {
                        self.current_frame -= 1;
                    }
                    self.set_new_timer_duration();
                }
            }
        }
    }

    pub fn send_message(&mut self, mut message: GMMessage) -> GMValue {
        let tag = message.next_tag();
        let method = message.method.as_str();
        let value = message.value;

        match tag.as_str() {
            "" => {
                match method {
                    "get" => {
                        return self.clone().into()
                    }
                    "set" => {
                        *self = value.into_animation();
                    }
                    "texture_index" => {
                        return self.texture_index().into()
                    }
                    "frame_at_start" => {
                        return self.frame_at_start().into()
                    }
                    "frame_at_end" => {
                        return self.frame_at_end().into()
                    }
                    "finished" => {
                        return self.finished().into()
                    }
                    "update" => {
                        self.update();
                    }
                    _ => {
                        error_panic(&format!("GMAnimation::send_message, unknown method: '{}', no tag", method));
                    }
                }
            }
            "active" => {
                return send_message_bool(&mut self.active, method, value);
            }
            "frame" => {
                return send_message_usize(&mut self.current_frame, method, value);
            }
            "repetition" => {
                return self.repetition.send_message(method, value);
            }
            _ => {
                error_panic(&format!("GMAnimation::send_message, unknown tag: '{}'", tag));
            }
        }

        GMValue::None
    }
}
