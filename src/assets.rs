
use serde_derive::Deserialize;

#[derive(Clone, Deserialize)]
pub(crate) struct GMAssetTexture {
    pub(crate) name: String,
    pub(crate) file: String,
    pub(crate) rows: u32,
    pub(crate) cols: u32,
}

#[derive(Clone, Deserialize)]
pub(crate) struct GMAssetAnimation {
    pub(crate) name: String,
    pub(crate) frames: Vec<(usize, f32)>, // (index, duration)
    pub(crate) animation_type: u8 // TODO
}

#[derive(Clone, Deserialize)]
pub(crate) struct GMAssetFont {
    pub(crate) name: String,
    pub(crate) texture: String,
    pub(crate) mapping: String,
}

#[derive(Clone, Deserialize)]
pub(crate) struct GMAssetText {
    pub(crate) name: String,
    pub(crate) text: String,
    pub(crate) font: String,
    pub(crate) x: u32,
    pub(crate) y: u32,
}

#[derive(Clone, Deserialize)]
pub(crate) struct GMAssetSpriteLine {
    pub(crate) name: String,
    pub(crate) texture: String,
    pub(crate) animation: String,
    pub(crate) x1: u32,
    pub(crate) y1: u32,
    pub(crate) x2: u32,
    pub(crate) y2: u32,
}

#[derive(Clone, Deserialize)]
pub(crate) struct GMAssetSpriteBorder {
    pub(crate) name: String,
    pub(crate) texture: String,
    pub(crate) x: u32,
    pub(crate) y: u32,
    pub(crate) width: u32,
    pub(crate) height: u32,
    // TODO: border type
}

#[derive(Clone, Deserialize)]
pub(crate) struct GMAssetSprite {
    pub(crate) name: String,
    pub(crate) texture: String,
    pub(crate) animation: String,
    pub(crate) x: u32,
    pub(crate) y: u32,
    pub(crate) custom_id: u32,
}

#[derive(Clone, Deserialize)]
pub(crate) struct GMAssetParticleEmitter {
    pub(crate) name: String,
    pub(crate) texture: String,
    pub(crate) animation: String,
    pub(crate) x: u32,
    pub(crate) y: u32,
    // TODO: particle speed range, rotation range, angle, ...
}

#[derive(Clone, Deserialize)]
pub(crate) struct GMAssetBulletEmitter {
    pub(crate) name: String,
    pub(crate) texture: String,
    pub(crate) animation: String,
    pub(crate) x: u32,
    pub(crate) y: u32,
    // TODO: bullet type, direction, speed, ...
}

#[derive(Clone, Deserialize)]
pub(crate) struct GMAssetTileMap {
    pub(crate) name: String,
    pub(crate) texture: String,
    pub(crate) num_tiles_x: u32,
    pub(crate) num_tiles_y: u32,
    pub(crate) x: u32,
    pub(crate) y: u32,
    pub(crate) screen_width: u32,
    pub(crate) screen_height: u32,
    pub(crate) data: Vec<u32>,
}

#[derive(Clone, Deserialize)]
pub(crate) struct GMAssets {
    #[serde(default)]
    pub(crate) textures: Vec<GMAssetTexture>,

    #[serde(default)]
    pub(crate) animations: Vec<GMAssetAnimation>,

    #[serde(default)]
    pub(crate) fonts: Vec<GMAssetFont>,
}
