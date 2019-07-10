

// Local modules
use crate::resources::{GM_Resources};

pub trait GM_Draw_T {
    fn draw(&self, resources: &mut GM_Resources) {
    }
}
