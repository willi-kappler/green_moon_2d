
use std::fmt::Debug;
use std::collections::HashSet;

use log::debug;

use crate::timer::GMTimer;
use crate::util::{GMRepetition, error_panic};
use crate::context::GMContext;
use crate::data::GMData;
use crate::effect::GMEffectManager;
use crate::object_manager::{GMObjectBaseT, GMObjectManager};

use crate::return_name_and_groups;

#[derive(Clone, Debug)]
pub struct GMAnimationBase {
    pub active: bool,
    pub name: String,
    pub groups: HashSet<String>,
    pub current_frame: usize,
    pub frames: Vec<(u32, f32)>, // index, duration in seconds
    pub timer: GMTimer,
    pub repetition: GMRepetition,
}

impl GMAnimationBase {
    pub fn new<S: Into<String>>(name: S, frames: &[(u32, f32)]) -> Self {
        let name = name.into();

        debug!("GMAnimationBase::new(), name: '{}', frames: '{:?}'", name, frames);

        Self {
            active: true,
            name,
            groups: HashSet::new(),
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
}

impl GMObjectBaseT for GMAnimationBase {
    fn update(&mut self, _context: &mut GMContext) {
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

    fn send_message(&mut self, message: &str, data: GMData, _context: &mut GMContext) {
        match message {
            "set_new_timer_duration" => {
                self.set_new_timer_duration();
            }
            "reverse" => {
                self.reverse();
            }
            "inc_frame" => {
                self.inc_frame(data.into());
            }
            "dec_frame" => {
                self.dec_frame(data.into());
            }
            "set_active" => {
                self.active = data.into();
            }
            "set_name" => {
                self.name = data.into();
            }
            "add_group" => {
                self.groups.insert(data.into());
            }
            "remove_group" => {
                let group: String = data.into();
                self.groups.remove(&group);
            }
            "clear_group" => {
                self.groups.clear();
            }
            _ => {
                error_panic(&format!("GMAnimationBase::send_message(), unknown message: '{}'", message))
            }
        }
    }

    return_name_and_groups!();
}


pub type GMAnimation = GMObjectManager<GMAnimationBase>;

impl GMAnimation {
    pub fn new<S: Into<String>>(name: S, frames: &[(u32, f32)]) -> Self {
        Self {
            base: GMAnimationBase::new(name, frames),
            effects: GMEffectManager::new(),
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
        debug!("GMAnimationBuilder::with_active(), active: '{}'", active);

        self.animation.base.active = active;
        self
    }

    pub fn with_name<S: Into<String>>(mut self, name: S) -> Self {
        let name = name.into();
        debug!("GMAnimationBuilder::with_name(), name: '{}'", name);

        self.animation.base.name = name;
        self
    }

    pub fn with_group<S: Into<String>>(mut self, group: S) -> Self {
        let group = group.into();
        debug!("GMAnimationBuilder::with_group(), group: '{}'", group);

        self.animation.base.groups.insert(group);
        self
    }

    pub fn with_groups(mut self, groups: HashSet<String>) -> Self {
        debug!("GMAnimationBuilder::with_groups(), groups: '{:?}'", groups);

        self.animation.base.groups = groups;
        self
    }

    pub fn with_current_frame(mut self, current_frame: usize) -> Self {
        debug!("GMAnimationBuilder::with_current_frame(), current_frame: '{}'", current_frame);

        self.animation.base.current_frame = current_frame;
        self
    }

    pub fn with_repetition(mut self, repetition: GMRepetition) -> Self {
        debug!("GMAnimationBuilder::with_repetition(), repetition: '{:?}'", repetition);

        self.animation.base.repetition = repetition;
        self
    }

    pub fn build(self) -> GMAnimation {
        self.animation
    }
}
