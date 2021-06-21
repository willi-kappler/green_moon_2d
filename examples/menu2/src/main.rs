use green_moon_2d::menu::GMMenu;
use green_moon_2d::menu_item::{GMMenuItemStatic, GMMenuItemEnum, GMMenuItemNumeric};
use green_moon_2d::value::GMValue;
use green_moon_2d::sprite::GMSprite;
use green_moon_2d::text::GMWaveText;
use green_moon_2d::error::GMError;
use green_moon_2d::resource_manager::GMResourceManager;

use macroquad::prelude::*;

use log4rs;

use std::thread;
use std::time::Duration;
use std::rc::Rc;
use std::any::Any;

#[macroquad::main("Menu2")]
async fn main() -> Result<(), GMError> {
    log4rs::init_file("log4rs.yaml", Default::default()).unwrap();

    let resources = GMResourceManager::new_from_file("resources.json").await?;

    let fonts = [
        &resources.get_font("cuddly").unwrap(),
        &resources.get_font("bbc1").unwrap(),
        &resources.get_font("blagger").unwrap(),
    ];

    /*

    let animation1 = GMAnimationPingPong::new_box(&[
        (Rect::new(0.0, 0.0, 16.0, 20.0), 0.100), (Rect::new(16.0, 0.0, 16.0, 20.0), 0.100), (Rect::new(32.0, 0.0, 16.0, 20.0), 0.100)
    ]);
    let animation2 = GMAnimationPingPong::new_box(&[
        (Rect::new(0.0, 0.0, 28.0, 23.0), 0.100), (Rect::new(28.0, 0.0, 28.0, 23.0), 0.100), (Rect::new(56.0, 0.0, 28.0, 23.0), 0.100)
    ]);
    let animation3 = GMAnimationPingPong::new_box(&[
        (Rect::new(0.0, 0.0, 40.0, 41.0), 0.100), (Rect::new(40.0, 0.0, 40.0, 41.0), 0.100), (Rect::new(80.0, 0.0, 40.0, 41.0), 0.100)
    ]);
    */

    let sprite1 = GMSprite::new(
        &resources.get_sprite_sheet("bat").unwrap(),
        resources.get_animation("bat").unwrap(), 0.0, 0.0);
    let sprite2 = GMSprite::new(
        &resources.get_sprite_sheet("cat").unwrap(),
        resources.get_animation("cat").unwrap(), 0.0, 0.0);
    let sprite3 = GMSprite::new(
        &resources.get_sprite_sheet("ghost").unwrap(),
        resources.get_animation("ghost").unwrap(), 0.0, 0.0);

    let sprites = [&sprite1, &sprite2, &sprite3];

    let menu_title = GMWaveText::new_static("MAIN MENU", 100.0, 40.0, &fonts[0], 8.0, 10.0);

    let mut main_menu = GMMenu::new_empty(menu_title,
        &resources.get_sound("change").unwrap(),
        &resources.get_sound("enter").unwrap()
    );

    main_menu.add_item(GMMenuItemEnum::new_static_sprite("TITLE FONT ", 100.0, 100.0,
        &fonts[0], sprites[0].clone_sprite(), &["CUDDLY", "BBC1", "BLAGGER"], 0), 0.0, 0.0);

    main_menu.add_item(GMMenuItemEnum::new_static_sprite("ITEM FONT ", 0.0, 0.0,
        &fonts[0], sprites[0].clone_sprite(), &["CUDDLY", "BBC1", "BLAGGER"], 0), 0.0, 40.0);

    main_menu.add_item(GMMenuItemEnum::new_static_sprite("SPRITE ", 0.0, 0.0,
        &fonts[0], sprites[0].clone_sprite(), &["BAT", "CAT", "GHOST"], 0), 0.0, 40.0);

    main_menu.add_item(GMMenuItemNumeric::new_static_sprite("AMPLITUDE ", 0.0, 0.0,
        &fonts[0], sprites[0].clone_sprite(), 1.0, 20.0, 8.0, 1.0), 0.0, 40.0);

    main_menu.add_item(GMMenuItemNumeric::new_static_sprite("FREQUENCY ", 0.0, 0.0,
        &fonts[0], sprites[0].clone_sprite(), 1.0, 30.0, 10.0, 0.5), 0.0, 40.0);

    main_menu.add_item(GMMenuItemNumeric::new_static_sprite("OFFSET ", 0.0, 0.0,
        &fonts[0], sprites[0].clone_sprite(), 1.0, 3.1, 1.0, 0.1), 0.0, 40.0);

    main_menu.add_item(GMMenuItemStatic::new_static_sprite("EXIT", 0.0, 0.0,
        &fonts[0], sprites[0].clone_sprite()), 0.0, 40.0);

    loop {
        clear_background(BLACK);

        main_menu.draw();
        main_menu.update();

        match main_menu.event() {
            None => {
                // Nothing to do...
            }
            Some((i, v)) => {
                use GMValue::*;
                println!("User has selected item: {}", i);

                match v {
                    GMNone => {
                        if i == 6 {
                            break;
                        }
                    }
                    GMUSize(j) => {
                        println!("New usize value: {}", j);

                        if i == 0 {
                            main_menu.set_title_font(fonts[j]);
                        } else if i == 1 {
                            main_menu.set_item_font(fonts[j]);
                        } else if i == 2 {
                            let sprite: Rc<dyn Any> = Rc::new(sprites[j].clone_sprite());
                            main_menu.change_property_all("sprite", &sprite);
                        }
                    }
                    GMF32(f) => {
                        println!("New f32 value: {}", f);
                        let value: Rc<dyn Any> = Rc::new(f);

                        if i == 3 {
                            main_menu.change_property_title("amplitude", &value);
                        } else if i == 4 {
                            main_menu.change_property_title("frequency", &value);
                        } else if i == 5 {
                            main_menu.change_property_title("offset", &value);
                        }
                    }
                    _ => {
                        unreachable!("Should not happen: unexpected menu value");
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
