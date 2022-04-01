

use std::{time::Instant};


pub trait GMAnimationT {
    fn set_frame_index(&mut self, index: usize);
    fn set_active(&mut self, active: bool);
    fn update(&mut self);
    fn finished(&self) -> bool;
    fn frame_index(&self) -> usize;
}

pub struct GMAnimationForwardOnce {
    active: bool,
    current_frame: usize,
    frames: Vec<(usize, f32)>,
    instant: Instant,
}

impl GMAnimationForwardOnce {
    pub fn new(frames: &[(usize, f32)]) -> Self {
        Self {
            active: false,
            current_frame: 0,
            frames: frames.to_vec(),
            instant: Instant::now(),
        }
    }

    pub fn set_frames(&mut self, frames: &[(usize, f32)]) {
        self.frames = frames.to_vec();
    }
}

impl GMAnimationT for GMAnimationForwardOnce {
    fn set_frame_index(&mut self, index: usize) {
        self.current_frame = index;
    }

    fn set_active(&mut self, active: bool) {
        self.active = active;
    }

    fn update(&mut self) {
        if self.active {
            if self.instant.elapsed().as_secs_f32() >= self.frames[self.current_frame].1 {
                if self.current_frame < self.frames.len() - 1 {
                    self.current_frame += 1;
                } else {
                    self.active = false;
                }
            }
        }
    }

    fn finished(&self) -> bool {
        self.current_frame == self.frames.len() - 1
    }

    fn frame_index(&self) -> usize {
        self.frames[self.current_frame].0
    }
}

pub struct GMAnimationForwardLoop {
    active: bool,
    current_frame: usize,
    frames: Vec<(usize, f32)>,
    instant: Instant,
}

impl GMAnimationForwardLoop {
    pub fn new(frames: Vec<(usize, f32)>) -> Self {
        Self {
            active: false,
            current_frame: 0,
            frames,
            instant: Instant::now(),
        }
    }

    pub fn set_frames(&mut self, frames: &[(usize, f32)]) {
        self.frames = frames.to_vec();
    }
}

impl GMAnimationT for GMAnimationForwardLoop {
    fn set_frame_index(&mut self, index: usize) {
        self.current_frame = index;
    }

    fn set_active(&mut self, active: bool) {
        self.active = active;
    }

    fn update(&mut self) {
        if self.active {
            if self.instant.elapsed().as_secs_f32() >= self.frames[self.current_frame].1 {
                self.current_frame += 1;
                if self.current_frame >= self.frames.len() {
                    self.current_frame = 0;
                }
            }
        }
    }

    fn finished(&self) -> bool {
        false
    }

    fn frame_index(&self) -> usize {
        self.frames[self.current_frame].0
    }
}
