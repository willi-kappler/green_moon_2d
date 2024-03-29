use macroquad::math::Rect;
use macroquad::time::get_time;


// TODO: Add other animation types

pub trait GMAnimationT {
    fn start(&mut self);
    fn pause(&mut self);
    fn resume(&mut self);
    fn next_frame(&mut self);
    fn get_rect(&self) -> Rect;
    fn finished(&self) -> bool;
    // impl Clone is not possible because of object safety:
    // clone() returns Self
    fn clone_animation(&self) -> GMAnimation;
}

pub struct GMAnimation {
    animation: Box<dyn GMAnimationT>,
}

impl GMAnimation {
    pub fn new<T: 'static + GMAnimationT>(animation: T) -> Self {
        Self {
            animation: Box::new(animation),
        }
    }
    pub fn set_animation(&mut self, animation: Box<dyn GMAnimationT>) {
        self.animation = animation;
    }
    pub fn start(&mut self) {
        self.animation.start();
    }
    pub fn pause(&mut self){
        self.animation.pause();
    }
    pub fn resume(&mut self){
        self.animation.resume();
    }
    pub fn next_frame(&mut self){
        self.animation.next_frame();
    }
    pub fn get_rect(&self) -> Rect {
        self.animation.get_rect()
    }
    pub fn finished(&self) -> bool {
        self.animation.finished()
    }
}

impl Clone for GMAnimation {
    fn clone(&self) -> Self {
        self.animation.clone_animation()
    }
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
        let duration = self.frames[self.current_frame].1;

        if duration == 0.0 {
            return true
        }

        if !self.active {
            return true
        }

        if self.frames.len() == 1 {
            return true
        }

        if get_time() - self.start_time < duration {
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
    pub fn new_anim(frames: &[(Rect, f64)]) -> GMAnimation {
        let animation = Self::new(frames);
        GMAnimation::new(animation)
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
    fn clone_animation(&self) -> GMAnimation {
        GMAnimation::new(self.clone())
    }
}


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
    pub fn new_anim(frames: &[(Rect, f64)]) -> GMAnimation {
        let animation = Self::new(frames);
        GMAnimation::new(animation)
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
    fn clone_animation(&self) -> GMAnimation {
        GMAnimation::new(self.clone())
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
    pub fn new_anim(frames: &[(Rect, f64)]) -> GMAnimation {
        let animation = Self::new(frames);
        GMAnimation::new(animation)
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
    fn clone_animation(&self) -> GMAnimation {
        GMAnimation::new(self.clone())
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
    pub fn new_anim(frames: &[(Rect, f64)]) -> GMAnimation {
        let animation = Self::new(frames);
        GMAnimation::new(animation)
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
    fn clone_animation(&self) -> GMAnimation {
        GMAnimation::new(self.clone())
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
    pub fn new_anim(frames: &[(Rect, f64)]) -> GMAnimation {
        let animation = Self::new(frames);
        GMAnimation::new(animation)
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
    fn clone_animation(&self) -> GMAnimation {
        GMAnimation::new(self.clone())
   }
}
