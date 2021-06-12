use green_moon_2d::menu::GMMenu;
use green_moon_2d::font::{GMBitmapFont, GMFontT};
use green_moon_2d::error::GMError;
use green_moon_2d::sound::GMSound;

use macroquad::prelude::*;

use std::rc::Rc;
use std::thread;
use std::time::Duration;

#[macroquad::main("Menu1")]
async fn main() -> Result<(), GMError> {
    let mut font = GMBitmapFont::new("../assets/gfx/fonts/cuddly.png").await?;
    font.set_mapping_fixed(32.0, 32.0, "ABCDEFGHIJKLMNOPQRSTUVWXYZ.!0123456789?()<>- ");
    let font: Rc<dyn GMFontT> = Rc::new(font);

    let change_sound = GMSound::new_rc("../assets/sfx/change1.ogg").await?;
    let enter_sound = GMSound::new_rc("../assets/sfx/enter1.ogg").await?;

    let items = ["START", "CONTROLS", "GFX OPTIONS", "SFX OPTIONS", "HIGH SCORE", "CREDITS", "EXIT"];
    let mut main_menu = GMMenu::new_simple(240.0, 100.0, "MAIN MENU", &items, &font, &change_sound, &enter_sound);

    loop {
        clear_background(BLACK);

        main_menu.draw();
        main_menu.update();

        if let Some(i) = main_menu.event() {
            println!("Use has selected item: {}", i);

            if i == 6 {
                break;
            }
        }

        next_frame().await
    }

    println!("Good bye!");
    thread::sleep(Duration::from_millis(500));

    Ok(())
}
