 

// Local modules
use crate::texture::{GM_Texture};

pub struct GM_Canvas {
   width: u32,
   height: u32,
}

impl GM_Canvas {
   pub fn new() -> GM_Canvas {
      GM_Canvas {
         width: 0,
         height: 0,
      }
   }

   pub fn draw_texture(&mut self, x: u32, y: u32, texture: &GM_Texture) {
   }

   pub fn draw_sub_texture(&mut self, x: u32, y: u32, texture: &GM_Texture, tx: u32, ty: u32, tw: u32, th: u32) {

   }
}
