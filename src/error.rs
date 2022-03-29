
use std::io;
use std::fmt;

use serde_json;


#[derive(Debug)]
pub enum GMError {
    SceneNotFound(String),
    IO(io::Error),
    JSON(serde_json::Error),
}

impl std::error::Error for GMError {

}

impl fmt::Display for GMError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      match self {
            GMError::SceneNotFound(name) => write!(f, "Scene not found: {}", name),
            GMError::IO(e) => write!(f, "Could not open file: '{}'", e),
            GMError::JSON(e) => write!(f, "Could not parse JSON: '{}'", e),
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
