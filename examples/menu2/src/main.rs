use green_moon_2d::menu::GMMenu;
use green_moon_2d::menuitem::{GMMenuItemStatic, GMMenuItemEnum, GMMenuItemNumeric};
use green_moon_2d::utils::{GMValue, GMKeyValue};
use green_moon_2d::text::GMTextWave;
use green_moon_2d::error::GMError;
use green_moon_2d::resources::GMResourceManager;

use macroquad::prelude::*;

use log4rs;

use std::thread;
use std::time::Duration;
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

    let sprites = [
        resources.get_sprite("bat1").unwrap(),
        resources.get_sprite("cat1").unwrap(),
        resources.get_sprite("ghost1").unwrap(),
    ];

    let menu_title = GMTextWave::new_static("MAIN MENU", 100.0, 40.0, &fonts[0], 8.0, 10.0);

    let mut main_menu = GMMenu::new_empty(menu_title,
        &resources.get_sound("change1").unwrap(),
        &resources.get_sound("enter1").unwrap()
    );

    main_menu.add_item(GMMenuItemEnum::new_static_sprite("TITLE FONT ", 100.0, 100.0,
        &fonts[0], sprites[0], &["CUDDLY", "BBC1", "BLAGGER"], 0), 0.0, 0.0);

    main_menu.add_item(GMMenuItemEnum::new_static_sprite("ITEM FONT ", 0.0, 0.0,
        &fonts[0], sprites[0], &["CUDDLY", "BBC1", "BLAGGER"], 0), 0.0, 40.0);

    main_menu.add_item(GMMenuItemEnum::new_static_sprite("SPRITE ", 0.0, 0.0,
        &fonts[0], sprites[0], &["BAT", "CAT", "GHOST"], 0), 0.0, 40.0);

    main_menu.add_item(GMMenuItemNumeric::new_static_sprite("AMPLITUDE ", 0.0, 0.0,
        &fonts[0], sprites[0], 1.0, 20.0, 8.0, 1.0), 0.0, 40.0);

    main_menu.add_item(GMMenuItemNumeric::new_static_sprite("FREQUENCY ", 0.0, 0.0,
        &fonts[0], sprites[0], 1.0, 30.0, 10.0, 0.5), 0.0, 40.0);

    main_menu.add_item(GMMenuItemNumeric::new_static_sprite("OFFSET ", 0.0, 0.0,
        &fonts[0], sprites[0], 1.0, 3.1, 1.0, 0.1), 0.0, 40.0);

    main_menu.add_item(GMMenuItemStatic::new_static_sprite("EXIT", 0.0, 0.0,
        &fonts[0], sprites[0]), 0.0, 40.0);

    loop {
        clear_background(BLACK);

        main_menu.draw();
        main_menu.update();

        match main_menu.event() {
            None => {
                // Nothing to do...
            }
            Some((i, v)) => {
                println!("User has selected item: {}", i);

                match v {
                    GMValue::None => {
                        if i == 6 {
                            break;
                        }
                    }
                    GMValue::USize(j) => {
                        println!("New usize value: {}", j);

                        if i == 0 {
                            main_menu.set_title_font(fonts[j]);
                        } else if i == 1 {
                            main_menu.set_item_font(fonts[j]);
                        } else if i == 2 {
                            main_menu.change_property_all(&GMKeyValue::new("sprite", Box::new(sprites[j].clone())));
                        }
                    }
                    GMValue::F32(f) => {
                        println!("New f32 value: {}", f);
                        let value: Box<dyn Any> = Box::new(f);

                        if i == 3 {
                            main_menu.change_property_title(&GMKeyValue::new("amplitude", value));
                        } else if i == 4 {
                            main_menu.change_property_title(&GMKeyValue::new("frequency", value));
                        } else if i == 5 {
                            main_menu.change_property_title(&GMKeyValue::new("offset", value));
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
