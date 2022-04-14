
use std::io;
use std::fmt;

use serde_json;

#[derive(Debug)]
pub enum GMError {
    SceneNotFound(String),
    SceneAlreadyExists(String),
    IO(io::Error),
    JSON(serde_json::Error),
    AnimationNotFound(String),
    AnimationAlreadyExists(String),
    DrawObjectNotFound(String),
    DrawObjectAlreadyExists(String),
    FontNotFound(String),
    FontAlreadyExists(String),
    SpriteAlreadyExists(String),
    TextAlreadyExists(String),
}

impl std::error::Error for GMError {

}

impl fmt::Display for GMError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            GMError::SceneNotFound(name) => write!(f, "Scene not found: {}", name),
            GMError::SceneAlreadyExists(name) => write!(f, "Scene already exists: '{}'", name),
            GMError::IO(e) => write!(f, "Could not open file: '{}'", e),
            GMError::JSON(e) => write!(f, "Could not parse JSON: '{}'", e),
            GMError::DrawObjectAlreadyExists(name) => write!(f, "Draw object already exists: '{}'", name),
            GMError::AnimationNotFound(name) => write!(f, "Animation not found: '{}'", name),
            GMError::AnimationAlreadyExists(name) => write!(f, "Animation already exists: '{}'", name),
            GMError::DrawObjectNotFound(name) => write!(f, "Draw object not found: '{}'", name),
            GMError::FontNotFound(name) => write!(f, "Font not found: '{}'", name),
            GMError::FontAlreadyExists(name) => write!(f, "Font already exists: '{}'", name),
            GMError::SpriteAlreadyExists(name) => write!(f, "Sprite already exists: '{}'", name),
            GMError::TextAlreadyExists(name) => write!(f, "Text already exists: '{}'", name),
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
