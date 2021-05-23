use thiserror::Error;
use macroquad::file::FileError;
/// This data structure contains all error codes for the GreenMoon2D crate.
#[derive(Error, Debug)]
pub enum GMError {
    #[error("The item name is already used: {0} {1}")]
    ItemNameAlreadyInUse(String, String),
    #[error("The item was not found: {0} {1}")]
    ItemNotFound(String, String),
    #[error("No item with prefix found: {0} {1}")]
    ItemPrefixNotFound(String, String),
    #[error("IO error while loading file")]
    FileError(#[from] FileError),
    #[error("Error while parsing font config")]
    ParseFont(String),
}
