

use std::{time::Instant, iter::Inspect};


pub trait GMAnimationT {
    fn update(&mut self);
    fn stop(&mut self);
    fn pause(&mut self);
    fn resume(&mut self);
    fn finished(&self) -> bool;
    fn frame_index(&self) -> usize;
}

pub struct GMAnimationForwardOnce {
    paused: bool,
    current_frame: usize,
    frames: Vec<(usize, f32)>,
    instant: Instant,
}

impl GMAnimationForwardOnce {
    pub fn new(frames: Vec<(usize, f32)>) -> Self {
        Self {
            paused: false,
            current_frame: 0,
            frames,
            instant: Instant::now(),
        }
    }
}

impl GMAnimationT for GMAnimationForwardOnce {
    fn update(&mut self) {
        if !self.paused {
            if self.instant.elapsed().as_secs_f32() >= self.frames[self.current_frame].1 {
                if self.current_frame < self.frames.len() - 1 {
                    self.current_frame += 1;
                } else {
                    self.paused = true;
                }
            }
        }
    }

    fn stop(&mut self) {
        self.paused = true;
        self.current_frame = 0;
    }

    fn pause(&mut self) {
        self.paused = true;
    }

    fn resume(&mut self) {
        self.paused = false
    }

    fn finished(&self) -> bool {
        self.current_frame == self.frames.len() - 1
    }

    fn frame_index(&self) -> usize {
        self.frames[self.current_frame].0
    }
}
