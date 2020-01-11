extern crate failure;

pub type Result<T> = ::std::result::Result<T, Error>;

#[derive(Fail, Debug)]
pub enum Error {
    #[fail(display = "clap_task error")]
    ClapTaskError(clap_task::Error),
}

impl From<clap_task::Error> for Error {
    fn from(e: clap_task::Error) -> Self {
        Error::ClapTaskError(e)
    }
}
