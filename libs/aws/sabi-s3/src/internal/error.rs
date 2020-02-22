pub type Result<T> = std::result::Result<T, Error>;

#[derive(Fail, Debug)]
pub enum Error {
    #[fail(display = "FileNotFound > {}", 0)]
    FileNotFound { path: String, description: String },

    #[fail(display = "region not specified")]
    RegionNotSpecified,

    #[fail(display = "reqwest::Error > {}", 0)]
    ReqwestError(reqwest::Error),

    #[fail(display = "sabi_core::Error > {}", 0)]
    SabiCoreError(sabi_core::Error),

    #[fail(display = "std::io::Error > {}", 0)]
    StdIoError(std::io::Error),
}

impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Self {
        Error::ReqwestError(e)
    }
}

impl From<sabi_core::Error> for Error {
    fn from(e: sabi_core::Error) -> Self {
        Error::SabiCoreError(e)
    }
}
