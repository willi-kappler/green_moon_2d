use green_moon_2d::menu::GMMenu;
use green_moon_2d::error::GMError;
use green_moon_2d::resource_manager::GMResourceManager;

use macroquad::prelude::*;

use log4rs;

use std::thread;
use std::time::Duration;

#[macroquad::main("Menu1")]
async fn main() -> Result<(), GMError> {
    log4rs::init_file("log4rs.yaml", Default::default()).unwrap();

    let resources = GMResourceManager::new_from_file("resources.json").await?;

    let items = ["START", "CONTROLS", "GFX OPTIONS", "SFX OPTIONS", "HIGH SCORE", "CREDITS", "EXIT"];
    let mut main_menu = GMMenu::new_static_arrow(240.0, 100.0, "MAIN MENU", &items,
        &resources.get_font("cuddly").unwrap(),
        &resources.get_sound("change").unwrap(),
        &resources.get_sound("enter").unwrap());

    show_mouse(true);

    loop {
        clear_background(BLACK);

        main_menu.draw();
        main_menu.update();

        if let Some((i, _)) = main_menu.event() {
            println!("User has selected item: {}", i);

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
