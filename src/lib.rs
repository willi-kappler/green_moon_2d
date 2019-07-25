
#![allow(non_camel_case_types)]

pub mod gfx;
pub mod misc;
pub mod sfx;
pub mod ui;
pub mod util;

pub mod prelude {
    pub use crate::misc::{GreenMoon2D};
    pub use crate::misc::{GM_Settings};
    pub use crate::misc::{GM_Runtime};
    pub use crate::misc::{GM_Position};
    pub use crate::misc::{GM_Event};
    pub use crate::misc::{GM_Dimension};

    pub use crate::gfx::{GM_Screen_T, GM_ScreenState};
    pub use crate::gfx::{GM_Canvas};
    pub use crate::gfx::{GM_BitmapFont};

    pub use crate::ui::menu::{GM_Menu};
    pub use crate::ui::text::{GM_StaticText, GM_WaveText, GM_SelectableText, GM_SelectableText_T};
}
