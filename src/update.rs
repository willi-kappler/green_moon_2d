

// Local modules
use crate::resources::{GM_Resources};

pub trait GM_Update_Resource_T {
    fn update(&mut self, resources: &mut GM_Resources) {
    }
}

pub trait GM_Update_Elapsed_T {
    fn update(&mut self, elapsed: u16) {
    }
}
