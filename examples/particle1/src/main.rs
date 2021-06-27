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
    let mut sprite = GMSprite::new(
        &resources.get_sprite_sheet("particle1").unwrap(),
        resources.get_animation("particle1").unwrap(), 0.0, 0.0);
    sprite.start_animation();
    let mut emitter = GMParticleEmitter::new(&sprite, 400.0, 300.0);

    emitter.set_active(true);
    emitter.set_speed_max(2.0);
    emitter.set_duration(2.0);
    emitter.set_rot_speed_max(0.1);

    loop {
        clear_background(BLACK);

        emitter.draw();
        emitter.update();

        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        next_frame().await
    }

    println!("Good bye!");
    Ok(())
}
