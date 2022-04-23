

use std::fmt::{self, Debug, Formatter};

use crate::timer::GMTimer;

pub trait GMAnimationT {
    fn set_active(&mut self, active: bool);
    fn update(&mut self);
    fn finished(&self) -> bool;
    fn frame_index(&self) -> u32;
    fn box_clone(&self) -> Box<dyn GMAnimationT>;
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
pub struct GMAnimationStatic {
    pub frames: Vec<(u32, f32)>,
}

impl GMAnimationStatic {
    pub fn new() -> Self {
        Self {
            frames: Vec::new(),
        }
    }
}

impl GMAnimationT for GMAnimationStatic {
    fn set_active(&mut self, _active: bool) {}

    fn update(&mut self) {}

    fn finished(&self) -> bool {
        true
    }

    fn frame_index(&self) -> u32 {
        0
    }

    fn box_clone(&self) -> Box<dyn GMAnimationT> {
        let result = GMAnimationStatic::new();
        Box::new(result)
    }
}

#[derive(Clone, Debug)]
pub struct GMAnimationForwardOnce {
    pub active: bool,
    pub current_frame: usize,
    pub frames: Vec<(u32, f32)>, // index, duration
    timer: GMTimer,
}

impl GMAnimationForwardOnce {
    pub fn new(frames: &[(u32, f32)]) -> Self {
        Self {
            active: true,
            current_frame: 0,
            frames: frames.to_vec(),
            timer: GMTimer::new(frames[0].1),
        }
    }
}

impl GMAnimationT for GMAnimationForwardOnce {
    fn set_active(&mut self, active: bool) {
        self.active = active;
    }

    fn update(&mut self) {
        if self.active {
            if self.timer.finished() {
                if self.current_frame < self.frames.len() - 1 {
                    self.current_frame += 1;
                    self.timer.set_duration(self.frames[self.current_frame].1);
                } else {
                    self.active = false;
                }
            }
        }
    }

    fn finished(&self) -> bool {
        self.current_frame == self.frames.len() - 1
    }

    fn frame_index(&self) -> u32 {
        self.frames[self.current_frame].0
    }

    fn box_clone(&self) -> Box<dyn GMAnimationT> {
        let result = GMAnimationForwardOnce::new(&self.frames.clone());
        Box::new(result)
    }
}

#[derive(Clone, Debug)]
pub struct GMAnimationForwardLoop {
    pub active: bool,
    pub current_frame: usize,
    pub frames: Vec<(u32, f32)>,
    timer: GMTimer,
}

impl GMAnimationForwardLoop {
    pub fn new(frames: &[(u32, f32)]) -> Self {
        Self {
            active: true,
            current_frame: 0,
            frames: frames.to_vec(),
            timer: GMTimer::new(frames[0].1),
        }
    }
}

impl GMAnimationT for GMAnimationForwardLoop {
    fn set_active(&mut self, active: bool) {
        self.active = active;
    }

    fn update(&mut self) {
        if self.active {
            if self.timer.finished() {
                self.current_frame += 1;
                if self.current_frame >= self.frames.len() {
                    self.current_frame = 0;
                }
                self.timer.set_duration(self.frames[self.current_frame].1)
            }
        }
    }

    fn finished(&self) -> bool {
        false
    }

    fn frame_index(&self) -> u32 {
        self.frames[self.current_frame].0
    }

    fn box_clone(&self) -> Box<dyn GMAnimationT> {
        let result = GMAnimationForwardLoop::new(&self.frames.clone());
        Box::new(result)
    }
}

#[derive(Clone, Debug)]
pub struct GMAnimationPingPong {
    pub active: bool,
    pub current_frame: usize,
    pub frames: Vec<(u32, f32)>,
    timer: GMTimer,
    foreward: bool,
}

impl GMAnimationPingPong {
    pub fn new(frames: &[(u32, f32)]) -> Self {
        Self {
            active: true,
            current_frame: 0,
            frames: frames.to_vec(),
            timer: GMTimer::new(frames[0].1),
            foreward: true,
        }
    }
}

impl GMAnimationT for GMAnimationPingPong {
    fn set_active(&mut self, active: bool) {
        self.active = active;
    }

    fn update(&mut self) {
        if self.active {
            if self.timer.finished() {
                if self.foreward {
                    self.current_frame += 1;
                    if self.current_frame >= self.frames.len() {
                        self.current_frame = self.frames.len() - 2;
                        self.foreward = false;
                    }
                } else {
                    if self.current_frame > 0 {
                        self.current_frame -= 1;
                    } else {
                        self.foreward = true;
                    }
                }
                self.timer.set_duration(self.frames[self.current_frame].1)
            }
        }
    }

    fn finished(&self) -> bool {
        false
    }

    fn frame_index(&self) -> u32 {
        self.frames[self.current_frame].0
    }

    fn box_clone(&self) -> Box<dyn GMAnimationT> {
        let result = GMAnimationPingPong::new(&self.frames.clone());
        Box::new(result)
    }
}
