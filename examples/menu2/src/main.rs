use green_moon_2d::menu::GMMenu;
use green_moon_2d::menu_item::{GMMenuItemT, GMMenuItemEnum, GMMenuItemStatic};
use green_moon_2d::value::GMValue;
use green_moon_2d::spritesheet::{GMSpriteSheet};
use green_moon_2d::sprite::GMSprite;
use green_moon_2d::text::{GMStaticText, GMSpriteText, GMWaveText};
use green_moon_2d::font::GMBitmapFont;
use green_moon_2d::error::GMError;
use green_moon_2d::sound::GMSound;

use green_moon_2d::animation::GMAnimationPingPong;

use macroquad::prelude::*;

use std::thread;
use std::time::Duration;

#[macroquad::main("Menu2")]
async fn main() -> Result<(), GMError> {
    let font1 = GMBitmapFont::new_rc("../assets/gfx/fonts/cuddly.png", 32.0, 32.0, "ABCDEFGHIJKLMNOPQRSTUVWXYZ.!0123456789?()<>- ").await?;
    let font2 = GMBitmapFont::new_rc("../assets/gfx/fonts/bbc1.png", 32.0, 32.0, "ABCDEFGHIJKLMNOPQRSTUVWXYZ'.,-!?0123456789?<> ").await?;
    let font3 = GMBitmapFont::new_rc("../assets/gfx/fonts/blagger.png", 32.0, 30.0, "ABCDEFGHIJKLMNOPQRSTUVWXYZ.,\"-+!?()': 0123456789").await?;

    let sheet1 = GMSpriteSheet::new_rc("../assets/gfx/sprite_sheets/bat1.png").await?;
    let sheet2 = GMSpriteSheet::new_rc("../assets/gfx/sprite_sheets/cat1.png").await?;
    let sheet3 = GMSpriteSheet::new_rc("../assets/gfx/sprite_sheets/ghost1.png").await?;

    let mut animation1 = GMAnimationPingPong::new_box(&[
        (Rect::new(0.0, 0.0, 16.0, 20.0), 0.100), (Rect::new(16.0, 0.0, 16.0, 20.0), 0.100), (Rect::new(32.0, 0.0, 16.0, 20.0), 0.100)
    ]);
    animation1.start();
    let mut animation2 = GMAnimationPingPong::new_box(&[
        (Rect::new(0.0, 0.0, 28.0, 23.0), 0.100), (Rect::new(28.0, 0.0, 28.0, 23.0), 0.100), (Rect::new(56.0, 0.0, 28.0, 23.0), 0.100)
    ]);
    animation2.start();
    let mut animation3 = GMAnimationPingPong::new_box(&[
        (Rect::new(0.0, 0.0, 40.0, 41.0), 0.100), (Rect::new(40.0, 0.0, 40.0, 41.0), 0.100), (Rect::new(80.0, 0.0, 40.0, 41.0), 0.100)
    ]);
    animation3.start();

    let mut sprite1 = GMSprite::new(&sheet1, animation1, 0.0, 0.0);
    let mut sprite2 = GMSprite::new(&sheet2, animation2, 0.0, 0.0);
    let mut sprite3 = GMSprite::new(&sheet3, animation3, 0.0, 0.0);

    let mut item_y = 100.0;
    let item_x = 100.0;
    let mut menu_items: Vec<Box<dyn GMMenuItemT>> = Vec::new();

    let inactive = GMStaticText::new_box("TITLE FONT ", item_x, item_y, &font1);
    let active = GMSpriteText::new_static("TITLE FONT ", item_x, item_y, &font1, sprite1.clone_sprite());
    let item = GMMenuItemEnum::new_box(inactive, active, "TITLE FONT ", &["CUDDLY", "BBC1", "BLAGGER"], 0);

    menu_items.push(item);

    item_y += 40.0;
    let inactive = GMStaticText::new_box("EXIT", item_x, item_y, &font1);
    let active = GMSpriteText::new_static("EXIT", item_x, item_y, &font1, sprite3.clone_sprite());
    let item = GMMenuItemStatic::new_box(inactive, active);

    menu_items.push(item);

    let change_sound = GMSound::new_rc("../assets/sfx/change1.ogg").await?;
    let enter_sound = GMSound::new_rc("../assets/sfx/enter1.ogg").await?;

    let menu_title = GMWaveText::new_static("MAIN MENU", item_x, 40.0, &font1, 8.0, 10.0);

    let mut main_menu = GMMenu::new(menu_title, menu_items, &change_sound, &enter_sound);

    loop {
        clear_background(BLACK);

        // sprite3.draw();
        // sprite3.update();

        main_menu.draw();
        main_menu.update();

        match main_menu.event() {
            None => {
                // Nothing to do...
            }
            Some((i, v)) => {
                use GMValue::*;
                println!("Use has selected item: {}", i);

                match v {
                    GMNone => {
                        if i == 1 {
                            break;
                        }
                    }
                    GMUSize(j) => {
                        println!("New value: {}", j);
                        match j {
                            0 => {
                                main_menu.set_title_font(&font1);
                            }
                            1 => {
                                main_menu.set_title_font(&font2);
                            }
                            2 => {
                                main_menu.set_title_font(&font3);
                            }
                            _ => {
                                unreachable!("Font not possible");
                            }
                        }
                    }
                    _ => {
                        unreachable!("Should not happen:m unexpected menu value");
                    }
                }
            }
        }

        next_frame().await
    }

    println!("Good bye!");
    thread::sleep(Duration::from_millis(500));

    Ok(())
}
