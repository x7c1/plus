extern crate failure;

use std::fmt::Debug;
use std::string;

pub type Result<T> = ::std::result::Result<T, Error>;

#[derive(Fail, Debug)]
pub enum Error {
    #[fail(display = "clap_task::Error > {}", 0)]
    ClapTaskError(clap_task::Error),

    #[fail(display = "clap_extractor::Error > {}", 0)]
    ClapExtractorError(String),

    #[fail(display = "std::io::Error > {}", 0)]
    StdIoError(std::io::Error),

    #[fail(display = "string::FromUtf8Error > {}", 0)]
    StringFromUtf8Error(string::FromUtf8Error),

    #[fail(display = "command failed > code: {:?}, command: {}", code, command)]
    CommandFailed { code: Option<i32>, command: String },
}

impl From<clap_task::Error> for Error {
    fn from(e: clap_task::Error) -> Self {
        Error::ClapTaskError(e)
    }
}

impl<A: Debug> From<clap_extractor::Error<A>> for Error {
    fn from(e: clap_extractor::Error<A>) -> Self {
        Error::ClapExtractorError(format!("{:?}", e))
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::StdIoError(e)
    }
}

impl From<string::FromUtf8Error> for Error {
    fn from(e: string::FromUtf8Error) -> Self {
        Error::StringFromUtf8Error(e)
    }
}
