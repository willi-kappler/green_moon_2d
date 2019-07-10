 

// Local modules
use crate::texture::{GM_Texture};
use crate::font::{GM_BitmapFont};
use crate::spritesheet::{GM_SpriteSheet};
use crate::animation::{GM_Animation_T};
use crate::path::{GM_Path_T};
use crate::sprite::{GM_Sprite_T};
use crate::text::{GM_Text_T};




 pub struct GM_Canvas {
    texture_pool: Vec<GM_Texture>,
    font_pool: Vec<GM_BitmapFont>,
    sprite_sheet_pool: Vec<GM_SpriteSheet>,
    animation_pool: Vec<Box<dyn GM_Animation_T>>,
    path_pool: Vec<Box<dyn GM_Path_T>>,
    sprite_pool: Vec<Box<dyn GM_Sprite_T>>,
    text_pool: Vec<Box<dyn GM_Text_T>>,
    width: u32,
    height: u32,
 }

impl GM_Canvas {
   pub fn new() -> GM_Canvas {
      GM_Canvas {
         texture_pool: Vec::new(),
         font_pool: Vec::new(),
         sprite_sheet_pool: Vec::new(),
         animation_pool: Vec::new(),
         path_pool: Vec::new(),
         sprite_pool: Vec::new(),
         text_pool: Vec::new(),
         width: 0,
         height: 0,
      }
   }

   pub fn update(&mut self, time_elapsed: u16) {
      for sprite in self.sprite_pool.iter_mut() {
         if sprite.is_active() {
            sprite.update(time_elapsed);
         }
      }

      for text in self.text_pool.iter_mut() {
         if text.is_active() {
            text.update(time_elapsed);
         }
      }

      for animation in self.animation_pool.iter_mut() {
         if animation.is_active() {
            animation.update(time_elapsed);
         }
      }

      for path in self.path_pool.iter_mut() {
         if path.is_active() {
            path.update(time_elapsed);
         }
      }
   }

   pub fn draw(&mut self) {

   }

   fn draw_texture(&mut self, texture_id: usize) {

   }
}


