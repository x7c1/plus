extern crate failure;

use std::fmt::Debug;
use std::path::PathBuf;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Fail, Debug)]
pub enum Error {
    #[fail(display = "serde_json::Error > {}", 0)]
    SerdeJsonError(serde_json::Error),

    #[fail(display = "std::io::Error > message: {}, cause: {}", message, cause)]
    StdIoError {
        cause: std::io::Error,
        message: String,
    },

    #[fail(display = "workspace invalid > path: {:?}, cause: {}", 0, 1)]
    InvalidWorkspace(PathBuf, std::io::Error),
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        Error::SerdeJsonError(e)
    }
}
