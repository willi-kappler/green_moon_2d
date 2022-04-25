

use std::fmt::{self, Debug, Formatter};

use crate::timer::GMTimer;

#[derive(Clone, Debug)]
pub struct GMAnimationCommon {
    pub name: String,
    pub active: bool,
    pub current_frame: usize,
    pub frames: Vec<(u32, f32)>, // index, duration in seconds
    pub timer: GMTimer,
}

impl GMAnimationCommon {
    pub fn new(name: &str, frames: &[(u32, f32)]) -> Self {
        Self {
            name: name.to_string(),
            active: true,
            current_frame: 0,
            frames: frames.to_vec(),
            timer: GMTimer::new(frames[0].1),
        }
    }
}

pub trait GMAnimationT {
    fn update(&mut self);
    fn finished(&self) -> bool;
    fn frame_index(&self) -> u32;
    fn get_common_ref(&self) -> &GMAnimationCommon;
    fn get_common_mut_ref(&mut self) -> &mut GMAnimationCommon;
    fn box_clone(&self) -> Box<dyn GMAnimationT>;
}

impl Clone for Box<dyn GMAnimationT> {
    fn clone(&self) -> Self {
        self.box_clone()
    }
}

impl Debug for Box<dyn GMAnimationT> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "GMAnimationT: '{}'", self.get_common_ref().name)
    }
}

#[derive(Clone, Debug)]
pub struct GMAnimationStatic {
    pub common: GMAnimationCommon,
}

impl GMAnimationStatic {
    pub fn new() -> Self {
        Self {
            common: GMAnimationCommon::new("static", &[]),
        }
    }
}

impl GMAnimationT for GMAnimationStatic {
    fn update(&mut self) {}

    fn finished(&self) -> bool {
        true
    }

    fn frame_index(&self) -> u32 {
        0
    }

    fn get_common_ref(&self) -> &GMAnimationCommon {
        &self.common
    }

    fn get_common_mut_ref(&mut self) -> &mut GMAnimationCommon {
        &mut self.common
    }

    fn box_clone(&self) -> Box<dyn GMAnimationT> {
        let result = GMAnimationStatic::new();
        Box::new(result)
    }
}

#[derive(Clone, Debug)]
pub struct GMAnimationForwardOnce {
    pub common: GMAnimationCommon,
}

impl GMAnimationForwardOnce {
    pub fn new(name: &str, frames: &[(u32, f32)]) -> Self {
        Self {
            common: GMAnimationCommon::new(name, frames),
        }
    }
}

impl GMAnimationT for GMAnimationForwardOnce {
    fn update(&mut self) {
        if self.common.active {
            if self.common.timer.finished() {
                if self.common.current_frame < self.common.frames.len() - 1 {
                    self.common.current_frame += 1;
                    self.common.timer.set_duration(self.common.frames[self.common.current_frame].1);
                } else {
                    self.common.active = false;
                }
            }
        }
    }

    fn finished(&self) -> bool {
        self.common.current_frame == self.common.frames.len() - 1
    }

    fn frame_index(&self) -> u32 {
        self.common.frames[self.common.current_frame].0
    }

    fn get_common_ref(&self) -> &GMAnimationCommon {
        &self.common
    }

    fn get_common_mut_ref(&mut self) -> &mut GMAnimationCommon {
        &mut self.common
    }

    fn box_clone(&self) -> Box<dyn GMAnimationT> {
        let result = self.clone();
        Box::new(result)
    }
}

#[derive(Clone, Debug)]
pub struct GMAnimationForwardLoop {
    pub common: GMAnimationCommon,
}

impl GMAnimationForwardLoop {
    pub fn new(name: &str, frames: &[(u32, f32)]) -> Self {
        Self {
            common: GMAnimationCommon::new(name, frames),
        }
    }
}

impl GMAnimationT for GMAnimationForwardLoop {
    fn update(&mut self) {
        if self.common.active {
            if self.common.timer.finished() {
                self.common.current_frame += 1;
                if self.common.current_frame >= self.common.frames.len() {
                    self.common.current_frame = 0;
                }
                self.common.timer.set_duration(self.common.frames[self.common.current_frame].1)
            }
        }
    }

    fn finished(&self) -> bool {
        false
    }

    fn frame_index(&self) -> u32 {
        self.common.frames[self.common.current_frame].0
    }

    fn get_common_ref(&self) -> &GMAnimationCommon {
        &self.common
    }

    fn get_common_mut_ref(&mut self) -> &mut GMAnimationCommon {
        &mut self.common
    }

    fn box_clone(&self) -> Box<dyn GMAnimationT> {
        let result = self.clone();
        Box::new(result)
    }
}

#[derive(Clone, Debug)]
pub struct GMAnimationPingPong {
    pub common: GMAnimationCommon,
    pub foreward: bool,
}

impl GMAnimationPingPong {
    pub fn new(name: &str, frames: &[(u32, f32)]) -> Self {
        Self {
            common: GMAnimationCommon::new(name, frames),
            foreward: true,
        }
    }
}

impl GMAnimationT for GMAnimationPingPong {
    fn update(&mut self) {
        if self.common.active {
            if self.common.timer.finished() {
                if self.foreward {
                    self.common.current_frame += 1;
                    if self.common.current_frame >= self.common.frames.len() {
                        self.common.current_frame = self.common.frames.len() - 2;
                        self.foreward = false;
                    }
                } else {
                    if self.common.current_frame > 0 {
                        self.common.current_frame -= 1;
                    } else {
                        self.foreward = true;
                    }
                }
                self.common.timer.set_duration(self.common.frames[self.common.current_frame].1)
            }
        }
    }

    fn finished(&self) -> bool {
        false
    }

    fn frame_index(&self) -> u32 {
        self.common.frames[self.common.current_frame].0
    }

    fn get_common_ref(&self) -> &GMAnimationCommon {
        &self.common
    }

    fn get_common_mut_ref(&mut self) -> &mut GMAnimationCommon {
        &mut self.common
    }

    fn box_clone(&self) -> Box<dyn GMAnimationT> {
        let result = self.clone();
        Box::new(result)
    }
}
