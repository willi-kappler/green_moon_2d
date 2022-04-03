

use std::{time::Instant};


pub trait GMAnimationT {
    fn set_frame_index(&mut self, index: usize);
    fn set_active(&mut self, active: bool);
    fn update(&mut self);
    fn finished(&self) -> bool;
    fn frame_index(&self) -> u32;
    fn set_frames(&mut self, frames: &[(u32, f32)]);
    fn frames_mut(&mut self) -> &mut [(u32, f32)];
    fn box_clone(&self) -> Box<dyn GMAnimationT>;
}

pub struct GMAnimationStatic {
    frames: Vec<(u32, f32)>,
}

impl GMAnimationStatic {
    pub fn new() -> Self {
        Self {
            frames: Vec::new(),
        }
    }
}

impl GMAnimationT for GMAnimationStatic {
    fn set_frame_index(&mut self, index: usize) {}

    fn set_active(&mut self, active: bool) {}

    fn update(&mut self) {}

    fn finished(&self) -> bool {
        true
    }

    fn frame_index(&self) -> u32 {
        0
    }

    fn set_frames(&mut self, frames: &[(u32, f32)]) {}

    fn frames_mut(&mut self) -> &mut [(u32, f32)] {
        &mut self.frames
    }

    fn box_clone(&self) -> Box<dyn GMAnimationT> {
        let result = GMAnimationStatic::new();
        Box::new(result)
    }
}

pub struct GMAnimationForwardOnce {
    active: bool,
    current_frame: usize,
    frames: Vec<(u32, f32)>,
    instant: Instant,
}

impl GMAnimationForwardOnce {
    pub fn new(frames: &[(u32, f32)]) -> Self {
        Self {
            active: true,
            current_frame: 0,
            frames: frames.to_vec(),
            instant: Instant::now(),
        }
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

    fn frame_index(&self) -> u32 {
        self.frames[self.current_frame].0
    }

    fn set_frames(&mut self, frames: &[(u32, f32)]) {
        self.frames = frames.to_vec();
    }

    fn frames_mut(&mut self) -> &mut [(u32, f32)] {
        &mut self.frames
    }

    fn box_clone(&self) -> Box<dyn GMAnimationT> {
        let result = GMAnimationForwardOnce::new(&self.frames.clone());
        Box::new(result)
    }
}

pub struct GMAnimationForwardLoop {
    active: bool,
    current_frame: usize,
    frames: Vec<(u32, f32)>,
    instant: Instant,
}

impl GMAnimationForwardLoop {
    pub fn new(frames: &[(u32, f32)]) -> Self {
        Self {
            active: true,
            current_frame: 0,
            frames: frames.to_vec(),
            instant: Instant::now(),
        }
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

    fn frame_index(&self) -> u32 {
        self.frames[self.current_frame].0
    }

    fn set_frames(&mut self, frames: &[(u32, f32)]) {
        self.frames = frames.to_vec();
    }

    fn frames_mut(&mut self) -> &mut [(u32, f32)] {
        &mut self.frames
    }

    fn box_clone(&self) -> Box<dyn GMAnimationT> {
        let result = GMAnimationForwardLoop::new(&self.frames.clone());
        Box::new(result)
    }
}

pub struct GMAnimationPingPong {
    active: bool,
    current_frame: usize,
    frames: Vec<(u32, f32)>,
    instant: Instant,
    foreward: bool,
}

impl GMAnimationPingPong {
    pub fn new(frames: &[(u32, f32)]) -> Self {
        Self {
            active: true,
            current_frame: 0,
            frames: frames.to_vec(),
            instant: Instant::now(),
            foreward: true,
        }
    }
}

impl GMAnimationT for GMAnimationPingPong {
    fn set_frame_index(&mut self, index: usize) {
        self.current_frame = index;
    }

    fn set_active(&mut self, active: bool) {
        self.active = active;
    }

    fn update(&mut self) {
        if self.active {
            if self.instant.elapsed().as_secs_f32() >= self.frames[self.current_frame].1 {
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
            }
        }
    }

    fn finished(&self) -> bool {
        false
    }

    fn frame_index(&self) -> u32 {
        self.frames[self.current_frame].0
    }

    fn set_frames(&mut self, frames: &[(u32, f32)]) {
        self.frames = frames.to_vec();
    }

    fn frames_mut(&mut self) -> &mut [(u32, f32)] {
        &mut self.frames
    }

    fn box_clone(&self) -> Box<dyn GMAnimationT> {
        let result = GMAnimationPingPong::new(&self.frames.clone());
        Box::new(result)
    }
}
