
use macroquad::window::next_frame;

pub enum GMSceneResult {
    KeepScene,
    ChangeScene(String),
    Exit,
}

pub trait GMSceneT {
    fn init(&mut self);
    fn draw(&self);
    fn update(&mut self);
    fn event(&mut self) -> GMSceneResult;
}

pub struct GMSceneManager {
    scenes: Vec<(String, Box<dyn GMSceneT>)>,
    current_scene: usize,
}

impl GMSceneManager {
    pub fn new() -> Self {
        Self {
            scenes: Vec::new(),
            current_scene: 0,
        }
    }

    pub fn add_scene(&mut self, name: &str, scene: Box<dyn GMSceneT>) {
        self.scenes.push((name.to_string(), scene));

        if self.scenes.len() == 1 {
            self.scenes[0].1.init();
        }
    }

    pub fn remove_scene(&mut self, name: &str) {
        let result = self.scenes.iter().position(|x| x.0 == name);
        if let Some(i) = result {
            self.scenes.remove(i);
        }
    }

    pub fn draw(&self)  {
        self.scenes[self.current_scene].1.draw();
    }

    pub fn update(&mut self) {
        self.scenes[self.current_scene].1.update();
    }

    pub fn event(&mut self) -> bool {
        let result = self.scenes[self.current_scene].1.event();

        use GMSceneResult::*;

        match result {
            KeepScene => {
                false
            }
            ChangeScene(new_scene) => {
                let mut change_scene = false;

                for i in 0..self.scenes.len() {
                    if self.scenes[i].0 == new_scene {
                        self.current_scene = i;
                        change_scene = true;
                        break;
                    }
                }

                if change_scene {
                    self.scenes[self.current_scene].1.init();
                }

                false
            }
            Exit => {
                true
            }
        }
    }

    pub async fn start_loop(&mut self) {
        loop {
            self.draw();
            self.update();

            if self.event() {
                break;
            }

            next_frame().await
        }
    }
}
