use green_moon_2d::sprite::GMSprite;
use green_moon_2d::error::GMError;
use green_moon_2d::resource_manager::GMResourceManager;

use macroquad::prelude::*;

use log4rs;

#[macroquad::main("BulletFactory")]
async fn main() -> Result<(), GMError> {
    log4rs::init_file("log4rs.yaml", Default::default()).unwrap();

    let resources = GMResourceManager::new_from_file("resources.json").await?;

    loop {
        clear_background(BLACK);

        next_frame().await
    }

    println!("Good bye!");
    Ok(())
}
