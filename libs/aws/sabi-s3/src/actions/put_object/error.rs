use crate::internal;

#[derive(Fail, Debug)]
pub enum Error {
    #[fail(display = "internal::Error > {}", 0)]
    InternalError(internal::Error),

    #[fail(display = "crate::core::Error > {}", 0)]
    S3CoreError(crate::core::Error),

    #[fail(display = "sabi_core::Error > {}", 0)]
    SabiCoreError(sabi_core::Error),
}

impl From<internal::Error> for Error {
    fn from(e: internal::Error) -> Self {
        Error::InternalError(e)
    }
}

impl From<crate::core::Error> for Error {
    fn from(e: crate::core::Error) -> Self {
        Error::S3CoreError(e)
    }
}

impl From<sabi_core::Error> for Error {
    fn from(e: sabi_core::Error) -> Self {
        Error::SabiCoreError(e)
    }
}
