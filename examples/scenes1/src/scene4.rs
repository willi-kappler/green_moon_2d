use green_moon_2d::menu::GMMenu;
use green_moon_2d::scene::{GMSceneT, GMSceneResult};
use green_moon_2d::resource_manager::GMResourceManager;

use macroquad::prelude::*;

pub struct Scene4 {
    menu: GMMenu,
}

impl Scene4 {
    pub fn new(resources: &GMResourceManager) -> Box<dyn GMSceneT> {
        let items = ["SCENE 1", "SCENE 2", "SCENE 3", "EXIT"];
        let menu = GMMenu::new_static_arrow(240.0, 100.0, "SCENE 4", &items,
            &resources.get_font("bbc1").unwrap(),
            &resources.get_sound("change1").unwrap(),
            &resources.get_sound("enter1").unwrap());

        let result = Self {
            menu,
        };
        Box::new(result)
    }
}

impl GMSceneT for Scene4 {
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
        use GMSceneResult::*;

        if let Some((i, _)) = self.menu.event() {
            println!("Scene 4, user has selected item: {}", i);

            match i {
                0 => {
                    GMChangeScene("scene1".to_string())
                }
                1 => {
                    GMChangeScene("scene2".to_string())
                }
                2 => {
                    GMChangeScene("scene3".to_string())
                }
                _ => {
                    GMExit
                }
            }

        } else {
            GMKeepScene
        }
    }
}
