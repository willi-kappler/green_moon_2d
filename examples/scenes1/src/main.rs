use green_moon_2d::error::GMError;
use green_moon_2d::resource_manager::GMResourceManager;
use green_moon_2d::scene::GMSceneManager;

use macroquad::prelude::*;

use log4rs;

use std::rc::Rc;

mod scene1;
mod scene2;
mod scene3;
mod scene4;

#[macroquad::main("Scenes1")]
async fn main() -> Result<(), GMError> {
    log4rs::init_file("log4rs.yaml", Default::default()).unwrap();

    let resources = Rc::new(GMResourceManager::new_from_file("resources.json").await?);

    let mut scenes_manager = GMSceneManager::new();

    let sc1 = scene1::Scene1::new(&resources);
    let sc2 = scene2::Scene2::new(&resources);
    let sc3 = scene3::Scene3::new(&resources);
    let sc4 = scene4::Scene4::new(&resources);

    scenes_manager.add_scene("scene1", sc1);
    scenes_manager.add_scene("scene2", sc2);
    scenes_manager.add_scene("scene3", sc3);
    scenes_manager.add_scene("scene4", sc4);

    scenes_manager.start_loop().await;

    println!("Good bye!");
    Ok(())
}
