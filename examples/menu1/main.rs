
use green_moon_2d::prelude::*;

mod main_screen;

use main_screen::{MainScreen};


fn main() {
    let settings = GM_Settings::new();
    let mut menu_example = GreenMoon2D::new();

    menu_example.set_settings(settings);

    menu_example.add_screen(MainScreen::new());

    menu_example.run();
}
