
use std::fmt::Debug;

use crate::context::GMContext;
use crate::animation::GMAnimationBase;

pub trait GMAnimationEffectT: Debug {
    fn update(&mut self, _animation: &mut GMAnimationBase, _context: &mut GMContext);

    fn send_message(&mut self, _message: &str, _context: &mut GMContext) {
    }

    fn set_active(&mut self, active: bool);

    fn clone_box(&self) -> Box<dyn GMAnimationEffectT>;
}

impl Clone for Box<dyn GMAnimationEffectT> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}
