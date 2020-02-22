extern crate failure;

use crate::core::headers;
use crate::{internal, operations};
use std::fmt::Debug;

pub type Result<T> = ::std::result::Result<T, Error>;

#[derive(Fail, Debug)]
pub enum Error {
    #[fail(display = "env_extractor::Error > {}", 0)]
    EnvExtractorError(String),

    #[fail(display = "EnvVarNotPresent > {}", 0)]
    EnvVarNotPresent(String),

    #[fail(display = "FileNotFound > {}", 0)]
    FileNotFound {
        operation: operations::Kind,
        path: String,
        description: String,
    },

    #[fail(display = "operations::get_object::Error > {}", 0)]
    GetObjectError(operations::get_object::Error),

    #[fail(display = "internal::Error > {}", 0)]
    InternalError(internal::Error),

    #[fail(display = "region not specified")]
    RegionNotSpecified,

    #[fail(display = "headers::Error > {}", 0)]
    S3HeaderError(headers::Error),

    #[fail(display = "sabi_core::Error > {}", 0)]
    SabiCoreError(sabi_core::Error),

    #[fail(display = "std::io::Error > {}", 0)]
    StdIoError(std::io::Error),

    #[fail(display = "url::ParseError > {}", 0)]
    UrlParseError(url::ParseError),
}

impl<A: Debug> From<env_extractor::Error<A>> for Error {
    fn from(e: env_extractor::Error<A>) -> Self {
        Error::EnvExtractorError(format!("{:?}", e))
    }
}

impl From<operations::get_object::Error> for Error {
    fn from(e: operations::get_object::Error) -> Self {
        Error::GetObjectError(e)
    }
}

impl From<internal::Error> for Error {
    fn from(e: internal::Error) -> Self {
        Error::InternalError(e)
    }
}

impl From<headers::Error> for Error {
    fn from(e: headers::Error) -> Self {
        Error::S3HeaderError(e)
    }
}

impl From<sabi_core::Error> for Error {
    fn from(e: sabi_core::Error) -> Self {
        match e {
            sabi_core::Error::EnvVarNotPresent(key) => Error::EnvVarNotPresent(key),
            _ => Error::SabiCoreError(e),
        }
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
