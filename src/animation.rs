
use std::fmt::Debug;

use log::debug;

use crate::timer::GMTimer;
use crate::util::GMRepetition;
use crate::context::GMContext;

pub trait GMAnimationT : Debug {
    fn update(&mut self);

    fn finished(&self) -> bool;

    fn texture_index(&self) -> u32;

    fn clone_box(&self) -> Box<dyn GMAnimationT>;

    fn set_active(&mut self, active: bool);

    fn reverse(&mut self);

    fn send_message(&mut self, _message: &str, _context: &mut GMContext) {
    }
}

impl Clone for Box<dyn GMAnimationT> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}


#[derive(Clone, Debug)]
pub struct GMAnimationStatic {
    texture_index: u32,
}

impl GMAnimationStatic {
    pub fn new(texture_index: u32) -> Self {
        Self {
            texture_index,
        }
    }
}

impl GMAnimationT for GMAnimationStatic {
    fn update(&mut self) {
        // Nothing to do
    }

    fn finished(&self) -> bool {
        true
    }

    fn texture_index(&self) -> u32 {
        self.texture_index
    }

    fn clone_box(&self) -> Box<dyn GMAnimationT> {
        let result = self.clone();

        Box::new(result)
    }

    fn set_active(&mut self, _active: bool) {
        // Nothing to do
    }

    fn reverse(&mut self) {
        // Nothing to do
    }
}


#[derive(Clone, Debug)]
pub struct GMAnimationSimple {
    active: bool,
    current_frame: usize,
    frames: Vec<(u32, f32)>, // index, duration in seconds
    timer: GMTimer,
    repetition: GMRepetition,
}

impl GMAnimationSimple {
    pub fn new(frames: &[(u32, f32)]) -> Self {
        debug!("GMAnimationSimple::new(), frames: '{:?}'", frames);

        Self {
            active: true,
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

    pub fn set_active(&mut self, active: bool) {
        self.active = active;
    }

    pub fn get_active(&self) -> bool {
        self.active
    }

    pub fn set_frame(&mut self, index: usize) {
        self.current_frame = index;
    }

    pub fn get_frame(&self) -> usize {
        self.current_frame
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
    }

    pub fn set_repetition(&mut self, repetition: GMRepetition) {
        self.repetition = repetition;
    }

    pub fn get_repetition(&self) -> GMRepetition {
        self.repetition
    }
}

impl GMAnimationT for GMAnimationSimple {
    fn update(&mut self) {
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

    fn finished(&self) -> bool {
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

    fn texture_index(&self) -> u32 {
        self.texture_index()
    }

    fn clone_box(&self) -> Box<dyn GMAnimationT> {
        let result = self.clone();

        Box::new(result)
    }

    fn set_active(&mut self, active: bool) {
        self.active = active;
    }

    fn reverse(&mut self) {
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

    fn send_message(&mut self, _message: &str, _context: &mut GMContext) {
        // TODO: implement
        todo!();
    }
}

pub struct GMAnimationSimpleBuilder {
    animation: GMAnimationSimple,
}

impl GMAnimationSimpleBuilder {
    pub fn new(frames: &[(u32, f32)]) -> Self {
        Self {
            animation: GMAnimationSimple::new(frames),
        }
    }

    pub fn with_active(mut self, active: bool) -> Self {
        debug!("GMAnimationSimpleBuilder::with_active(), active: '{}'", active);

        self.animation.active = active;
        self
    }

    pub fn with_current_frame(mut self, current_frame: usize) -> Self {
        debug!("GMAnimationSimpleBuilder::with_current_frame(), current_frame: '{}'", current_frame);

        self.animation.current_frame = current_frame;
        self
    }

    pub fn with_repetition(mut self, repetition: GMRepetition) -> Self {
        debug!("GMAnimationSimpleBuilder::with_repetition(), repetition: '{:?}'", repetition);

        self.animation.repetition = repetition;
        self
    }

    pub fn build(self) -> GMAnimationSimple {
        self.animation
    }
}
