
use std::io;
use std::fmt;

use serde_json;

#[derive(Debug)]
pub enum GMError {
    SceneNotFound(String),
    SceneAlreadyExists(String),
    SceneStackEmpty,
    CantRemoveCurrentScene(String),
    IO(io::Error),
    JSON(serde_json::Error),
    AnimationNotFound(String),
    AnimationAlreadyExists(String),
    FontNotFound(String),
    FontAlreadyExists(String),
}

impl std::error::Error for GMError {

}

impl fmt::Display for GMError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            GMError::SceneNotFound(name) => write!(f, "Scene not found: {}", name),
            GMError::SceneAlreadyExists(name) => write!(f, "Scene already exists: '{}'", name),
            GMError::SceneStackEmpty => write!(f, "The scene stack is empty"),
            GMError::CantRemoveCurrentScene(name) => write!(f, "Can't remove current scene: '{}'", name),
            GMError::IO(e) => write!(f, "Could not open file: '{}'", e),
            GMError::JSON(e) => write!(f, "Could not parse JSON: '{}'", e),
            GMError::AnimationNotFound(name) => write!(f, "Animation not found: '{}'", name),
            GMError::AnimationAlreadyExists(name) => write!(f, "Animation already exists: '{}'", name),
            GMError::FontNotFound(name) => write!(f, "Font not found: '{}'", name),
            GMError::FontAlreadyExists(name) => write!(f, "Font already exists: '{}'", name),
        }
    }
}

impl From<io::Error> for GMError {
    fn from(e: io::Error) -> Self {
     GMError::IO(e)
    }
}

impl From<serde_json::Error> for GMError {
    fn from(e: serde_json::Error) -> Self {
      GMError::JSON(e)
    }
}
