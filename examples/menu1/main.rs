
use green_moon_2d::prelude::*;

mod main_screen;

use main_screen::{MainScreen};


fn main() {
    let settings = GM_Settings::load("assets/settings.toml");
    let screens: Vec<Box<dyn GM_Screen_T>> = vec![Box::new(MainScreen::new(&settings))];

    let mut menu_example = GreenMoon2D::new(settings, screens);
    menu_example.run();
}
