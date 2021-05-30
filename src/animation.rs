use crate::resource_manager::GMName;

use macroquad::math::Rect;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum GMAnimationDirection {
    ForwardOnce,
    ForwardLoop,
    BackwardOnce,
    BackwardLoop,
    PingPong,
}

#[derive(Clone, Debug, PartialEq)]
pub struct GMAnimation {
    name: String,
    frames: Vec<(Rect, u32)>,
    direction: GMAnimationDirection,
    current_frame: usize,
}

impl GMAnimation {
    pub fn new(name: &str, frames: &[(Rect, u32)]) -> GMAnimation {
        GMAnimation {
            name: name.to_string(),
            frames: frames.to_vec(),
            direction: GMAnimationDirection::ForwardOnce,
            current_frame: 0,
        }
    }

    pub fn set_direction(&mut self, direction: GMAnimationDirection) {
        self.direction = direction;
    }

    pub fn reset(&mut self) {
        self.current_frame = 0;
    }

    pub fn next_frame(&mut self) {
        use GMAnimationDirection::*;

        // TODO: Check time!

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
                // TODO:
            }
            BackwardLoop => {
                // TODO:
            }
            PingPong => {
                // TODO:
            }
        }
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
