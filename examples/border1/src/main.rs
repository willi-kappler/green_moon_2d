use green_moon_2d::error::GMError;
use green_moon_2d::resources::GMResourceManager;
use green_moon_2d::particle::GMParticleManager;

use macroquad::prelude::*;

use log4rs;

#[macroquad::main("Border1")]
async fn main() -> Result<(), GMError> {
    log4rs::init_file("log4rs.yaml", Default::default()).unwrap();

    let resources = GMResourceManager::new_from_file("resources.json").await?;

    // let tileset1 = resources.get_tileset("set1").unwrap();
    // let tileset2 = resources.get_tileset("set2").unwrap();

    let border1 = resources.get_border("border1").unwrap();
    let border2 = resources.get_border("border2").unwrap();
    let border3 = resources.get_border("border3").unwrap();
    let border4 = resources.get_border("border4").unwrap();

    loop {
        clear_background(BLACK);

        border1.draw();
        border1.update();

        border2.draw();
        border2.update();

        border3.draw();
        border3.update();

        border4.draw();
        border4.update();

        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        next_frame().await
    }

    println!("Good bye!");
    Ok(())
}
