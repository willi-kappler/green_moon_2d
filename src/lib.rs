
pub mod animation;
pub mod collision;
pub mod configuration;
pub mod context;
pub mod engine;
pub mod error;
pub mod font;
pub mod input;
pub mod math;
pub mod object;
pub mod resources;
pub mod scene;
pub mod sprite;
pub mod text;
pub mod texture;
pub mod timer;

pub use context::{GMUpdateContext, GMDrawContext};
pub use engine::GMEngine;
pub use error::GMError;
pub use scene::GMSceneT;

// Export from SDL2
pub use sdl2::pixels::Color;
