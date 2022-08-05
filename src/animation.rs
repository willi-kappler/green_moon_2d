
use std::fmt::Debug;

use log::debug;

use crate::timer::GMTimer;


pub trait GMAnimationT : Debug {
    fn update(&mut self);

    fn finished(&self) -> bool;

    fn texture_index(&self) -> u32;

    fn clone_box(&self) -> Box<dyn GMAnimationT>;

    fn set_active(&mut self, active: bool);

    // TODO: Add reverse() ?
}

impl Clone for Box<dyn GMAnimationT> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}

#[derive(Clone, Debug)]
pub struct GMAnimationBase {
    active: bool,
    current_frame: usize,
    frames: Vec<(u32, f32)>, // index, duration in seconds
    timer: GMTimer,
}

impl GMAnimationBase {
    pub fn new(frames: &[(u32, f32)]) -> Self {
        debug!("GMAnimationBase::new(), frames: {:?}", frames);

        Self {
            active: true,
            current_frame: 0,
            frames: frames.to_vec(),
            timer: GMTimer::new(frames[0].1),
        }
    }

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

    pub fn timer_finished(&mut self) -> bool {
        self.timer.finished()
    }

    pub fn set_new_timer_duration(&mut self) {
        self.timer.set_duration(self.frames[self.current_frame].1);
    }
}


#[derive(Clone, Debug)]
pub struct GMAnimationForwardOnce {
    base: GMAnimationBase,
}

impl GMAnimationForwardOnce {
    pub fn new(frames: &[(u32, f32)]) -> Self {
        debug!("GMAnimationForwardOnce::new(), frames: {:?}", frames);

        Self {
            base: GMAnimationBase::new(frames),
        }
    }
}

impl GMAnimationT for GMAnimationForwardOnce {
    fn update(&mut self) {
        if self.base.get_active() {
            if self.base.timer_finished() {
                if self.base.frame_at_end() {
                    self.set_active(false);
                } else {
                    self.base.inc_frame(1);
                    self.base.set_new_timer_duration();
                }
            }
        }
    }

    fn finished(&self) -> bool {
        self.base.frame_at_end()
    }

    fn texture_index(&self) -> u32 {
        self.base.texture_index()
    }

    fn clone_box(&self) -> Box<dyn GMAnimationT> {
        let result = self.clone();

        Box::new(result)
    }

    fn set_active(&mut self, active: bool) {
        self.base.set_active(active);
    }
}

#[derive(Clone, Debug)]
pub struct GMAnimationForwardLoop {
    base: GMAnimationBase,
}

impl GMAnimationForwardLoop {
    pub fn new(frames: &[(u32, f32)]) -> Self {
        debug!("GMAnimationForwardLoop::new(), frames: {:?}", frames);

        Self {
            base: GMAnimationBase::new(frames),
        }
    }
}

impl GMAnimationT for GMAnimationForwardLoop {
    fn update(&mut self) {
        if self.base.get_active() {
            if self.base.timer_finished() {
                if self.base.frame_at_end() {
                    // Restart animation
                    self.base.set_frame(0);
                } else {
                    self.base.inc_frame(1);
                }
                self.base.set_new_timer_duration();
            }
        }
    }

    fn finished(&self) -> bool {
        false
    }

    fn texture_index(&self) -> u32 {
        self.base.texture_index()
    }

    fn clone_box(&self) -> Box<dyn GMAnimationT> {
        let result = self.clone();

        Box::new(result)
    }

    fn set_active(&mut self, active: bool) {
        self.base.set_active(active);
    }
}

#[derive(Clone, Debug)]
pub struct GMAnimationPingPong {
    base: GMAnimationBase,
    forward: bool,
}

impl GMAnimationPingPong {
    pub fn new(frames: &[(u32, f32)]) -> Self {
        debug!("GMAnimationPingPong::new(), frames: {:?}", frames);

        Self {
            base: GMAnimationBase::new(frames),
            forward: true,
        }
    }
}

impl GMAnimationT for GMAnimationPingPong {
    fn update(&mut self) {
        if self.base.get_active() {
            if self.base.timer_finished() {
                if self.forward {
                    if self.base.frame_at_end() {
                        self.forward = false;
                    } else {
                        self.base.inc_frame(1);
                    }
                } else {
                    if self.base.frame_at_start() {
                        self.forward = true;
                    } else {
                        self.base.dec_frame(1)
                    }
                }
                self.base.set_new_timer_duration();
            }
        }
    }

    fn finished(&self) -> bool {
        false
    }

    fn texture_index(&self) -> u32 {
        self.base.texture_index()
    }

    fn clone_box(&self) -> Box<dyn GMAnimationT> {
        let result = self.clone();

        Box::new(result)
    }

    fn set_active(&mut self, active: bool) {
        self.base.set_active(active);
    }
}
