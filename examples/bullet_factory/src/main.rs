use green_moon_2d::error::GMError;
use green_moon_2d::resource_manager::GMResourceManager;
use green_moon_2d::bullet_factory::GMBulletFactory;

use macroquad::prelude::*;

use log4rs;

#[macroquad::main("BulletFactory")]
async fn main() -> Result<(), GMError> {
    log4rs::init_file("log4rs.yaml", Default::default()).unwrap();

    let resources = GMResourceManager::new_from_file("resources.json").await?;

    let bullet = resources.get_sprite("bullet1").unwrap();
    let mut bullet_factory = GMBulletFactory::new(bullet, 20);
    bullet_factory.set_delay(0.1);
    let bullet_speed = 4.0;

    let mut player = resources.get_sprite("ship1").unwrap().clone();
    player.set_x(400.0);
    player.set_y(300.0);

    show_mouse(true);

    loop {
        clear_background(BLACK);

        bullet_factory.draw();
        player.draw();

        bullet_factory.update();
        player.update();

        let (mousex, mousey) = mouse_position();
        player.rotate_to_point(mousex, mousey);

        if is_mouse_button_pressed(MouseButton::Left) {
            let rotation = player.get_rotation();
            let bullet_vx = rotation.cos() * bullet_speed;
            let bullet_vy = rotation.sin() * bullet_speed;
            bullet_factory.add_bullet(player.get_mid_x(), player.get_mid_y(), bullet_vx, bullet_vy, rotation, true);
        }

        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        next_frame().await
    }

    println!("Good bye!");
    Ok(())
}
