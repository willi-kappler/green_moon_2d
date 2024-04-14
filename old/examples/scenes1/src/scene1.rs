use green_moon_2d::menu::GMMenu;
use green_moon_2d::scene::{GMSceneT, GMSceneResult};
use green_moon_2d::resources::GMResourceManager;

use macroquad::prelude::*;

pub struct Scene1 {
    menu: GMMenu,
}

impl Scene1 {
    pub fn new(resources: &GMResourceManager) -> Box<dyn GMSceneT> {
        let items = ["SCENE 2", "SCENE 3", "SCENE 4", "EXIT"];
        let menu = GMMenu::new_static_arrow(240.0, 100.0, "SCENE 1", &items,
            &resources.get_font("cuddly").unwrap(),
            &resources.get_sound("change1").unwrap(),
            &resources.get_sound("enter1").unwrap());

        let result = Self {
            menu,
        };
        Box::new(result)
    }
}

impl GMSceneT for Scene1 {
    fn init(&mut self) {
        println!("Scene 1, init()");
    }
    fn draw(&self) {
        clear_background(BLACK);
        self.menu.draw();
    }
    fn update(&mut self) {
        self.menu.update();
    }
    fn event(&mut self) -> GMSceneResult {
        if let Some((i, _)) = self.menu.event() {
            println!("Scene 1, user has selected item: {}", i);

            match i {
                0 => {
                    GMSceneResult::ChangeScene("scene2".to_string())
                }
                1 => {
                    GMSceneResult::ChangeScene("scene3".to_string())
                }
                2 => {
                    GMSceneResult::ChangeScene("scene4".to_string())
                }
                _ => {
                    GMSceneResult::Exit
                }
            }

        } else {
            GMSceneResult::KeepScene
        }
    }
}
