
/*
use serde_derive::Deserialize;

#[derive(Clone, Deserialize)]
pub struct GMAssetTexture {
    pub name: String,
    pub file: String,
    pub rows: u32,
    pub cols: u32,
}

#[derive(Clone, Deserialize)]
pub struct GMAssetAnimation {
    pub name: String,
    pub frames: Vec<(usize, f32)>, // (index, duration)
    pub animation_type: u8 // TODO
}

#[derive(Clone, Deserialize)]
pub struct GMAssetFont {
    pub name: String,
    pub texture: String,
    pub mapping: String,
}

#[derive(Clone, Deserialize)]
pub struct GMAssetText {
    pub name: String,
    pub text: String,
    pub font: String,
    pub x: u32,
    pub y: u32,
}

#[derive(Clone, Deserialize)]
pub struct GMAssetSpriteLine {
    pub name: String,
    pub texture: String,
    pub animation: String,
    pub x1: u32,
    pub y1: u32,
    pub x2: u32,
    pub y2: u32,
}

#[derive(Clone, Deserialize)]
pub struct GMAssetSpriteBorder {
    pub name: String,
    pub texture: String,
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
    // TODO: border type
}

#[derive(Clone, Deserialize)]
pub struct GMAssetSprite {
    pub name: String,
    pub texture: String,
    pub animation: String,
    pub x: u32,
    pub y: u32,
    pub custom_id: u32,
}

#[derive(Clone, Deserialize)]
pub struct GMAssetParticleEmitter {
    pub name: String,
    pub texture: String,
    pub animation: String,
    pub x: u32,
    pub y: u32,
    // TODO: particle speed range, rotation range, angle, ...
}

#[derive(Clone, Deserialize)]
pub struct GMAssetBulletEmitter {
    pub name: String,
    pub texture: String,
    pub animation: String,
    pub x: u32,
    pub y: u32,
    // TODO: bullet type, direction, speed, ...
}

#[derive(Clone, Deserialize)]
pub struct GMAssetTileMap {
    pub name: String,
    pub texture: String,
    pub num_tiles_x: u32,
    pub num_tiles_y: u32,
    pub x: u32,
    pub y: u32,
    pub screen_width: u32,
    pub screen_height: u32,
    pub data: Vec<u32>,
}

#[derive(Clone, Deserialize)]
pub struct GMAssets {
    #[serde(default)]
    pub textures: Vec<GMAssetTexture>,

    #[serde(default)]
    pub animations: Vec<GMAssetAnimation>,

    #[serde(default)]
    pub fonts: Vec<GMAssetFont>,
}
*/
