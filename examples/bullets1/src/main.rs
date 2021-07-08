use green_moon_2d::error::GMError;
use green_moon_2d::resources::GMResourceManager;
use green_moon_2d::bullets::{GMBulletManager, GMOffscreenMode};

use macroquad::prelude::*;

use log4rs;

#[macroquad::main("BulletFactory")]
async fn main() -> Result<(), GMError> {
    log4rs::init_file("log4rs.yaml", Default::default()).unwrap();

    let resources = GMResourceManager::new_from_file("resources.json").await?;

    let bullet = resources.get_sprite("bullet1").unwrap();
    let mut bullet_manager = GMBulletManager::new(bullet, 30);
    bullet_manager.set_delay(0.05);
    //bullet_manager.set_offscreen_mode(GMOffscreenMode::WrapAround);
    let bullet_speed = 6.0;

    let mut player = resources.get_sprite("ship1").unwrap().clone();
    player.set_x(400.0);
    player.set_y(300.0);

    let laser = resources.get_sound("laser1").unwrap();

    show_mouse(true);

    loop {
        clear_background(BLACK);

        bullet_manager.draw();
        player.draw();

        bullet_manager.update();
        player.update();

        let (mousex, mousey) = mouse_position();
        player.rotate_to_point(mousex, mousey);

        if is_mouse_button_pressed(MouseButton::Left) {
            laser.play();

            let rotation = player.get_rotation();
            let bullet_vx = rotation.cos() * bullet_speed;
            let bullet_vy = rotation.sin() * bullet_speed;
            bullet_manager.add_bullet(player.get_mid_x(), player.get_mid_y(), bullet_vx, bullet_vy, rotation, true);
        }

        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        next_frame().await
    }

    println!("Good bye!");
    Ok(())
}
