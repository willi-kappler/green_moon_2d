
use std::fmt::Debug;

use crate::animation::GMAnimationBase;
use crate::effect::GMEffectT;


#[derive(Clone, Debug)]
pub struct GMAESimple {
    pub active: bool,
}

impl GMEffectT<GMAnimationBase> for GMAESimple {
    fn set_active(&mut self, active: bool) {
        self.active = active;
    }

    fn clone_box(&self) -> Box<dyn GMEffectT<GMAnimationBase>> {
        Box::new(self.clone())
    }
}
