extern crate failure;

use crate::operations;

pub type Result<T> = ::std::result::Result<T, Error>;

#[derive(Fail, Debug)]
pub enum Error {
    #[fail(display = "operations::get_object::Error > {}", 0)]
    GetObjectError(operations::get_object::Error),

    #[fail(display = "operations::put_object::Error > {}", 0)]
    PutObjectError(operations::put_object::Error),

    #[fail(display = "operations::S3ClientError > {}", 0)]
    S3ClientError(operations::S3ClientError),

    #[fail(display = "std::io::Error > {}", 0)]
    StdIoError(std::io::Error),

    #[fail(display = "url::ParseError > {}", 0)]
    UrlParseError(url::ParseError),
}

impl From<operations::get_object::Error> for Error {
    fn from(e: operations::get_object::Error) -> Self {
        Error::GetObjectError(e)
    }
}

impl From<operations::put_object::Error> for Error {
    fn from(e: operations::put_object::Error) -> Self {
        Error::PutObjectError(e)
    }
}

impl From<operations::S3ClientError> for Error {
    fn from(e: operations::S3ClientError) -> Self {
        Error::S3ClientError(e)
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::StdIoError(e)
    }
}

impl From<url::ParseError> for Error {
    fn from(e: url::ParseError) -> Self {
        Error::UrlParseError(e)
    }
}
