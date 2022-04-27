

use std::fmt::{self, Debug, Formatter};

use crate::timer::GMTimer;


pub trait GMAnimationT {
    fn update(&mut self) {
    }

    fn finished(&self) -> bool {
        true
    }

    fn texture_index(&self) -> u32;

    fn box_clone(&self) -> Box<dyn GMAnimationT>;

    fn set_active(&mut self, active: bool);

    fn get_active(&self) -> bool;

    fn set_frame(&mut self, index: usize);

    fn get_frame(&self) -> usize;

    fn inc_frame(&mut self, amount: usize);

    fn dec_frame(&mut self, amount: usize);

    fn frame_at_end(&self) -> bool;

    fn frame_at_start(&self) -> bool;

    fn timer_finished(&self) -> bool;

    fn set_new_timer_duration(&mut self);
}

impl Clone for Box<dyn GMAnimationT> {
    fn clone(&self) -> Self {
        self.box_clone()
    }
}

impl Debug for Box<dyn GMAnimationT> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "GMAnimationT")
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
        Self {
            active: true,
            current_frame: 0,
            frames: frames.to_vec(),
            timer: GMTimer::new(frames[0].1),
        }
    }
}

impl GMAnimationT for GMAnimationBase {
    fn texture_index(&self) -> u32 {
        self.frames[self.current_frame].0
    }

    fn box_clone(&self) -> Box<dyn GMAnimationT> {
        let result = self.clone();

        Box::new(result)
    }

    fn set_active(&mut self, active: bool) {
        self.active = active;
    }

    fn get_active(&self) -> bool {
        self.active
    }

    fn set_frame(&mut self, index: usize) {
        self.current_frame = index;
    }

    fn get_frame(&self) -> usize {
        self.current_frame
    }

    fn inc_frame(&mut self, amount: usize) {
        self.current_frame += amount;
    }

    fn dec_frame(&mut self, amount: usize) {
        self.current_frame -= amount;
    }

    fn frame_at_end(&self) -> bool {
        self.current_frame >= self.frames.len() - 1
    }

    fn frame_at_start(&self) -> bool {
        self.current_frame == 0
    }

    fn timer_finished(&self) -> bool {
        self.timer.finished()
    }

    fn set_new_timer_duration(&mut self) {
        self.timer.set_duration(self.frames[self.current_frame].1);
    }
}

#[derive(Clone, Debug)]
pub struct GMAnimationForwardOnce {
    base: GMAnimationBase,
}

impl GMAnimationForwardOnce {
    pub fn new(frames: &[(u32, f32)]) -> Self {
        Self {
            base: GMAnimationBase::new(frames),
        }
    }
}

impl GMAnimationT for GMAnimationForwardOnce {
    fn update(&mut self) {
        if self.get_active() {
            if self.timer_finished() {
                if self.frame_at_end() {
                    self.set_active(false);
                } else {
                    self.inc_frame(1);
                    self.set_new_timer_duration();
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

    fn box_clone(&self) -> Box<dyn GMAnimationT> {
        let result = self.clone();

        Box::new(result)
    }

    fn set_active(&mut self, active: bool) {
        self.base.set_active(active);
    }

    fn get_active(&self) -> bool {
        self.base.get_active()
    }

    fn set_frame(&mut self, index: usize) {
        self.base.set_frame(index);
    }

    fn get_frame(&self) -> usize {
        self.base.get_frame()
    }

    fn inc_frame(&mut self, amount: usize) {
        self.base.inc_frame(amount);
    }

    fn dec_frame(&mut self, amount: usize) {
        self.base.dec_frame(amount);
    }

    fn frame_at_end(&self) -> bool {
        self.base.frame_at_end()
    }

    fn frame_at_start(&self) -> bool {
        self.base.frame_at_start()
    }

    fn timer_finished(&self) -> bool {
        self.base.timer_finished()
    }

    fn set_new_timer_duration(&mut self) {
        self.base.set_new_timer_duration();
    }
}

#[derive(Clone, Debug)]
pub struct GMAnimationForwardLoop {
    base: GMAnimationBase,
}

impl GMAnimationForwardLoop {
    pub fn new(frames: &[(u32, f32)]) -> Self {
        Self {
            base: GMAnimationBase::new(frames),
        }
    }
}

impl GMAnimationT for GMAnimationForwardLoop {
    fn update(&mut self) {
        if self.get_active() {
            if self.timer_finished() {
                if self.frame_at_end() {
                    // Restart animation
                    self.set_frame(0);
                } else {
                    self.inc_frame(1);
                }
                self.set_new_timer_duration();
            }
        }
    }

    fn finished(&self) -> bool {
        false
    }

    fn texture_index(&self) -> u32 {
        self.base.texture_index()
    }

    fn box_clone(&self) -> Box<dyn GMAnimationT> {
        let result = self.clone();

        Box::new(result)
    }

    fn set_active(&mut self, active: bool) {
        self.base.set_active(active);
    }

    fn get_active(&self) -> bool {
        self.base.get_active()
    }

    fn set_frame(&mut self, index: usize) {
        self.base.set_frame(index);
    }

    fn get_frame(&self) -> usize {
        self.base.get_frame()
    }

    fn inc_frame(&mut self, amount: usize) {
        self.base.inc_frame(amount);
    }

    fn dec_frame(&mut self, amount: usize) {
        self.base.dec_frame(amount);
    }

    fn frame_at_end(&self) -> bool {
        self.base.frame_at_end()
    }

    fn frame_at_start(&self) -> bool {
        self.base.frame_at_start()
    }

    fn timer_finished(&self) -> bool {
        self.base.timer_finished()
    }

    fn set_new_timer_duration(&mut self) {
        self.base.set_new_timer_duration();
    }
}

#[derive(Clone, Debug)]
pub struct GMAnimationPingPong {
    base: GMAnimationBase,
    forward: bool,
}

impl GMAnimationPingPong {
    pub fn new(frames: &[(u32, f32)]) -> Self {
        Self {
            base: GMAnimationBase::new(frames),
            forward: true,
        }
    }
}

impl GMAnimationT for GMAnimationPingPong {
    fn update(&mut self) {
        if self.get_active() {
            if self.timer_finished() {
                if self.forward {
                    if self.frame_at_end() {
                        self.forward = false;
                    } else {
                        self.inc_frame(1);
                    }
                } else {
                    if self.frame_at_start() {
                        self.forward = true;
                    } else {
                        self.dec_frame(1)
                    }
                }
                self.set_new_timer_duration();
            }
        }
    }

    fn finished(&self) -> bool {
        false
    }

    fn texture_index(&self) -> u32 {
        self.base.texture_index()
    }

    fn box_clone(&self) -> Box<dyn GMAnimationT> {
        let result = self.clone();

        Box::new(result)
    }

    fn set_active(&mut self, active: bool) {
        self.base.set_active(active);
    }

    fn get_active(&self) -> bool {
        self.base.get_active()
    }

    fn set_frame(&mut self, index: usize) {
        self.base.set_frame(index);
    }

    fn get_frame(&self) -> usize {
        self.base.get_frame()
    }

    fn inc_frame(&mut self, amount: usize) {
        self.base.inc_frame(amount);
    }

    fn dec_frame(&mut self, amount: usize) {
        self.base.dec_frame(amount);
    }

    fn frame_at_end(&self) -> bool {
        self.base.frame_at_end()
    }

    fn frame_at_start(&self) -> bool {
        self.base.frame_at_start()
    }

    fn timer_finished(&self) -> bool {
        self.base.timer_finished()
    }

    fn set_new_timer_duration(&mut self) {
        self.base.set_new_timer_duration();
    }
}
