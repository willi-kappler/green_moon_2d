use green_moon_2d::menu::{GMMenuItemStatic, GMMenu, GMMenuItemT};
use green_moon_2d::text::{GMStaticText, GMArrowText};
use green_moon_2d::font::{GMBitmapFont, GMFontT};
use green_moon_2d::error::GMError;

use macroquad::prelude::*;

use std::rc::Rc;

fn make_menu(x: f32, y: f32, title: &str, items: &[&str], font: &Rc<dyn GMFontT>) -> GMMenu {
    let mut current_y = y;

    let title = GMStaticText::new_box(title, x, y, &font);
    let mut menu_items = Vec::new();
    let (_, font_height) = font.get_extend('A');
    current_y += font_height * 2.0;

    for item in items.iter() {
        let inactive = GMStaticText::new_box(item, x, current_y, &font);
        let active = GMStaticText::new_box(item, x, current_y, &font);
        let active = GMArrowText::new_box(active);
        let menu_item = GMMenuItemStatic::new_box(inactive, active);

        menu_items.push(menu_item);

        current_y += font_height + 4.0;
    }

    GMMenu::new(title, menu_items)
}

#[macroquad::main("Menu1")]
async fn main() -> Result<(), GMError> {
    let font = GMBitmapFont::new("../assets/gfx/fonts/cuddly.png", 32.0, 32.0, "ABCDEFGHIJKLMNOPQRSTUVWXYZ.!0123456789?()<>- ").await?;
    let font: Rc<dyn GMFontT> = Rc::new(font);

    let items = ["START", "CONTROLS", "GFX OPTIONS", "SFX OPTIONS", "HIGH SCORE", "CREDITS", "EXIT"];
    let mut main_menu = make_menu(200.0, 100.0, "MAIN MENU", &items, &font);

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

    Ok(())
}
