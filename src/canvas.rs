 struct GM_Canvas {
    texture_pool: Vec<GM_Texture>,
    font_pool: Vec<GM_Font>,
    sprite_sheet_pool: Vec<GM_SpriteSheet>,
    animation_pool: Vec<GM_Animations>,
    sprite_pool: Vec<GM_Sprite>,
    text_pool: Vec<GM_Text>,

    width: u32,
    height: u32,
 }