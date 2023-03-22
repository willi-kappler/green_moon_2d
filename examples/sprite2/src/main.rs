
use std::fs::File;

use simplelog::{WriteLogger, LevelFilter, ConfigBuilder};

use green_moon_2d::{GMEngine, GMSceneT, GMContext, GMEventCode};
use green_moon_2d::bitmap_text::{GMBitmapText};
use green_moon_2d::util::{GMDrawT, GMAlign, GMRepetition, GMUpdateT, GMFlipXYT};
use green_moon_2d::sprite::{GMSprite};
use green_moon_2d::movement::{GMPositionT, GMMVCircle, GMMVCircleMultiple, GMMVScale, GMMVPolygon, GMMVFollow};

#[derive(Debug)]
struct SpriteScene2 {
    title: GMBitmapText,
    ghost_text: GMBitmapText,
    ghost_sprite: GMSprite,
    ghost_circle: GMMVCircle,
    text_circle: GMMVCircle,

    multiple_ghosts: Vec<GMSprite>,
    multi_circle: GMMVCircleMultiple,
    multi_circle_scale: GMMVScale,

    ice_sprite: GMSprite,
    head_sprite: GMSprite,

    ice_polygon: GMMVPolygon,
    ice_follow: GMMVFollow,
}

impl SpriteScene2 {
    fn new(engine: &GMEngine) -> Self {
        let resources = engine.get_resources();

        // Set up title text:
        let font = resources.get_font("font_cuddly");
        let mut title = GMBitmapText::new(&font, (512.0, 100.0), "SPRITE 2");
        title.set_align2(GMAlign::BottomCenter);

        // Set up circle text
        let mut ghost_text = GMBitmapText::new(font, (0.0, 0.0), "BOOO!");
        ghost_text.set_align(GMAlign::MiddleCenter);
        ghost_text.reset_positions();

        // Set up ghost sprite
        let texture = resources.get_texture("tex_ghost1");
        let animation = resources.get_animation("anim_ghost1");
        let ghost_sprite = GMSprite::new(texture, (0.0, 0.0), animation.clone());

        // Set up circle movement for ghost
        let mut ghost_circle = GMMVCircle::new(0.0, 360.0, 0.001, (250.0, 250.0), 100.0);
        ghost_circle.get_interpolation_mut().set_repetition(GMRepetition::LoopForward);

        // Set up circle movement for text
        let mut text_circle = GMMVCircle::new(0.0, 360.0, 0.01, (0.0, 0.0 / 2.0), 50.0);
        text_circle.get_interpolation_mut().set_repetition(GMRepetition::LoopForward);

        // Multiple sprites on a circle:
        let mut multiple_ghosts = Vec::new();


        for _ in 0..4 {
            let sprite = GMSprite::new(texture, (0.0, 0.0), animation.clone());
            multiple_ghosts.push(sprite);
        }

        multiple_ghosts[0].set_flip_x(true);
        multiple_ghosts[2].set_flip_x(true);

        let mut multi_circle = GMMVCircleMultiple::new(0.0, 360.0, 90.0, 0.005, (600.0, 250.0), 100.0);
        multi_circle.get_interpolation_mut().set_repetition(GMRepetition::LoopForward);

        let mut multi_circle_scale = GMMVScale::new(50.0, 100.0, 0.02);
        multi_circle_scale.get_interpolation_mut().set_repetition(GMRepetition::PingPongForward);

        // Head following ice:
        // Ice:
        let texture = resources.get_texture("tex_ice2");
        let animation = resources.get_empty_animation();
        let ice_sprite = GMSprite::new(texture, (0.0, 0.0), animation);

        // Head:
        let texture = resources.get_texture("tex_head1");
        let animation = resources.get_animation("anim_head1");
        let head_sprite = GMSprite::new(texture, (0.0, 0.0), animation);

        // Polygon movement:
        let mut ice_polygon = GMMVPolygon::new2(&[(50.0, 600.0), (900.0, 400.0), (800.0, 700.0), (100.0, 700.0), (50.0, 600.0)]);
        ice_polygon.set_repetition(GMRepetition::LoopForward);

        // Follow the ice
        let ice_follow = GMMVFollow::new(3.0, 1.0, (512.0, 600.0));

        Self {
            title,
            ghost_text,
            ghost_sprite,
            ghost_circle,
            text_circle,
            multiple_ghosts,
            multi_circle,
            multi_circle_scale,
            ice_sprite,
            head_sprite,
            ice_polygon,
            ice_follow,
        }
    }
}

impl GMSceneT for SpriteScene2 {
    fn update(&mut self, context: &mut GMContext) {
        if context.event(GMEventCode::KeyESCUp) ||
           context.event(GMEventCode::Quit) ||
           context.event(GMEventCode::WindowClose) {
            context.quit();
        }

        // Update animation
        self.ghost_sprite.update();

        self.ghost_circle.set_position_of(&mut self.ghost_sprite);
        self.ghost_circle.set_position_of(&mut self.text_circle);
        self.ghost_circle.update();

        self.text_circle.set_and_update(&mut self.ghost_text);

        for i in 0..4 {
            let sprite = &mut self.multiple_ghosts[i];
            sprite.update();
            self.multi_circle.set_position_of(sprite, i as u32);
        }

        self.multi_circle.update();
        self.multi_circle_scale.set_and_update(&mut self.multi_circle);

        self.head_sprite.update();

        self.ice_polygon.set_and_update(&mut self.ice_sprite);
        self.ice_follow.set_position_of(&mut self.head_sprite);
        self.ice_follow.set_target(&self.ice_sprite.get_position());
        self.ice_follow.update();
    }

    fn draw(&self, context: &mut GMContext) {
        context.clear_black();

        self.title.draw(context);
        self.ghost_text.draw(context);
        self.ghost_sprite.draw(context);

        for i in 0..4 {
            self.multiple_ghosts[i].draw(context);
        }

        self.ice_sprite.draw(context);
        self.head_sprite.draw(context);
    }
}

fn main() {
    let log_config = ConfigBuilder::new().build();
    let _simple_log = WriteLogger::init(LevelFilter::Debug, log_config, File::create("sprite2.log").expect("Could not create log file"));

    let mut engine = GMEngine::new();
    engine.load_configuration_and_init("config.json");

    let sprite2_scene = SpriteScene2::new(&engine);

    engine.add_scene("sprite2_scene", sprite2_scene);
    engine.run();
}
