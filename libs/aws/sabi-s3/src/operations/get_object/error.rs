#[derive(Fail, Debug)]
pub enum Error {
    #[fail(display = "OutfileError > {}", 0)]
    OutFileError(super::file::OutfileError),
}

impl From<super::file::OutfileError> for Error {
    fn from(e: super::file::OutfileError) -> Self {
        Error::OutFileError(e)
    }
}
