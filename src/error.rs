use thiserror::Error;
use macroquad::file::FileError;
/// This data structure contains all error codes for the GreenMoon2D crate.
#[derive(Error, Debug)]
pub enum GMError {
    #[error("IO error while loading file")]
    FileError(#[from] FileError),
    #[error("Error while parsing font config")]
    ParseFont(String),
}
