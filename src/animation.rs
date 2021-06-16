use macroquad::math::Rect;
use macroquad::time::get_time;

pub trait GMAnimationT {
    fn start(&mut self);
    fn pause(&mut self);
    fn resume(&mut self);
    fn next_frame(&mut self);
    fn get_rect(&self) -> Rect;
    fn finished(&self) -> bool;
    // Clone is not possible because of object safety
    fn clone_animation(&self) -> Box<dyn GMAnimationT>;
}

#[derive(Clone, Debug, PartialEq)]
pub struct GMAnimationBase {
    frames: Vec<(Rect, f64)>,
    current_frame: usize,
    start_time: f64,
    active: bool,
}

impl GMAnimationBase {
    fn new(frames: &[(Rect, f64)]) -> Self {
        Self {
            frames: frames.to_vec(),
            current_frame: 0,
            start_time: 0.0,
            active: false,
        }
    }
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
    fn check_frame(&mut self) -> bool {
        if !self.active {
            return true
        }

        if get_time() - self.start_time < self.frames[self.current_frame].1 {
            // Time for current frame has not elapsed yet, so nothing to do.
            // (display the same image as before until the frame time has elapsed)
            return true
        }

        // Set time for the current animation frame
        self.start_time = get_time();

        false
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct GMAnimationForwardOnce {
    pub(crate) base: GMAnimationBase,
}

impl GMAnimationForwardOnce {
    pub fn new(frames: &[(Rect, f64)]) -> Self {
        Self {
            base: GMAnimationBase::new(frames)
        }
    }
    pub fn new_box(frames: &[(Rect, f64)]) -> Box<dyn GMAnimationT> {
        let animation = Self::new(frames);
        Box::new(animation)
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
        if self.base.check_frame() {
            return
        }

        if self.base.current_frame < self.base.frames.len() - 1 {
            self.base.current_frame += 1;
        }
    }

    fn get_rect(&self) -> Rect {
        self.base.get_rect()
    }
    fn finished(&self) -> bool {
        self.base.current_frame == self.base.frames.len() - 1
    }
    fn clone_animation(&self) -> Box<(dyn GMAnimationT)> {
         Box::new(self.clone())
    }
}

// TODO: Add other animation types

#[derive(Clone, Debug, PartialEq)]
pub struct GMAnimationForwardLoop {
    pub(crate) base: GMAnimationBase,
}

impl GMAnimationForwardLoop {
    pub fn new(frames: &[(Rect, f64)]) -> Self {
        Self {
            base: GMAnimationBase::new(frames)
        }
    }
    pub fn new_box(frames: &[(Rect, f64)]) -> Box<dyn GMAnimationT> {
        let animation = Self::new(frames);
        Box::new(animation)
    }
}

impl GMAnimationT for GMAnimationForwardLoop {
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
        if self.base.check_frame() {
            return
        }

        if self.base.current_frame < self.base.frames.len() - 1 {
            self.base.current_frame += 1;
        } else {
            self.base.current_frame = 0;
        }
    }
    fn get_rect(&self) -> Rect {
        self.base.get_rect()
    }
    fn finished(&self) -> bool {
        false
    }
    fn clone_animation(&self) -> Box<(dyn GMAnimationT)> {
        Box::new(self.clone())
   }
}

#[derive(Clone, Debug, PartialEq)]
pub struct GMAnimationBackwardOnce {
    pub(crate) base: GMAnimationBase,
}

impl GMAnimationBackwardOnce {
    pub fn new(frames: &[(Rect, f64)]) -> Self {
        Self {
            base: GMAnimationBase::new(frames)
        }
    }
    pub fn new_box(frames: &[(Rect, f64)]) -> Box<dyn GMAnimationT> {
        let animation = Self::new(frames);
        Box::new(animation)
    }
}

impl GMAnimationT for GMAnimationBackwardOnce {
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
        if self.base.check_frame() {
            return
        }

        if self.base.current_frame > 0 {
            self.base.current_frame -= 1;
        }
    }
    fn get_rect(&self) -> Rect {
        self.base.get_rect()
    }
    fn finished(&self) -> bool {
        self.base.current_frame == 0
    }
    fn clone_animation(&self) -> Box<(dyn GMAnimationT)> {
        Box::new(self.clone())
   }
}

#[derive(Clone, Debug, PartialEq)]
pub struct GMAnimationBackwardLoop {
    pub(crate) base: GMAnimationBase,
}

impl GMAnimationBackwardLoop {
    pub fn new(frames: &[(Rect, f64)]) -> Self {
        Self {
            base: GMAnimationBase::new(frames)
        }
    }
    pub fn new_box(frames: &[(Rect, f64)]) -> Box<dyn GMAnimationT> {
        let animation = Self::new(frames);
        Box::new(animation)
    }
}

impl GMAnimationT for GMAnimationBackwardLoop {
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
        if self.base.check_frame() {
            return
        }

        if self.base.current_frame > 0 {
            self.base.current_frame -= 1;
        } else {
            self.base.current_frame = self.base.frames.len() - 1;
        }
    }
    fn get_rect(&self) -> Rect {
        self.base.get_rect()
    }
    fn finished(&self) -> bool {
        false
    }
    fn clone_animation(&self) -> Box<(dyn GMAnimationT)> {
        Box::new(self.clone())
   }
}

#[derive(Clone, Debug, PartialEq)]
pub struct GMAnimationPingPong {
    pub(crate) base: GMAnimationBase,
    pub(crate) forward: bool,
}

impl GMAnimationPingPong {
    pub fn new(frames: &[(Rect, f64)]) -> Self {
        Self {
            base: GMAnimationBase::new(frames),
            forward: true,
        }
    }
    pub fn new_box(frames: &[(Rect, f64)]) -> Box<dyn GMAnimationT> {
        let animation = Self::new(frames);
        Box::new(animation)
    }
}

impl GMAnimationT for GMAnimationPingPong {
    fn start(&mut self) {
        self.base.start();
        self.forward = true;
    }
    fn pause(&mut self) {
        self.base.pause()
    }
    fn resume(&mut self) {
        self.base.resume()
    }
    fn next_frame(&mut self) {
        if self.base.check_frame() {
            return
        }

        if self.forward {
            if self.base.current_frame < self.base.frames.len() - 1 {
                self.base.current_frame += 1;
            } else {
                self.forward = false;
            }
        } else {
            if self.base.current_frame > 0 {
                self.base.current_frame -= 1;
            } else {
                self.forward = true;
            }
        }
    }
    fn get_rect(&self) -> Rect {
        self.base.get_rect()
    }
    fn finished(&self) -> bool {
        false
    }
    fn clone_animation(&self) -> Box<(dyn GMAnimationT)> {
        Box::new(self.clone())
   }
}
