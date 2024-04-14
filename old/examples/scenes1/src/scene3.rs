use green_moon_2d::menu::GMMenu;
use green_moon_2d::scene::{GMSceneT, GMSceneResult};
use green_moon_2d::resources::GMResourceManager;

use macroquad::prelude::*;

pub struct Scene3 {
    menu: GMMenu,
}

impl Scene3 {
    pub fn new(resources: &GMResourceManager) -> Box<dyn GMSceneT> {
        let items = ["SCENE 1", "SCENE 2", "SCENE 4", "EXIT"];
        let menu = GMMenu::new_static_arrow(240.0, 100.0, "SCENE 3", &items,
            &resources.get_font("cuddly").unwrap(),
            &resources.get_sound("change1").unwrap(),
            &resources.get_sound("enter1").unwrap());

        let result = Self {
            menu,
        };
        Box::new(result)
    }
}

impl GMSceneT for Scene3 {
    fn init(&mut self) {
        println!("Scene 3, init()");
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
            println!("Scene 3, user has selected item: {}", i);

            match i {
                0 => {
                    GMSceneResult::ChangeScene("scene1".to_string())
                }
                1 => {
                    GMSceneResult::ChangeScene("scene2".to_string())
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
