
// Local modules
use crate::active::GM_Active_T;
use crate::animation::{GM_Animation_T};
use crate::canvas::{GM_Canvas};
use crate::font::{GM_BitmapFont};
use crate::music::{GM_Music};
use crate::path::{GM_Path_T};
use crate::settings::{GM_Settings};
use crate::sound::{GM_Sound};
use crate::sprite::{GM_Sprite_T};
use crate::spritesheet::{GM_SpriteSheet};
use crate::text::{GM_Text};
use crate::texteffect::{GM_TextEffect_T};
use crate::texture::{GM_Texture};


pub struct GM_Resources {
    // GFX
    canvas: GM_Canvas,
    texture_pool: Vec<GM_Texture>,
    font_pool: Vec<GM_BitmapFont>,
    sprite_sheet_pool: Vec<GM_SpriteSheet>,
    animation_pool: Vec<Box<dyn GM_Animation_T>>,
    path_pool: Vec<Box<dyn GM_Path_T>>,
    sprite_pool: Vec<Box<dyn GM_Sprite_T>>,
    text_pool: Vec<GM_Text>,
    text_effect_pool: Vec<Box<dyn GM_TextEffect_T>>,

    // SFX
    sound_pool: Vec<GM_Sound>,
    music_pool: Vec<GM_Music>,

    // Misc
    settings: GM_Settings,
    quit: bool,
    time_elapsed: u16,
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
            text_effect_pool: Vec::new(),
            sound_pool: Vec::new(),
            music_pool: Vec::new(),
            settings: GM_Settings::new(),
            quit: false,
            time_elapsed: 0,
        }
    }
    pub fn quit_game(&mut self) {
        self.quit = true;
    }

    pub fn game_still_running(&self) -> bool {
        !self.quit
    }

    pub fn frame_duration(&self) -> i16 {
        self.settings.frame_duration()
    }

    pub fn set_time_elapsed(&mut self, time_elapsed: u16) {
        self.time_elapsed = time_elapsed;
    }

    pub fn update(&mut self) {
        for sprite in self.sprite_pool.iter_mut() {
            if sprite.is_active() {
                sprite.update(self.time_elapsed);
            }
        }

        for text in self.text_pool.iter_mut() {
            if text.is_active() {
                text.update(self.time_elapsed, &mut self.text_effect_pool);
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
                sprite.draw(&self.sprite_sheet_pool,
                    &self.texture_pool,
                    &self.animation_pool,
                    &mut self.canvas);
            }
        }

        for text in self.text_pool.iter() {
            if text.is_active() {
                text.draw(&self.font_pool,
                    &self.texture_pool,
                    &self.text_effect_pool,
                    &mut self.canvas);
            }
        }
    }
}
