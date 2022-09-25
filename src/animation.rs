
use std::fmt::Debug;

use log::debug;

use crate::timer::GMTimer;
use crate::util::GMRepetition;
use crate::context::{GMContext, GMObjectMessage};
use crate::data::GMData;
use crate::effect::GMEffectManager;

#[derive(Clone, Debug)]
pub struct GMAnimationBase {
    pub active: bool,
    pub name: String,
    pub current_frame: usize,
    frames: Vec<(u32, f32)>, // index, duration in seconds
    timer: GMTimer,
    pub repetition: GMRepetition,
}

impl GMAnimationBase {
    pub fn new<S: Into<String>>(name: S, frames: &[(u32, f32)]) -> Self {
        debug!("GMAnimationBase::new(), frames: '{:?}'", frames);

        Self {
            active: true,
            name: name.into(),
            current_frame: 0,
            frames: frames.to_vec(),
            timer: GMTimer::new(frames[0].1),
            repetition: GMRepetition::OnceForward,
        }
    }

    // Other methods
    pub fn texture_index(&self) -> u32 {
        self.frames[self.current_frame].0
    }

    pub fn inc_frame(&mut self, amount: usize) {
        self.current_frame += amount;
    }

    pub fn dec_frame(&mut self, amount: usize) {
        self.current_frame -= amount;
    }

    pub fn frame_at_end(&self) -> bool {
        self.current_frame >= self.frames.len() - 1
    }

    pub fn frame_at_start(&self) -> bool {
        self.current_frame == 0
    }

    pub fn last_frame(&self) -> usize {
        self.frames.len() - 1
    }

    pub fn timer_finished(&mut self) -> bool {
        self.timer.finished()
    }

    pub fn set_new_timer_duration(&mut self) {
        self.timer.set_duration(self.frames[self.current_frame].1);
        self.timer.start();
    }

    pub fn update(&mut self) {
        if self.active && self.timer.finished() {
            match self.repetition {
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
                        self.repetition =  GMRepetition::PingPongBackward;
                    } else {
                        self.current_frame += 1;
                    }
                    self.set_new_timer_duration();
                }
                GMRepetition::PingPongBackward => {
                    if self.frame_at_start() {
                        self.repetition =  GMRepetition::PingPongForward;
                    } else {
                        self.current_frame -= 1;
                    }
                    self.set_new_timer_duration();
                }
            }
        }
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
        match self.repetition {
            GMRepetition::OnceForward => {
                self.repetition = GMRepetition::OnceBackward;
            }
            GMRepetition::OnceBackward => {
                self.repetition = GMRepetition::OnceForward;
            }
            GMRepetition::LoopForward => {
                self.repetition = GMRepetition::LoopBackward;
            }
            GMRepetition::LoopBackward => {
                self.repetition = GMRepetition::LoopForward;
            }
            GMRepetition::PingPongForward => {
                self.repetition = GMRepetition::PingPongBackward;
            }
            GMRepetition::PingPongBackward => {
                self.repetition = GMRepetition::PingPongForward;
            }
        }
    }

    pub fn send_message(&mut self, _message: &str, _context: &mut GMContext) {
        todo!();
    }

    pub fn send_message_data(&mut self, _message: &str, _data: GMData, _context: &mut GMContext) {
        todo!();
    }

}

#[derive(Clone, Debug)]
pub struct GMAnimation {
    pub base: GMAnimationBase,
    pub effects: GMEffectManager<GMAnimationBase>,
}

impl GMAnimation {
    pub fn new<S: Into<String>>(name: S, frames: &[(u32, f32)]) -> Self {
        Self {
            base: GMAnimationBase::new(name, frames),
            effects: GMEffectManager::new(),
        }
    }

    pub fn update(&mut self, context: &mut GMContext) {
        self.base.update();

        if self.base.active {
            self.effects.update(&mut self.base, context);
        }
    }

    pub fn check_messages(&mut self, context: &mut GMContext) {
        let mut messages = context.get_object_messages(&self.base.name);

        while let Some(message) = messages.pop_front() {
            match message {
                GMObjectMessage::Simple(message) => {
                    self.base.send_message(&message, context);
                }
                GMObjectMessage::Data(message, data) => {
                    self.base.send_message_data(&message, data, context);
                }
                GMObjectMessage::SimpleEffect(index, message) => {
                    self.effects.send_effect_message(index, &message, context);
                }
                GMObjectMessage::DataEffect(index, message, data) => {
                    self.effects.send_effect_message_data(index, &message, data, context);
                }
            }
        }
    }
}


pub struct GMAnimationBuilder {
    animation: GMAnimation,
}

impl GMAnimationBuilder {
    pub fn new<S: Into<String>>(name: S, frames: &[(u32, f32)]) -> Self {
        Self {
            animation: GMAnimation::new(name, frames),
        }
    }

    pub fn with_active(mut self, active: bool) -> Self {
        debug!("GMAnimationSimpleBuilder::with_active(), active: '{}'", active);

        self.animation.base.active = active;
        self
    }

    pub fn with_current_frame(mut self, current_frame: usize) -> Self {
        debug!("GMAnimationSimpleBuilder::with_current_frame(), current_frame: '{}'", current_frame);

        self.animation.base.current_frame = current_frame;
        self
    }

    pub fn with_repetition(mut self, repetition: GMRepetition) -> Self {
        debug!("GMAnimationSimpleBuilder::with_repetition(), repetition: '{:?}'", repetition);

        self.animation.base.repetition = repetition;
        self
    }

    pub fn build(self) -> GMAnimation {
        self.animation
    }
}
