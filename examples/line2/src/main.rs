
use std::fs::File;

use simplelog::{WriteLogger, LevelFilter, ConfigBuilder};

use green_moon_2d::{GMEngine, GMSceneT, GMContext, GMEventCode};
use green_moon_2d::bitmap_text::{GMBitmapText};
use green_moon_2d::util::{GMDrawT, GMAlign, GMRepetition, GMUpdateT};
use green_moon_2d::sprite::{GMSprite};
use green_moon_2d::movement::{GMMVCircle};
use green_moon_2d::line::{GMLine, GMLineMode};

#[derive(Debug)]
struct LineScene2 {
    title: GMBitmapText,
    move_start: GMMVCircle,
    move_end: GMMVCircle,
    line: GMLine,
}

impl LineScene2 {
    fn new(engine: &GMEngine) -> Self {
        // Set up title text:
        let resources = engine.get_resources();
        let font = resources.get_font("font_cuddly");
        let mut title = GMBitmapText::new(&font, (512.0, 60.0), "LINE 2");
        title.set_align2(GMAlign::BottomCenter);

        // Particle sprite
        let texture = resources.get_texture("tex_particle1");
        let animation = resources.get_animation("anim_particle1");
        let particle_sprite = GMSprite::new(texture, (512.0, 200.0), animation);

        let mut move_start = GMMVCircle::new(0.0, 360.0, 0.02, (80.0, 200.0), 60.0);
        move_start.get_interpolation_mut().set_repetition(GMRepetition::LoopForward);

        let mut move_end = GMMVCircle::new(0.0, 360.0, 0.01, (900.0, 600.0), 100.0);
        move_end.get_interpolation_mut().set_repetition(GMRepetition::LoopBackward);

        let line = GMLine::new((0.0, 0.0), (0.0, 0.0), particle_sprite, GMLineMode::Number(30));

        Self {
            title,
            move_start,
            move_end,
            line,
        }
    }
}

impl GMSceneT for LineScene2 {
    fn update(&mut self, context: &mut GMContext) {
        if context.event(GMEventCode::KeyESCUp) ||
           context.event(GMEventCode::Quit) ||
           context.event(GMEventCode::WindowClose) {
            context.quit();
        }

        self.move_start.set_and_update_n(&mut self.line, 0);
        self.move_end.set_and_update_n(&mut self.line, 1);
        self.line.end_point_changed();
        self.line.update();
    }

    fn draw(&self, context: &mut GMContext) {
        context.clear_black();

        self.title.draw(context);

        self.line.draw(context);
    }
}

fn main() {
    let log_config = ConfigBuilder::new().build();
    let _simple_log = WriteLogger::init(LevelFilter::Debug, log_config, File::create("line2.log").expect("Could not create log file"));

    let mut engine = GMEngine::new();
    engine.load_configuration_and_init("config.json");

    let line2_scene = LineScene2::new(&engine);

    engine.add_scene("line2_scene", line2_scene);
    engine.run();
}
