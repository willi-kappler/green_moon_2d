use green_moon_2d::error::GMError;
use green_moon_2d::sprite::GMSprite;
use green_moon_2d::resource_manager::GMResourceManager;
use green_moon_2d::particle::GMParticleEmitter;

use macroquad::prelude::*;

use log4rs;

#[macroquad::main("Particle")]
async fn main() -> Result<(), GMError> {
    log4rs::init_file("log4rs.yaml", Default::default()).unwrap();

    let resources = GMResourceManager::new_from_file("resources.json").await?;

    let mut sprite1 = GMSprite::new(
        &resources.get_sprite_sheet("particle1").unwrap(),
        resources.get_animation("particle1").unwrap(), 0.0, 0.0);
    sprite1.start_animation();

    let mut sprite2 = GMSprite::new(
        &resources.get_sprite_sheet("heads").unwrap(),
        resources.get_animation("heads").unwrap(), 0.0, 0.0);
    sprite2.start_animation();

    let mut emitter1 = GMParticleEmitter::new(&sprite1, 200.0, 300.0);
    emitter1.set_active(true);
    emitter1.set_speed_max(2.0);
    emitter1.set_duration(2.0);
    emitter1.set_rot_speed_max(0.1);

    let mut emitter2 = GMParticleEmitter::new(&sprite2, 600.0, 300.0);
    emitter2.set_active(true);
    emitter2.set_speed_max(2.0);
    emitter2.set_duration(2.0);
    emitter2.set_delay(0.2);
    emitter2.set_rot_speed_max(0.05);


    loop {
        clear_background(BLACK);

        emitter1.draw();
        emitter1.update();

        emitter2.draw();
        emitter2.update();

        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        next_frame().await
    }

    println!("Good bye!");
    Ok(())
}
