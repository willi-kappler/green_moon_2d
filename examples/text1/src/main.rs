

use std::fs::File;

use log::{debug, info, error};
use simplelog::{WriteLogger, LevelFilter, ConfigBuilder};

use green_moon_2d::{GMApp, GMSceneT, GMContext, GMError, GMDrawContainer};

struct TextScene1 {
}

impl TextScene1 {
    pub fn new() -> Self {
        Self {
        }
    }
}

impl GMSceneT for TextScene1 {
    fn update_after(&mut self, context: &mut GMContext, draw_objects: &mut GMDrawContainer) -> Result<(), GMError> {
        let esc_pressed = context.key_esc_down();

        if esc_pressed {
            debug!("ESC key pressed");
            context.quit_app();
        }

        Ok(())
    }
}


fn main() {
    let config = ConfigBuilder::new().build();
    let _simple_log = WriteLogger::init(LevelFilter::Debug, config, File::create("test1.log").unwrap());

    let first_scene = TextScene1::new();

    let mut app = GMApp::new("TextScene1", first_scene);

    match app.run() {
        Ok(_) => {
            info!("Quit app");
        }
        Err(e) => {
            error!("An error occurred: '{}'", e);
        }
    }
}
