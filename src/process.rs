

// Local modules
use crate::event::{GM_Event};
use crate::resources::{GM_Resources};

pub trait GM_Process_T {
    fn process(&mut self, event: &GM_Event, resources: &mut GM_Resources) {

    }
}

