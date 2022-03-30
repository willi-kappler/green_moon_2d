// #![warn(clippy::pedantic)]

pub mod animation;
pub mod app;
pub mod assets;
pub mod configuration;
pub mod context;
pub mod error;
pub mod scene_container;
pub mod scene;

pub use app::GMApp;
pub use context::GMContext;
pub use scene::GMSceneT;
pub use error::GMError;
