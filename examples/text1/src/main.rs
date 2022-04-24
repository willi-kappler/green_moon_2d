

use std::fs::File;

use log::{debug, info, error};
use simplelog::{WriteLogger, LevelFilter, ConfigBuilder};

use green_moon_2d::{GMEngine, GMSceneT, GMUpdateContext, GMDrawContext, GMError};

struct TextScene1 {
}

impl TextScene1 {
    pub fn new() -> Self {
        Self {
        }
    }
}

impl GMSceneT for TextScene1 {
    fn update(&mut self, context: &mut GMUpdateContext) -> Result<(), GMError> {
        if context.input.key_esc_down() {
            debug!("ESC key pressed");
            context.quit();
        }

        Ok(())
    }

    fn draw(&mut self, _context: &mut GMDrawContext) -> Result<(), GMError> {
        Ok(())
    }

    fn get_name(&self) -> &str {
        "Text1"
    }
}


fn main() {
    let config = ConfigBuilder::new().build();
    let _simple_log = WriteLogger::init(LevelFilter::Debug, config, File::create("test1.log").unwrap());

    let text1_scene = TextScene1::new();

    let mut app = GMEngine::new();
    app.add_scene(text1_scene).unwrap();

    match app.run() {
        Ok(_) => {
            info!("Quit app");
        }
        Err(e) => {
            error!("An error occurred: '{}'", e);
        }
    }
}
