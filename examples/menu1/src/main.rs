use green_moon_2d::menu::GMMenu;
use green_moon_2d::text::{GMStaticText, GMArrowText, GMTextT};
use green_moon_2d::font::{GMBitmapFont, GMFontT};
use green_moon_2d::error::GMError;

use macroquad::prelude::*;

use std::rc::Rc;

#[macroquad::main("Menu1")]
async fn main() -> Result<(), GMError> {
    let font = GMBitmapFont::new("../assets/gfx/fonts/cuddly.png", 32.0, 32.0, "ABCDEFGHIJKLMNOPQRSTUVWXYZ.!0123456789?()<>- ").await?;
    let font = Rc::new(font);

    let title = GMStaticText::new("MAIN MENU", 128.0, 0.0, &font);
    let arrow_text = Box::new(GMStaticText::new("ARROW TEXT", 128.0, 64.0, &font));
    let mut arrow_text = GMArrowText::new(arrow_text);

    loop {
        clear_background(BLACK);

        title.draw();
        arrow_text.draw();
        arrow_text.update();

        next_frame().await
    }

    Ok(())
}
