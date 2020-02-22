use crate::core::headers;
use failure::_core::fmt::Debug;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Fail, Debug)]
pub enum Error {
    #[fail(display = "env_extractor::Error > {}", 0)]
    EnvExtractorError(String),

    #[fail(display = "headers::Error > {}", 0)]
    S3HeaderError(headers::Error),
}

impl<A: Debug> From<env_extractor::Error<A>> for Error {
    fn from(e: env_extractor::Error<A>) -> Self {
        Error::EnvExtractorError(format!("{:?}", e))
    }
}

impl From<headers::Error> for Error {
    fn from(e: headers::Error) -> Self {
        Error::S3HeaderError(e)
    }
}
