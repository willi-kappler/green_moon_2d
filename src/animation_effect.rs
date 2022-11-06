
use std::fmt::Debug;

use crate::animation::GMAnimationBase;
use crate::effect::GMEffectT;
use crate::data::GMData;
use crate::util::error_panic;

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

    fn get_property(&self, name: &str) -> GMData {
        match name {
            "" => {
                todo!()
            }
            _ => {
                error_panic(&format!("GMAESimple::get_property(), unknown property: '{}'", name))
            }
        }
    }
}
