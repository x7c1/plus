#[derive(Fail, Debug)]
pub enum Error {
    #[fail(display = "reqwest::Error > {}", 0)]
    ReqwestError(reqwest::Error),
}

impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Self {
        Error::ReqwestError(e)
    }
}
