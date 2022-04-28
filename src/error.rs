
use std::io;
use std::fmt;

use nanoserde::DeJsonErr;

#[derive(Debug)]
pub enum GMError {
    EngineNotInitialized,
    SceneNotFound(String),
    SceneAlreadyExists(String),
    SceneStackEmpty,
    CantRemoveCurrentScene(String),
    IO(io::Error),
    JSON(DeJsonErr),
    AnimationNotFound(String),
    AnimationAlreadyExists(String),
    FontNotFound(String),
    FontAlreadyExists(String),
    CouldNotLoadTexture(String),
    TextureNotFound(String),
    TextureAlreadyExists(String),
}

impl std::error::Error for GMError {

}

impl fmt::Display for GMError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use GMError::*;

        match self {
            EngineNotInitialized => write!(f, "The engine was not initialized, please call the init() method before the run() method."),
            SceneNotFound(name) => write!(f, "Scene not found: {}", name),
            SceneAlreadyExists(name) => write!(f, "Scene already exists: '{}'", name),
            SceneStackEmpty => write!(f, "The scene stack is empty"),
            CantRemoveCurrentScene(name) => write!(f, "Can't remove current scene: '{}'", name),
            IO(e) => write!(f, "Could not open file: '{}'", e),
            JSON(e) => write!(f, "Could not parse JSON: '{}'", e),
            AnimationNotFound(name) => write!(f, "Animation not found: '{}'", name),
            AnimationAlreadyExists(name) => write!(f, "Animation already exists: '{}'", name),
            FontNotFound(name) => write!(f, "Font not found: '{}'", name),
            FontAlreadyExists(name) => write!(f, "Font already exists: '{}'", name),
            CouldNotLoadTexture(name) => write!(f, "Texture could not be loaded: '{}'", name),
            TextureNotFound(name) => write!(f, "Texture not found: '{}'", name),
            TextureAlreadyExists(name) => write!(f, "Texture already exists: '{}'", name),
        }
    }
}

impl From<io::Error> for GMError {
    fn from(e: io::Error) -> Self {
     GMError::IO(e)
    }
}

impl From<DeJsonErr> for GMError {
    fn from(e: DeJsonErr) -> Self {
      GMError::JSON(e)
    }
}
