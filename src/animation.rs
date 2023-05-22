
use std::fmt::Debug;

use log::debug;

use crate::message::GMMessage;
use crate::timer::GMTimer;
use crate::util::{GMRepetition};
use crate::value::GMValue;
use crate::object_base::{GMValueBoolBase, GMValueUSizeBase};

#[derive(Clone, Debug)]
pub struct GMAnimation {
    pub active: GMValueBoolBase,
    pub current_frame: GMValueUSizeBase,
    pub repetition: GMRepetition,
    frames: Vec<(u32, f32)>, // (index, duration in seconds)
    timer: GMTimer,
}

impl GMAnimation {
    pub fn new(frames: &[(u32, f32)], repetition: GMRepetition) -> Self {
        debug!("GMAnimation::new(), number of frames: {}, repetition: {:?}", frames.len(), repetition);

        Self {
            active: GMValueBoolBase::new(true, "active"),
            current_frame: GMValueUSizeBase::new(0, "current_frame"),
            repetition,
            frames: frames.to_vec(),
            timer: GMTimer::new(frames[0].1),
        }
    }

    pub fn texture_index(&self) -> u32 {
        self.frames[self.current_frame.value].0
    }

    pub fn frame_at_start(&self) -> bool {
        self.current_frame.value == 0
    }

    pub fn frame_at_end(&self) -> bool {
        self.current_frame.value >= self.frames.len() - 1
    }

    fn set_new_timer_duration(&mut self) {
        self.timer.duration = self.frames[self.current_frame.value].1;
        self.timer.start();
    }

    pub fn finished(&self) -> bool {
        match self.repetition {
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

    pub fn reverse(&mut self) {
        self.repetition.reverse();
    }

    pub fn update(&mut self) {
        if self.active.value && self.timer.finished() {
            match self.repetition {
                GMRepetition::OnceForward => {
                    if self.frame_at_end() {
                        self.active.value = false;
                    } else {
                        self.current_frame.value += 1;
                        self.set_new_timer_duration();
                    }
                }
                GMRepetition::OnceBackward => {
                    if self.frame_at_start() {
                        self.active.value = false;
                    } else {
                        self.current_frame.value -= 1;
                        self.set_new_timer_duration();
                    }
                }
                GMRepetition::LoopForward => {
                    if self.frame_at_end() {
                        // Restart animation
                        self.current_frame.value = 0;
                    } else {
                        self.current_frame.value += 1;
                    }
                    self.set_new_timer_duration();
                }
                GMRepetition::LoopBackward => {
                    if self.frame_at_start() {
                        // Restart animation
                        self.current_frame.value = self.frames.len() - 1;
                    } else {
                        self.current_frame.value -= 1;
                    }
                    self.set_new_timer_duration();
                }
                GMRepetition::PingPongForward => {
                    if self.frame_at_end() {
                        self.repetition.reverse();
                    } else {
                        self.current_frame.value += 1;
                    }
                    self.set_new_timer_duration();
                }
                GMRepetition::PingPongBackward => {
                    if self.frame_at_start() {
                        self.repetition.reverse();
                    } else {
                        self.current_frame.value -= 1;
                    }
                    self.set_new_timer_duration();
                }
            }
        }
    }

    pub fn send_message(&mut self, message: GMMessage) -> GMValue {
        match message {
            GMMessage::Update => {
                self.update();
                GMValue::None
            }
            GMMessage::Custom0(name) if name == "get_texture_index" => {
                self.texture_index().into()
            }
            GMMessage::Custom0(name) if name == "frame_at_start" => {
                self.frame_at_start().into()
            }
            GMMessage::Custom0(name) if name == "frame_at_end" => {
                self.frame_at_end().into()
            }
            GMMessage::Custom0(name) if name == "finished" => {
                self.finished().into()
            }
            _ => {
                self.active.send_message(message)
                    .handle(|m| self.current_frame.send_message(m))
                    .handle(|m| self.repetition.send_message(m))
            }
        }
    }
}
