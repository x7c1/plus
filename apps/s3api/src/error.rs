extern crate failure;

pub type Result<T> = ::std::result::Result<T, Error>;

#[derive(Fail, Debug)]
pub enum Error {
    #[fail(display = "subcommand missing")]
    SubCommandMissing,

    #[fail(display = "clap error")]
    ClapError(clap::Error),
}

impl From<clap::Error> for Error {
    fn from(e: clap::Error) -> Self {
        Error::ClapError(e)
    }
}
