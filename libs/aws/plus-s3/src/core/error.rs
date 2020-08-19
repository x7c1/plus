use crate::core::response::headers;
use failure::_core::fmt::Debug;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Fail, Debug)]
pub enum Error {
    #[fail(display = "env_extractor::Error > {}", 0)]
    EnvExtractorError(String),

    #[fail(
        display = "FileNotFound > path: {}, description: {}",
        path, description
    )]
    FileNotFound { path: String, description: String },

    #[fail(display = "plus_aws::Error > {}", 0)]
    PlusAwsError(plus_aws::Error),

    #[fail(display = "headers::Error > {}", 0)]
    S3HeaderError(headers::Error),

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

impl From<plus_aws::Error> for Error {
    fn from(e: plus_aws::Error) -> Self {
        Error::PlusAwsError(e)
    }
}

impl From<headers::Error> for Error {
    fn from(e: headers::Error) -> Self {
        Error::S3HeaderError(e)
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
