use macroquad::math::Rect;
use macroquad::time::get_time;

pub trait GMAnimationT {
    fn start(&mut self);
    fn pause(&mut self);
    fn resume(&mut self);
    fn next_frame(&mut self);
    fn get_rect(&self) -> Rect;
}

/*
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum GMAnimationDirection {
    ForwardOnce,
    ForwardLoop,
    BackwardOnce,
    BackwardLoop,
    PingPongForward,
    PingPongBackward,
}
*/

#[derive(Clone, Debug, PartialEq)]
pub struct GMAnimationBase {
    pub(crate) frames: Vec<(Rect, f64)>,
    pub(crate) current_frame: usize,
    pub(crate) start_time: f64,
    pub(crate) active: bool,
}

impl GMAnimationBase {
    fn start(&mut self) {
        self.current_frame = 0;
        self.active = true;
        self.start_time = get_time();
    }

    fn pause(&mut self) {
        self.active = false;
    }

    fn resume(&mut self) {
        self.active = true;
        self.start_time = get_time();
    }

    fn get_rect(&self) -> Rect {
        self.frames[self.current_frame].0
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct GMAnimationForwardOnce {
    pub(crate) base: GMAnimationBase,
}

impl GMAnimationForwardOnce {
    pub fn new(frames: &[(Rect, f64)]) -> GMAnimationForwardOnce {
        let base = GMAnimationBase {
            frames: frames.to_vec(),
            current_frame: 0,
            start_time: 0.0,
            active: false,

        };

        GMAnimationForwardOnce {
            base
        }
    }

}

impl GMAnimationT for GMAnimationForwardOnce {
    fn start(&mut self) {
        self.base.start()
    }

    fn pause(&mut self) {
        self.base.pause()
    }

    fn resume(&mut self) {
        self.base.resume()
    }

    fn next_frame(&mut self) {
        if !self.base.active {
            return
        }

        if get_time() - self.base.start_time < self.base.frames[self.base.current_frame].1 {
            // Time for current frame has not elapsed yet, so nothing to do.
            // (display the same image as before until the frame time has elapsed)
            return
        }

        if self.base.current_frame < self.base.frames.len() - 1 {
            self.base.current_frame += 1;
        }

/*
        match self.direction {
            ForwardOnce => {
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
*/

        // Set time for the current animation frame
        self.base.start_time = get_time();
    }

    fn get_rect(&self) -> Rect {
        self.base.get_rect()
    }
}

// TODO: Add other animation types
