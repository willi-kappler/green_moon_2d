use thiserror::Error;

use macroquad::file::FileError;
use nanoserde::DeJsonErr;

/// This data structure contains all error codes for the GreenMoon2D crate.
#[derive(Error, Debug)]
pub enum GMError {
    #[error("IO error while loading file")]
    FileError(#[from] FileError),
    #[error("Could not deserialize JSON string")]
    JSONError(#[from] DeJsonErr)
}
