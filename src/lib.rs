
pub mod animation;
pub mod animation_effect;
pub mod bitmap_text;
pub mod bitmap_text_effects;
pub mod collision;
pub mod configuration;
pub mod context;
pub mod data;
pub mod engine;
pub mod input;
pub mod math;
pub mod resources;
pub mod scene;
pub mod sprite;
pub mod sprite_effect;
pub mod texture;
pub mod timer;
pub mod util;

pub use context::GMContext;
pub use engine::GMEngine;
pub use input::GMEventCode;
pub use resources::GMResources;
pub use scene::GMSceneT;

// Export from SDL2
pub use sdl2::pixels::Color;
