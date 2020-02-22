use crate::internal;

#[derive(Fail, Debug)]
pub enum Error {
    #[fail(display = "internal::Error > {}", 0)]
    InternalError(internal::Error),

    #[fail(display = "OutfileError > {}", 0)]
    OutFileError(super::file::OutfileError),

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

impl From<super::file::OutfileError> for Error {
    fn from(e: super::file::OutfileError) -> Self {
        Error::OutFileError(e)
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
