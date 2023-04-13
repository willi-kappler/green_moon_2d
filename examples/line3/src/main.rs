
use std::fs::File;

use simplelog::{WriteLogger, LevelFilter, ConfigBuilder};

use green_moon_2d::{GMEngine, GMSceneT, GMContext, GMEventCode};
use green_moon_2d::bitmap_text::{GMBitmapText};
use green_moon_2d::util::{GMDrawT, GMAlign, GMRepetition, GMUpdateT, GMActiveT, GMVisibleT};
use green_moon_2d::sprite::{GMSprite};
use green_moon_2d::movement::{GMPositionT, GMMV2Points};
use green_moon_2d::line::{GMLineT, GMLine, GMLineMode};
use green_moon_2d::{delegate_active, delegate_visible, delegate_draw, delegate_position};

#[derive(Debug, Clone)]
struct LineElement {
    sprite: GMSprite,
    movement: bool,
}

delegate_active!(LineElement, sprite);
delegate_visible!(LineElement, sprite);
delegate_draw!(LineElement, sprite);
delegate_position!(LineElement, sprite);

impl GMUpdateT for LineElement {
    fn update(&mut self) {
        self.sprite.update();
        todo!();
    }
}

impl GMLineT for LineElement {
    fn clone_box(&self) -> Box<dyn GMLineT> {
        Box::new(self.clone())
    }
}



#[derive(Debug)]
struct LineScene3 {
    title: GMBitmapText,
    move_start: GMMV2Points,
    move_end: GMMV2Points,
    line: GMLine,
}

impl LineScene3 {
    fn new(engine: &GMEngine) -> Self {
        // Set up title text:
        let resources = engine.get_resources();
        let font = resources.get_font("font_cuddly");
        let mut title = GMBitmapText::new(&font, (512.0, 60.0), "LINE 3");
        title.set_align2(GMAlign::BottomCenter);

        // Particle sprite
        let texture = resources.get_texture("tex_particle1");
        let animation = resources.get_animation("anim_particle1");
        let particle_sprite = GMSprite::new(texture, (512.0, 200.0), animation);

        let mut move_start = GMMV2Points::new((20.0, 40.0), (100.0, 700.0), 0.002);
        move_start.get_interpolation_mut().set_repetition(GMRepetition::PingPongForward);

        let mut move_end = GMMV2Points::new((980.0, 40.0), (900.0, 700.0), 0.003);
        move_end.get_interpolation_mut().set_repetition(GMRepetition::PingPongForward);

        let line = GMLine::new((0.0, 0.0), (0.0, 0.0), particle_sprite, GMLineMode::Spacing(30.0));

        Self {
            title,
            move_start,
            move_end,
            line,
        }
    }
}

impl GMSceneT for LineScene3 {
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
    let _simple_log = WriteLogger::init(LevelFilter::Debug, log_config, File::create("line3.log").expect("Could not create log file"));

    let mut engine = GMEngine::new();
    engine.load_configuration_and_init("config.json");

    let line3_scene = LineScene3::new(&engine);

    engine.add_scene("line3_scene", line3_scene);
    engine.run();
}
