use crate::resource_manager::GMName;

use macroquad::math::Rect;
use macroquad::time::get_time;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum GMAnimationDirection {
    ForwardOnce,
    ForwardLoop,
    BackwardOnce,
    BackwardLoop,
    PingPongForward,
    PingPongBackward,
}

#[derive(Clone, Debug, PartialEq)]
pub struct GMAnimation {
    name: String,
    frames: Vec<(Rect, f64)>,
    direction: GMAnimationDirection,
    current_frame: usize,
    start_time: f64,
    active: bool,
}

impl GMAnimation {
    pub fn new(name: &str, frames: &[(Rect, f64)]) -> GMAnimation {
        GMAnimation {
            name: name.to_string(),
            frames: frames.to_vec(),
            direction: GMAnimationDirection::ForwardOnce,
            current_frame: 0,
            start_time: 0.0,
            active: false,
        }
    }

    pub fn set_direction(&mut self, direction: GMAnimationDirection) {
        self.direction = direction;
    }

    pub fn start(&mut self) {
        self.current_frame = 0;
        self.active = true;
        self.start_time = get_time();
    }

    pub fn pause(&mut self) {
        self.active = false;
    }

    pub fn resume(&mut self) {
        self.active = true;
        self.start_time = get_time();
    }

    pub fn next_frame(&mut self) {
        use GMAnimationDirection::*;

        if get_time() - self.start_time < self.frames[self.current_frame].1 {
            // Time for current frame has not elapsed yet, so nothing to do.
            // (display the same image as before until the frame time has elapsed)
            return
        }

        match self.direction {
            ForwardOnce => {
                if self.current_frame < self.frames.len() - 1 {
                    self.current_frame += 1;
                }
            }
            ForwardLoop => {
                if self.current_frame < self.frames.len() - 1 {
                    self.current_frame += 1;
                } else {
                    self.current_frame = 0;
                }
            }
            BackwardOnce => {
                if self.current_frame > 0 {
                    self.current_frame -= 1;
                }
            }
            BackwardLoop => {
                if self.current_frame > 0 {
                    self.current_frame -= 1;
                } else {
                    self.current_frame = self.frames.len() - 1;
                }
            }
            PingPongForward => {
                if self.current_frame < self.frames.len() - 1 {
                    self.current_frame += 1;
                } else {
                    self.direction = PingPongBackward;
                }
            }
            PingPongBackward => {
                if self.current_frame > 0 {
                    self.current_frame -= 1;
                } else {
                    self.direction = PingPongForward;
                }
            }
        }

        // Set time for the current animation frame
        self.start_time = get_time();
    }

    pub fn get_rect(&self) -> Rect {
        self.frames[self.current_frame].0
    }
}

impl GMName for GMAnimation {
    fn get_name(&self) -> &str {
        &self.name
    }

    fn set_name(&mut self, name: &str) {
        self.name = name.to_string();
    }

    fn has_name(&self, name: &str) -> bool {
        self.name == name
    }

    fn has_prefix(&self, name: &str) -> bool {
        self.name.starts_with(name)
    }
}
