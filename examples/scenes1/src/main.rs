use green_moon_2d::error::GMError;
use green_moon_2d::resource_manager::GMResourceManager;
use green_moon_2d::scene::GMSceneManager;

use macroquad::prelude::*;

use log4rs;

use std::rc::Rc;

mod scenes;
use scenes::{Scene1, Scene2, Scene3, Scene4};

#[macroquad::main("Scenes1")]
async fn main() -> Result<(), GMError> {
    log4rs::init_file("log4rs.yaml", Default::default()).unwrap();

    let resources = Rc::new(GMResourceManager::new_from_file("resources.json").await?);

    let mut scenes_manager = GMSceneManager::new();

    let scene1 = Scene1::new(&resources);
    let scene2 = Scene2::new(&resources);
    let scene3 = Scene3::new(&resources);
    let scene4 = Scene4::new(&resources);

    scenes_manager.add_scene("scene1", scene1);
    scenes_manager.add_scene("scene2", scene2);
    scenes_manager.add_scene("scene3", scene3);
    scenes_manager.add_scene("scene4", scene4);

    scenes_manager.start_loop().await;

    println!("Good bye!");
    Ok(())
}
