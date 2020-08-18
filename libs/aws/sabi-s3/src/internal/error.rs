use crate::internal::impl_async::S3ErrorResponse;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Fail, Debug)]
pub enum Error {
    #[fail(display = "region not specified")]
    RegionNotSpecified,

    #[fail(display = "reqwest::Error > {}", 0)]
    ReqwestError(reqwest::Error),

    #[fail(display = "plus_aws::Error > {}", 0)]
    PlusAwsError(plus_aws::Error),

    #[fail(display = "crate::core::Error > {}", 0)]
    S3CoreError(crate::core::Error),

    #[fail(display = "S3ErrorResponse > {}", 0)]
    S3Error(S3ErrorResponse),

    #[fail(display = "std::io::Error > {}", 0)]
    StdIoError(std::io::Error),
}

impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Self {
        Error::ReqwestError(e)
    }
}

impl From<plus_aws::Error> for Error {
    fn from(e: plus_aws::Error) -> Self {
        Error::PlusAwsError(e)
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
