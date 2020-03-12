pub type Result<A> = std::result::Result<A, Error>;

#[derive(Fail, Debug)]
pub enum Error {
    #[fail(display = "std::io::Error > {}", 0)]
    StdIoError(std::io::Error),
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::StdIoError(e)
    }
}
