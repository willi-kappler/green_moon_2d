
// Local modules
use crate::canvas::{GM_Canvas};
use crate::settings::{GM_Settings};
use crate::sound::{GM_Sound};
use crate::music::{GM_Music};
use crate::font::{GM_BitmapFont};
use crate::spritesheet::{GM_SpriteSheet};
use crate::texture::{GM_Texture};
use crate::animation::{GM_Animation_T};
use crate::path::{GM_Path_T};
use crate::sprite::{GM_Sprite_T};
use crate::text::{GM_Text_T};


pub struct GM_Resources {
    // GFX
    pub (crate) canvas: GM_Canvas,
    texture_pool: Vec<GM_Texture>,
    font_pool: Vec<GM_BitmapFont>,
    sprite_sheet_pool: Vec<GM_SpriteSheet>,
    animation_pool: Vec<Box<dyn GM_Animation_T>>,
    path_pool: Vec<Box<dyn GM_Path_T>>,
    sprite_pool: Vec<Box<dyn GM_Sprite_T>>,
    text_pool: Vec<Box<dyn GM_Text_T>>,

    // SFX
    sound_pool: Vec<GM_Sound>,
    music_pool: Vec<GM_Music>,

    // Misc
    settings: GM_Settings,
    pub quit: bool,
    pub time_elapsed: u16,
}

impl GM_Resources {
    pub fn new() -> GM_Resources {
        GM_Resources {
            canvas: GM_Canvas::new(),
            texture_pool: Vec::new(),
            font_pool: Vec::new(),
            sprite_sheet_pool: Vec::new(),
            animation_pool: Vec::new(),
            path_pool: Vec::new(),
            sprite_pool: Vec::new(),
            text_pool: Vec::new(),
            sound_pool: Vec::new(),
            music_pool: Vec::new(),
            settings: GM_Settings::new(),
            quit: false,
            time_elapsed: 0,
        }
    }
/*
    pub fn quit_game(&mut self) {
        self.quit = true;
    }
*/
    pub fn update(&mut self) {
        for sprite in self.sprite_pool.iter_mut() {
            if sprite.is_active() {
                sprite.update(self.time_elapsed);
            }
        }

        for text in self.text_pool.iter_mut() {
            if text.is_active() {
                text.update(self.time_elapsed);
            }
        }

        for animation in self.animation_pool.iter_mut() {
            if animation.is_active() {
                animation.update(self.time_elapsed);
            }
        }

        for path in self.path_pool.iter_mut() {
            if path.is_active() {
                path.update(self.time_elapsed);
            }
        }
    }

    pub fn draw(&mut self) {
        for sprite in self.sprite_pool.iter() {
            if sprite.is_active() {
                let sprite_sheet = &self.sprite_sheet_pool[sprite.get_sprite_sheet_id()];
                let texture = &self.texture_pool[sprite_sheet.texture_id];
                let animation = &self.animation_pool[sprite.get_animation_id()];
                let (tx, ty) = sprite_sheet.frame_to_coordinates(animation.current_frame());
                let sx = sprite.get_x();
                let sy = sprite.get_y();
                self.canvas.draw_sub_texture(sx, sx, &texture, tx, ty, sprite_sheet.cell_width, sprite_sheet.cell_height);
            }
        }
    }
}
