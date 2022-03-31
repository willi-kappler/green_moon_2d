// #![warn(clippy::pedantic)]

pub mod animation;
pub mod app;
pub mod assets;
pub mod configuration;
pub mod context;
pub mod error;
pub mod font;
pub mod movement;
pub mod scene;
pub mod scene_container;
pub mod sprite;
pub mod text;
pub mod texture;

pub use app::GMApp;
pub use context::GMContext;
pub use scene::GMSceneT;
pub use error::GMError;
