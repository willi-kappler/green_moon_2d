

use std::fs::File;

use log::{debug};
use simplelog::{WriteLogger, LevelFilter, ConfigBuilder};

use green_moon_2d::{GMEngine, GMSceneT, GMContext, GMSceneMessage, GMSceneReply};

#[derive(Clone, Debug)]
struct TextScene1 {
}

impl TextScene1 {
    pub fn new() -> Self {
        Self {
        }
    }
}

impl GMSceneT for TextScene1 {
    fn send_message(&mut self, message: GMSceneMessage, context: &mut GMContext) -> GMSceneReply {
        use GMSceneMessage::*;

        match message {
            Update => {
                if context.input.key_esc_down() {
                    debug!("ESC key pressed");
                    context.quit();
                }        
            }
            _ => {
                debug!("Unhandled message: {:?}", message);
            }
        }

        GMSceneReply::Empty
    }
}

fn main() {
    let config = ConfigBuilder::new().build();
    let _simple_log = WriteLogger::init(LevelFilter::Debug, config, File::create("test1.log").expect("Could not create log file"));

    let text1_scene = TextScene1::new();

    let mut engine = GMEngine::new();
    engine.init();
    engine.add_scene("text1", text1_scene);
    engine.run();
}
