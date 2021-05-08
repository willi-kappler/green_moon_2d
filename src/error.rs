use thiserror::Error;

/// This data structure contains all error codes for the GreenMoon2D crate.
#[derive(Error, Debug)]
pub enum GMError {
    #[error("The scene name is already used: {0}")]
    SceneNameAlreadyInUse(String),
    #[error("The scene {0} was not found")]
    SceneNameNotFound(String),
}
