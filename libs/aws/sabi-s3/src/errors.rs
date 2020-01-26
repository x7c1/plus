extern crate failure;

pub type Result<T> = ::std::result::Result<T, Error>;

#[derive(Fail, Debug)]
pub enum Error {
    #[fail(display = "std::string::FromUtf8Error > {}", 0)]
    FromUtf8Error(std::string::FromUtf8Error),

    #[fail(display = "http::Error > {}", 0)]
    HttpError(http::Error),

    #[fail(display = "hyper::Error > {}", 0)]
    HyperError(hyper::Error),

    #[fail(display = "http::uri::InvalidUri > {}", 0)]
    InvalidUri(http::uri::InvalidUri),

    #[fail(display = "std::io::Error > {}", 0)]
    StdIoError(std::io::Error),
}

impl From<std::string::FromUtf8Error> for Error {
    fn from(e: std::string::FromUtf8Error) -> Self {
        Error::FromUtf8Error(e)
    }
}

impl From<http::Error> for Error {
    fn from(e: http::Error) -> Self {
        Error::HttpError(e)
    }
}

impl From<hyper::Error> for Error {
    fn from(e: hyper::Error) -> Self {
        Error::HyperError(e)
    }
}

impl From<http::uri::InvalidUri> for Error {
    fn from(e: http::uri::InvalidUri) -> Self {
        Error::InvalidUri(e)
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::StdIoError(e)
    }
}
