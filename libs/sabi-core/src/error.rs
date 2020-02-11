extern crate failure;

use http::header;
use url::Url;

pub type Result<T> = ::std::result::Result<T, Error>;

#[derive(Fail, Debug)]
pub enum Error {
    #[fail(display = "HostNotFound > {}", 0)]
    HostNotFound(Url),

    #[fail(display = "InvalidHeaderName > {}", 0)]
    InvalidHeaderName(header::InvalidHeaderName),

    #[fail(display = "InvalidHeaderValue > {}", 0)]
    InvalidHeaderValue(header::InvalidHeaderValue),

    #[fail(display = "std::io::Error > {}", 0)]
    StdIoError(std::io::Error),
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
