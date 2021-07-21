

use crate::error::GMError;
use crate::font::{GMBitmapFont, GMFont};
use crate::spritesheet::GMSpriteSheet;
use crate::sprite::{GMSprite, GMSpriteSingle, GMSpriteSimple};
use crate::sound::GMSound;
use crate::animation::{GMAnimationBackwardLoop, GMAnimationBackwardOnce, GMAnimationForwardLoop, GMAnimationForwardOnce, GMAnimationPingPong, GMAnimation};
use crate::tilemap::GMTileMap;
use crate::tileset::GMTileSet;
use crate::tilewindow::GMTileWindow;

use macroquad::file::load_string;
use macroquad::math::Rect;
use nanoserde::DeJson;

use log::{debug, info};

use std::collections::HashMap;
use std::rc::Rc;
use std::path::Path;

// TODO:
// - get_bullet_manager()
// - get_particle_manager()
// - get_menu_item()
// - get_border()
// -

#[derive(Clone, Debug, Default, DeJson)]
pub struct GMFormatResource {
    font_files: Option<Vec<String>>,
    sprite_sheets: Option<Vec<GMFormatSpriteSheet>>,
    sprites: Option<Vec<GMFormatSprite>>,
    sounds: Option<Vec<GMFormatSound>>,
    animation_files: Option<Vec<String>>,
    tileset_files: Option<Vec<String>>,
    tilemap_files: Option<Vec<String>>,
    tile_windows: Option<Vec<GMFormatTileWindow>>,
    borders: Option<Vec<GMFormatBorder>>,
}

#[derive(Clone, Debug, Default, DeJson)]
pub struct GMFormatFont{
    name: String,
    image_file: String,
    char_width: f32,
    char_height: f32,
    char_order: String,
}

#[derive(Clone, Debug, Default, DeJson)]
pub struct GMFormatSpriteSheet {
    name: String,
    file: String,
}

#[derive(Clone, Debug, Default, DeJson)]
pub struct GMFormatSprite {
    name: String,
    sprite_sheet: String,
    animation: String,
}

#[derive(Clone, Debug, Default, DeJson)]
pub struct GMFormatFrame {
    x: f32,
    y: f32,
    w: f32,
    h: f32,
    duration: f64,
}

#[derive(Clone, Debug, DeJson)]
pub enum GMFormatAnimationType {
    ForwardOnce,
    ForwardLoop,
    BackwardOnce,
    BackwardLoop,
    PingPong,
}

#[derive(Clone, Debug, DeJson)]
pub struct GMFormatAnimation {
    name: String,
    animation_type: GMFormatAnimationType,
    frames: Vec<GMFormatFrame>,
}

#[derive(Clone, Debug, DeJson)]
pub struct GMFormatAnimationMultiple {
    animations: Vec<GMFormatAnimation>,
}

#[derive(Clone, Debug, Default, DeJson)]
pub struct GMFormatSound {
    name: String,
    file: String,
}

#[derive(Clone, Debug, Default, DeJson)]
pub struct GMFormatTileSet {
    name: String,
    file: String,
    tile_width: f32,
    tile_height: f32,
    mapping: HashMap<u32, (f32, f32)>,
}

#[derive(Clone, Debug, Default, DeJson)]
pub struct GMFormatTileMap {
    name: String,
    tileset: String,
    map: Vec<u32>,
    width: usize,
    height: usize,
    range_mapping: Vec<(u32, u32, String)>,
}

#[derive(Clone, Debug, Default, DeJson)]
pub struct GMFormatTileWindow {
    name: String,
    tilemap: String,
    screen_x: f32,
    screen_y: f32,
    window_width: f32,
    window_height: f32,
}

#[derive(Clone, Debug, Default, DeJson)]
pub struct GMFormatBorder {

}

pub struct GMResourceManager {
    fonts: HashMap<String, GMFont>,
    sprite_sheets: HashMap<String, Rc<GMSpriteSheet>>,
    sprites: HashMap<String, GMSprite>,
    sounds: HashMap<String, Rc<GMSound>>,
    animations: HashMap<String, GMAnimation>,
    tileset: HashMap<String, Rc<GMTileSet>>,
    tilemap: HashMap<String, GMTileMap>,
    tile_window: HashMap<String, GMTileWindow>,
}

impl GMResourceManager {
    pub fn new() -> Self {
        Self {
            fonts: HashMap::new(),
            sprite_sheets: HashMap::new(),
            sprites: HashMap::new(),
            sounds: HashMap::new(),
            animations: HashMap::new(),
            tileset: HashMap::new(),
            tilemap: HashMap::new(),
            tile_window: HashMap::new(),
        }
    }
    pub async fn new_from_file(file_name: &str) -> Result<Self, GMError> {
        info!("Loading resource file: '{}'", file_name);
        let json = load_string(file_name).await?;
        let result: GMFormatResource = DeJson::deserialize_json(&json)?;
        let mut resource = Self::new();

        if let Some(font_files) = result.font_files {
            for file_name in font_files.iter() {
                resource.fonts_from_file(&file_name).await?;
            }
        }
        if let Some(sprite_sheets) = result.sprite_sheets {
            for item in sprite_sheets.into_iter() {
                debug!("SpriteSheet name: '{}'", item.name);

                let sprite_sheet = GMSpriteSheet::new_rc(&item.file).await?;
                resource.sprite_sheets.insert(item.name, sprite_sheet);
            }
        }
        if let Some(sounds) = result.sounds {
            for item in sounds.into_iter() {
                debug!("Sound name: '{}'", item.name);

                let sound = GMSound::new_rc(&item.file).await?;
                resource.sounds.insert(item.name, sound);
            }
        }
        if let Some(animation_files) = result.animation_files {
            for file_name in animation_files.iter() {
                resource.animations_from_file(file_name).await?;
            }
        }
        if let Some(sprites) = result.sprites {
            for item in sprites.into_iter() {
                debug!("Sprite name: '{}', sprite sheet: '{}', animation: '{}'", item.name,
                    item.sprite_sheet, item.animation);

                let sprite = GMSpriteSingle::new_wrapped(
                    &resource.get_sprite_sheet(&item.sprite_sheet).unwrap(),
                    &resource.get_animation(&item.animation).unwrap(), 0.0, 0.0);
                resource.sprites.insert(item.name, sprite);
            }
        }
        if let Some(tileset_files) = result.tileset_files {
            for file_name in tileset_files.iter() {
                resource.tileset_from_file(file_name).await?;
            }
        }
        if let Some(tilemap_files) = result.tilemap_files {
            for file_name in tilemap_files.iter() {
                resource.tilemap_from_file(file_name).await?;
            }            
        }
        if let Some(tile_windows) = result.tile_windows {
            for item in tile_windows.into_iter() {
                debug!("Tile window name: '{}', map name: '{}'", item.name, item.tilemap);

                let tilemap = resource.get_tilemap(&item.tilemap).unwrap();
                let tile_window = GMTileWindow::new(tilemap, item.screen_x,
                    item.screen_y, item.window_width, item.window_height);
                resource.tile_window.insert(item.name, tile_window);
            }            
        }

        Ok(resource)
    }
    pub fn add_font(&mut self, name: &str, font: &GMFont) {
        self.fonts.insert(name.to_string(), font.clone());
    }
    pub async fn fonts_from_file(&mut self, file_name: &str) -> Result<(), GMError>{
        info!("Loading font file: '{}'", file_name);
        let json = load_string(file_name).await?;
        let item: GMFormatFont = DeJson::deserialize_json(&json)?;

        debug!("Processing font...");
        debug!("Font name: '{}', width: {}, height: {}", item.name, item.char_width, item.char_height);

        let p1 = Path::new(file_name);
        let parent = p1.parent().unwrap();
        let p2 = Path::new(&item.image_file);
        let new_path = parent.join(p2);
        let os_str = new_path.into_os_string();
        let img_file_name = os_str.into_string().unwrap();

        debug!("Font image file: '{}'", img_file_name);

        let font = GMBitmapFont::new_font(&img_file_name,
            item.char_width, item.char_height, &item.char_order).await?;

        self.fonts.insert(item.name, font);

        Ok(())
    }
    pub fn get_font(&self, name: &str) -> Option<GMFont> {
        self.fonts.get(name).map(|v| v.clone())
    }
    pub fn remove_font(&mut self, name: &str) -> Option<GMFont> {
        self.fonts.remove(name)
    }
    pub fn clear_fonts(&mut self) {
        self.fonts.clear();
    }
    pub fn add_sprite_sheet(&mut self, name: &str, sprite_sheet: GMSpriteSheet) {
        self.sprite_sheets.insert(name.to_string(), Rc::new(sprite_sheet));
    }
    pub fn get_sprite_sheet(&self, name: &str) -> Option<Rc<GMSpriteSheet>> {
        self.sprite_sheets.get(name).map(|v| v.clone())
    }
    pub fn remove_sprite_sheet(&mut self, name: &str) -> Option<Rc<GMSpriteSheet>> {
        self.sprite_sheets.remove(name)
    }
    pub fn clear_sprite_sheets(&mut self) {
        self.sprite_sheets.clear();
    }
    pub fn add_sprite(&mut self, name: &str, sprite: &GMSprite) {
        self.sprites.insert(name.to_string(), sprite.clone());
    }
    pub fn get_sprite(&self, name: &str) -> Option<&GMSprite> {
        self.sprites.get(name)
    }
    pub fn get_sprite_simple(&self, name: &str) -> Option<GMSpriteSimple> {
        if let Some(sprite) = self.sprites.get(name) {
            Some(sprite.to_simple())
        } else {
            None
        }
    }
    pub fn remove_sprite(&mut self, name: &str) -> Option<GMSprite> {
        self.sprites.remove(name)
    }
    pub fn clear_sprites(&mut self) {
        self.sprites.clear();
    }
    pub fn add_animation(&mut self, name: &str, animation: GMAnimation) {
        self.animations.insert(name.to_string(), animation);
    }
    pub async fn animations_from_file(&mut self, file_name: &str) -> Result<(), GMError> {
        info!("Loading animation file: '{}'", file_name);
        let json = load_string(file_name).await?;
        let result: GMFormatAnimationMultiple = DeJson::deserialize_json(&json)?;

        for item in result.animations.into_iter() {
            debug!("Processing animations...");
            debug!("Animation name: '{}', type: {:?}", item.name, item.animation_type);

            let frames: Vec<(Rect, f64)> = item.frames.iter().map(|f| (Rect::new(f.x, f.y, f.w, f.h), f.duration)).collect();

            use GMFormatAnimationType::*;

            let animation = match item.animation_type {
                ForwardOnce => {
                    GMAnimationForwardOnce::new_anim(&frames)
                }
                ForwardLoop => {
                    GMAnimationForwardLoop::new_anim(&frames)
                }
                BackwardOnce => {
                    GMAnimationBackwardOnce::new_anim(&frames)
                }
                BackwardLoop => {
                    GMAnimationBackwardLoop::new_anim(&frames)
                }
                PingPong => {
                    GMAnimationPingPong::new_anim(&frames)
                }
            };

            self.animations.insert(item.name, animation);
        }

        Ok(())
    }
    pub fn get_animation(&self, name: &str) -> Option<GMAnimation> {
        self.animations.get(name).map(|v| v.clone())
    }
    pub fn remove_animation(&mut self, name: &str) -> Option<GMAnimation> {
        self.animations.remove(name)
    }
    pub fn clear_animations(&mut self) {
        self.animations.clear();
    }
    pub fn add_sound(&mut self, name: &str, sound: GMSound) {
        self.sounds.insert(name.to_string(), Rc::new(sound));
    }
    pub fn get_sound(&self, name: &str) -> Option<Rc<GMSound>> {
        self.sounds.get(name).map(|v| v.clone())
    }
    pub fn remove_sound(&mut self, name: &str) -> Option<Rc<GMSound>> {
        self.sounds.remove(name)
    }
    pub fn clear_sounds(&mut self) {
        self.sounds.clear();
    }
    pub fn add_tileset(&mut self, name: &str, tileset: GMTileSet) {
        self.tileset.insert(name.to_string(), Rc::new(tileset));
    }
    pub async fn tileset_from_file(&mut self, file_name: &str) -> Result<(), GMError> {
        info!("Loading tileset file: '{}'", file_name);
        let json = load_string(file_name).await?;
        let result: GMFormatTileSet = DeJson::deserialize_json(&json)?;
        let tileset = GMTileSet::new(&result.file, result.tile_width, result.tile_height, &result.mapping).await?;
        self.tileset.insert(result.name, Rc::new(tileset));
        Ok(())
    }
    pub fn get_tileset(&self, name: &str) -> Option<Rc<GMTileSet>>{
        self.tileset.get(name).map(|v| v.clone())
    }
    pub fn remove_tileset(&mut self, name: &str) -> Option<Rc<GMTileSet>> {
        self.tileset.remove(name)
    }
    pub fn clear_tileset(&mut self) {
        self.tileset.clear();
    }
    pub fn add_tilemap(&mut self, name: &str, tilemap: GMTileMap) {
        self.tilemap.insert(name.to_string(), tilemap);
    }
    pub async fn tilemap_from_file(&mut self, file_name: &str) -> Result<(), GMError> {
        info!("Loading tilemap file: '{}'", file_name);
        let json = load_string(file_name).await?;
        let result: GMFormatTileMap = DeJson::deserialize_json(&json)?;
        let tileset = self.get_tileset(&result.tileset).unwrap();
        let tilemap = GMTileMap::new(tileset, result.width, result.height, &result.map);
        self.tilemap.insert(result.name, tilemap);
        Ok(())
    }
    pub fn get_tilemap(&self, name: &str) -> Option<&GMTileMap>{
        self.tilemap.get(name)
    }
    pub fn remove_tilemap(&mut self, name: &str) -> Option<GMTileMap> {
        self.tilemap.remove(name)
    }
    pub fn clear_tilemap(&mut self) {
        self.tilemap.clear();
    }
    pub fn add_tile_window(&mut self, name: &str, tile_window: GMTileWindow) {
        self.tile_window.insert(name.to_string(), tile_window);
    }
    pub fn get_tile_window(&self, name: &str) -> Option<&GMTileWindow> {
        self.tile_window.get(name)
    }
    pub fn remove_tile_window(&mut self, name: &str) {
        self.tile_window.remove(name);
    }
    pub fn clear_tile_window(&mut self) {
        self.tile_window.clear();
    }
    pub fn clear_all(&mut self) {
        self.clear_fonts();
        self.clear_sprite_sheets();
        self.clear_sprites();
        self.clear_animations();
        self.clear_sounds();
    }
}
