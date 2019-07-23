
#![allow(non_camel_case_types)]

pub mod gfx;
pub mod misc;
pub mod sfx;
pub mod ui;
pub mod util;

pub mod prelude {
    pub use crate::misc::gm_2d::{GreenMoon2D};
    pub use crate::misc::settings::{GM_Settings};

}
