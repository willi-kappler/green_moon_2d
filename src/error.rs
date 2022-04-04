
use std::io;
use std::fmt;

use serde_json;


#[derive(Debug)]
pub enum GMError {
    SceneNotFound(String),
    SceneAlreadyExists(String),
    IO(io::Error),
    JSON(serde_json::Error),
    ObjectAlreadyExists(String),
    DrawObjectNotFound(String),
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
            GMError::ObjectAlreadyExists(name) => write!(f, "Draw object already exists: '{}'", name),
            GMError::DrawObjectNotFound(name) => write!(f, "Draw object not found: '{}'", name),
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
