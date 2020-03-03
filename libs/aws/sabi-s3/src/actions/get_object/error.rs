use crate::internal;

#[derive(Fail, Debug)]
pub enum Error {
    #[fail(display = "internal::Error > {}", 0)]
    InternalError(internal::Error),

    #[fail(display = "file::OutfileError > {}", 0)]
    OutFileError(super::request::OutfileError),

    #[fail(display = "reqwest::Error > {}", 0)]
    ReqwestError(reqwest::Error),

    #[fail(display = "crate::core::Error > {}", 0)]
    S3CoreError(crate::core::Error),

    #[fail(display = "std::io::Error > {}", 0)]
    StdIoError(std::io::Error),
}

impl From<internal::Error> for Error {
    fn from(e: internal::Error) -> Self {
        Error::InternalError(e)
    }
}

impl From<super::request::OutfileError> for Error {
    fn from(e: super::request::OutfileError) -> Self {
        Error::OutFileError(e)
    }
}

impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Self {
        Error::ReqwestError(e)
    }
}

impl From<crate::core::Error> for Error {
    fn from(e: crate::core::Error) -> Self {
        Error::S3CoreError(e)
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::StdIoError(e)
    }
}
