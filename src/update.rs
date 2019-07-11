

// Local modules
use crate::resources::{GM_Resources};

pub trait GM_UpdateResource_T {
    fn update(&mut self, resources: &mut GM_Resources) {
    }
}

pub trait GM_UpdateElapsed_T {
    fn update(&mut self, elapsed: u16) {
    }
}
