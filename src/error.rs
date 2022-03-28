
use std::io;
use std::fmt;

use serde_json;


#[derive(Debug)]
pub enum GMError {
    SceneNotFound(String),
    ConfigFile(io::Error),
    ConfigJSON(serde_json::Error),
}

impl std::error::Error for GMError {

}

impl fmt::Display for GMError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      match self {
            GMError::SceneNotFound(name) => write!(f, "Scene not found: {}", name),
            GMError::ConfigFile(e) => write!(f, "Could not open configuration file: '{}'", e),
            GMError::ConfigJSON(e) => write!(f, "Could not parse configuration JSON: '{}'", e),
        }
    }
}

impl From<io::Error> for GMError {
    fn from(e: io::Error) -> Self {
     GMError::ConfigFile(e)
    }
}

impl From<serde_json::Error> for GMError {
    fn from(e: serde_json::Error) -> Self {
      GMError::ConfigJSON(e)
    }
}
