extern crate failure;

use http::header;
use std::fmt::Debug;
use url::Url;

pub type Result<T> = ::std::result::Result<T, Error>;

#[derive(Fail, Debug)]
pub enum Error {
    #[fail(display = "env_extractor::Error > {}", 0)]
    EnvExtractorError(String),

    #[fail(display = "EnvVarNotPresent > {}", 0)]
    EnvVarNotPresent(String),

    #[fail(display = "HostNotFound > {}", 0)]
    HostNotFound(Url),

    #[fail(display = "InvalidHeaderName > {}", 0)]
    InvalidHeaderName(header::InvalidHeaderName),

    #[fail(display = "InvalidHeaderValue > {}", 0)]
    InvalidHeaderValue(header::InvalidHeaderValue),

    #[fail(display = "std::io::Error > {}", 0)]
    StdIoError(std::io::Error),
}

impl<A: Debug> From<env_extractor::Error<A>> for Error {
    fn from(e: env_extractor::Error<A>) -> Self {
        match e {
            env_extractor::Error::NotPresent(key) => Error::EnvVarNotPresent(key),
            _ => Error::EnvExtractorError(format!("{:?}", e)),
        }
    }
}

impl From<header::InvalidHeaderName> for Error {
    fn from(e: header::InvalidHeaderName) -> Self {
        Error::InvalidHeaderName(e)
    }
}

impl From<header::InvalidHeaderValue> for Error {
    fn from(e: header::InvalidHeaderValue) -> Self {
        Error::InvalidHeaderValue(e)
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::StdIoError(e)
    }
}
