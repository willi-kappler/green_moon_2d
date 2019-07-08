 
 pub struct GM_Canvas {
    texture_pool: Vec<GM_Texture>,
    font_pool: Vec<GM_BitmapFont>,
    sprite_sheet_pool: Vec<GM_SpriteSheet>,
    animation_pool: Vec<GM_Animations>,
    sprite_pool: Vec<dyn GM_Sprite_T>,
    text_pool: Vec<dyn GM_Text_T>,

    width: u32,
    height: u32,
 }

impl GM_Canvas {
   fn new() -> GM_Canvas {
      GM_Canvas {
         texture_pool: Vec::new(),
         font_pool: Vec::new(),
         sprite_sheet_pool: Vec::new(),
         animation_pool: Vec::new(),
         sprite_pool: Vec::new(),
         text_pool: Vec::new(),
         width: 0,
         height: 0,
      }
   }
}


